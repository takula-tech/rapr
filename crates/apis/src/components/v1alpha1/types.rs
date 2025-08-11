use crate::common::{NameValuePair, Scoped};
use crate::{K8sListMetaV1, K8sObjectMetaV1, K8sTypeMetaV1};
use rapr_common::utils::component_log_name;
use serde::{Deserialize, Serialize};

pub const KIND: &str = "Component";
pub const VERSION: &str = "v1alpha1";

/// ComponentSpec is the spec for a component.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComponentSpec {
    #[serde(rename = "type")]
    pub cmpt_type: String,
    pub version: String,
    #[serde(default, rename = "ignoreErrors")]
    pub ignore_errors: bool,
    #[serde(default)]
    pub metadata: Vec<NameValuePair>,
    #[serde(
        default,
        rename = "initTimeout",
        skip_serializing_if = "Option::is_none"
    )]
    pub init_timeout: Option<String>,
}

/// Auth represents authentication details for the component.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Auth {
    #[serde(rename = "secretStore")]
    pub secret_store: String,
}

/// ComponentList is a list of Dapr components.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComponentList {
    #[serde(flatten)]
    pub type_meta: K8sTypeMetaV1,
    pub metadata: K8sListMetaV1,
    pub items: Vec<Component>,
}

/// Component describes a Dapr component type.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Component {
    #[serde(flatten)]
    pub type_meta: K8sTypeMetaV1,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<K8sObjectMetaV1>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub spec: Option<ComponentSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub auth: Option<Auth>,
    #[serde(flatten)]
    pub scoped: Scoped,
}

impl Component {
    /// Returns the component kind.
    pub fn kind(&self) -> &'static str {
        "Component"
    }

    pub fn api_version(&self) -> String {
        format!("{}/{}", crate::components::GROUP_NAME, VERSION)
    }

    /// Returns the component name.
    pub fn get_name(&self) -> &str {
        self.metadata
            .as_ref()
            .and_then(|m| m.name.as_deref())
            .unwrap_or("")
    }

    /// Returns the component namespace.
    pub fn get_namespace(&self) -> &str {
        self.metadata
            .as_ref()
            .and_then(|m| m.namespace.as_deref())
            .unwrap_or("")
    }

    /// Returns the name of the component that can be used in logging.
    pub fn log_name(&self) -> String {
        let name = self.get_name();
        let component_type = self
            .spec
            .as_ref()
            .map(|s| s.cmpt_type.as_str())
            .unwrap_or("");
        let version = self.spec.as_ref().map(|s| s.version.as_str()).unwrap_or("");

        component_log_name(name, component_type, version)
    }

    /// Returns the name of the secret store.
    pub fn get_secret_store(&self) -> &str {
        self.auth
            .as_ref()
            .map(|a| a.secret_store.as_str())
            .unwrap_or("")
    }

    /// Returns the component's metadata as name/value pairs.
    pub fn name_value_pairs(&self) -> &[NameValuePair] {
        self.spec
            .as_ref()
            .map(|s| s.metadata.as_slice())
            .unwrap_or(&[])
    }

    pub fn get_scopes(&self) -> &[String] {
        &self.scoped.scopes
    }

    /// Returns a new instance of the component type with the
    /// TypeMeta's Kind and APIVersion fields set.
    pub fn empty_meta_deep_copy(&self) -> Component {
        let mut component = self.clone();
        component.type_meta = K8sTypeMetaV1 {
            kind: KIND.to_string(),
            api_version: format!("{}/{}", crate::components::GROUP_NAME, VERSION),
        };
        component.metadata = Some(K8sObjectMetaV1 {
            name: self.metadata.as_ref().and_then(|m| m.name.clone()),
            ..Default::default()
        });
        component
    }
}
