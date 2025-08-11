use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as Metav1Object;
use kube::Resource as ClientObject;
use rapr_apis::{common::NameValuePair, components::v1alpha1::Component};
use rapr_common::RaprMode;
use serde::{Deserialize, Serialize};
use snafu::{ResultExt, Snafu};
use std::collections::HashMap;
use uuid::Uuid;

/// MetaBase is the common metadata across components.
/// All components-specific metadata should embed this.
#[derive(Debug, Clone, PartialEq)]
pub struct MetaBase {
    /// Name is the name of the component that's being used.
    pub name: String,
    /// Properties is the metadata properties.
    pub properties: HashMap<String, String>,
}

impl MetaBase {
    /// Creates a new Base instance
    pub fn new(name: String, properties: HashMap<String, String>) -> Self {
        Self { name, properties }
    }

    /// Creates a new Base instance with empty properties
    pub fn with_name(name: String) -> Self {
        Self {
            name,
            properties: HashMap::new(),
        }
    }

    /// Returns the value of a property, looking it up case-insensitively
    pub fn get_property(&self, names: &[&str]) -> Option<String> {
        // Note that we must look for "names" inside the map, and not vice-versa: this way we can guarantee the order
        // Start by lowercasing all metadata keys
        let mut mdkeys: HashMap<String, String> = HashMap::with_capacity(self.properties.len());
        for key in self.properties.keys() {
            mdkeys.insert(key.to_lowercase(), key.clone());
        }

        for name in names {
            let lowercase_name = name.to_lowercase();
            if let Some(map_key) = mdkeys.get(&lowercase_name) {
                if let Some(value) = self.properties.get(map_key) {
                    return Some(value.clone());
                }
            }
        }

        None
    }

    /// Convenience method for getting a single property
    pub fn get_single_property(&self, name: &str) -> Option<String> {
        self.get_property(&[name])
    }
}

/// Resource trait that applies to both Component and HTTPEndpoint resources.
pub trait Resource {
    /// Returns the kind of the resource
    fn kind(&self) -> String;
    /// Returns the API version of the resource
    fn api_version(&self) -> String;
    /// Returns the name of the resource
    fn get_name(&self) -> String;
    /// Returns the namespace of the resource
    fn get_namespace(&self) -> String;
    /// Returns the log name of the resource
    fn log_name(&self) -> String;
    /// Returns the secret store name
    fn get_secret_store(&self) -> String;
    /// Returns the scopes for the resource
    fn get_scopes(&self) -> Vec<String>;
    /// Returns the name-value pairs for the resource
    fn name_value_pairs(&self) -> Vec<NameValuePair>;
    /// Returns the client object representation
    fn client_object(&self) -> impl ClientObject;
    /// Returns a deep copy of the resource, with the object meta set only with
    /// Name and Namespace.
    fn empty_meta_deep_copy(&self) -> Metav1Object;
}

type Result<T> = std::result::Result<T, MetaError>;

#[derive(Debug, Snafu)]
pub enum MetaError {
    #[snafu(display("Failed to generate UUID: {}", source))]
    UuidGeneration { source: uuid::Error },

    #[snafu(display(
        "Failed to parse metadata: property {} refers to {{podName}} but podName is not set",
        property_name
    ))]
    PodNameNotSet { property_name: String },
}

const WASM_STRICT_SANDBOX_METADATA_KEY: &str = "strictSandbox";

pub fn contains_namespace(items: &[NameValuePair]) -> bool {
    for item in items {
        if let Some(value) = &item.value
            && value.to_str().contains("{namespace}")
        {
            return true;
        }
    }
    false
}

/// Pure function approach - analyze component type string
pub fn is_wasm_component_type(component_type: &str) -> bool {
    // Check if component type follows WASM naming convention
    // or has specific WASM indicators
    component_type.contains("wasm") || component_type.ends_with(".wasm")
}

#[derive(Debug, Clone)]
pub struct Options {
    pub id: String,
    pub pod_name: String,
    pub namespace: String,
    pub strict_sandbox: bool,
    pub mode: RaprMode,
}

#[derive(Debug, Clone)]
pub struct Meta {
    pub id: String,
    pub pod_name: String,
    pub namespace: String,
    pub strict_sandbox: bool,
    pub mode: RaprMode,
}

impl Meta {
    pub fn new(options: Options) -> Self {
        Self {
            pod_name: options.pod_name,
            namespace: options.namespace,
            strict_sandbox: options.strict_sandbox,
            id: options.id,
            mode: options.mode,
        }
    }

    pub fn to_base_metadata(&self, mut comp: Component) -> Result<MetaBase> {
        // Add global wasm strict sandbox config to the wasm component metadata
        let cmpt_type = comp
            .spec
            .as_ref()
            .and_then(|s| Some(s.cmpt_type.as_str()))
            .unwrap_or_default();

        if is_wasm_component_type(cmpt_type) {
            self.add_wasm_strict_sandbox(&mut comp);
        }

        let items = comp
            .spec
            .as_ref()
            .and_then(|s| Some(s.metadata.as_slice()))
            .unwrap_or_default();

        let props = self.convert_items_to_props(items)?;

        Ok(MetaBase {
            properties: props,
            name: comp.get_name().to_string(),
        })
    }

    fn convert_items_to_props(&self, items: &[NameValuePair]) -> Result<HashMap<String, String>> {
        let mut properties = HashMap::new();

        for item in items {
            let mut val = item
                .value
                .as_ref()
                .and_then(|v| Some(v.to_str()))
                .unwrap_or_default();

            // Replace {uuid} placeholders
            while val.contains("{uuid}") {
                val = val.replace("{uuid}", &Uuid::new_v4().to_string());
            }

            // Replace {podName} placeholder
            if val.contains("{podName}") {
                if self.pod_name.is_empty() {
                    return Err(MetaError::PodNameNotSet {
                        property_name: item.name.clone(),
                    });
                }
                val = val.replace("{podName}", &self.pod_name);
            }

            // Replace {namespace} and {appID} placeholders
            val = val.replace("{namespace}", &format!("{}.{}", self.namespace, self.id));
            val = val.replace("{appID}", &self.id);

            properties.insert(item.name.clone(), val);
        }

        Ok(properties)
    }

    pub fn auth_secret_store_or_default<R: Resource>(&self, resource: &R) -> String {
        let secret_store = resource.get_secret_store();
        if secret_store.is_empty() {
            match self.mode {
                RaprMode::Kubernetes => "kubernetes".to_string(),
                _ => secret_store,
            }
        } else {
            secret_store
        }
    }

    /// AddWasmStrictSandbox adds global wasm strict sandbox configuration to component metadata.
    /// When strict sandbox is enabled, WASM components always run in strict mode regardless of their configuration.
    /// When strict sandbox is disabled or unset, keep the original component configuration.
    pub fn add_wasm_strict_sandbox(&self, comp: &mut Component) {
        // If the global strict sandbox is disabled (or unset), it is not enforced.
        if !self.strict_sandbox {
            return;
        }

        // If the metadata already contains the strict sandbox key, update the value to global strict sandbox config.
        if let Some(spec) = comp.spec.as_mut() {
            for name_pair in &mut spec.metadata {
                if name_pair.name.as_str().to_lowercase()
                    == WASM_STRICT_SANDBOX_METADATA_KEY.to_lowercase()
                {
                    name_pair.set_value(b"true");
                    return;
                }
            }

            // If the metadata does not contain the strict sandbox key, add it.
            let mut sandbox = NameValuePair {
                name: WASM_STRICT_SANDBOX_METADATA_KEY.to_string(),
                ..Default::default()
            };
            sandbox.set_value(b"true".as_slice());

            spec.metadata.push(sandbox);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_property_case_insensitive() {
        let mut properties = HashMap::new();
        properties.insert("TestKey".to_string(), "test_value".to_string());
        properties.insert("AnotherKey".to_string(), "another_value".to_string());

        let base = MetaBase::new("test_component".to_string(), properties);

        // Test case insensitive lookup
        assert_eq!(
            base.get_property(&["testkey"]),
            Some("test_value".to_string())
        );
        assert_eq!(
            base.get_property(&["TESTKEY"]),
            Some("test_value".to_string())
        );
        assert_eq!(
            base.get_property(&["TestKey"]),
            Some("test_value".to_string())
        );

        // Test multiple names (priority order)
        assert_eq!(
            base.get_property(&["nonexistent", "testkey"]),
            Some("test_value".to_string())
        );

        // Test non-existent key
        assert_eq!(base.get_property(&["nonexistent"]), None);
    }

    #[test]
    fn test_get_single_property() {
        let mut properties = HashMap::new();
        properties.insert("Key1".to_string(), "value1".to_string());

        let base = MetaBase::new("test".to_string(), properties);

        assert_eq!(base.get_single_property("key1"), Some("value1".to_string()));
        assert_eq!(base.get_single_property("nonexistent"), None);
    }
}
