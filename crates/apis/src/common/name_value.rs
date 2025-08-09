use serde::{Deserialize, Serialize};
use serde_json::Value;

/// NameValuePair is a name/value pair.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct NameValuePair {
    /// Name of the property.
    pub name: String,
    /// Value of the property, in plaintext.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<DynamicValue>,
    /// SecretKeyRef is the reference of a value in a secret store component.
    #[serde(skip_serializing_if = "Option::is_none", rename = "secretKeyRef")]
    pub secret_key_ref: Option<SecretKeyRef>,
    /// EnvRef is the name of an environmental variable to read the value from.
    #[serde(skip_serializing_if = "Option::is_none", rename = "envRef")]
    pub env_ref: Option<String>,
}
impl NameValuePair {
    /// HasValue returns true if the NameValuePair has a non-empty value.
    pub fn has_value(&self) -> bool {
        self.value
            .as_ref()
            .map(|v| !v.raw.is_null())
            .unwrap_or(false)
    }
    /// SetValue sets the value.
    pub fn set_value(&mut self, val: &[u8]) {
        self.value = Some(DynamicValue { raw: val.into() });
    }
}

/// SecretKeyRef is a reference to a secret holding the value for the name/value item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretKeyRef {
    /// Secret name.
    pub name: String,
    /// Field in the secret.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
}

/// DynamicValue is a dynamic value struct for the component.metadata pair value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DynamicValue {
    #[serde(flatten)]
    pub raw: Value,
}
impl DynamicValue {
    /// String returns the string representation of the raw value.
    /// If the value is a string, it will be unquoted as the string is guaranteed to be a JSON serialized string.
    /// ## Example behavior:
    /// - Input: "\"hello world\"" (JSON-encoded string)
    /// - Returns: hello world (quotes removed)
    /// - Input: "42" (JSON number as string)
    /// - Returns: 42
    pub fn to_str(&self) -> String {
        match &self.raw {
            Value::String(s) => {
                // Try to parse as JSON string first, if it fails, return as-is
                match serde_json::from_str::<String>(s) {
                    Ok(unquoted) => unquoted,
                    Err(_) => s.clone(),
                }
            }
            _ => self.raw.to_string(),
        }
    }

    /// Create a new DynamicValue from raw bytes
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, serde_json::Error> {
        let raw = serde_json::from_slice(bytes)?;
        Ok(DynamicValue { raw })
    }

    /// Get the raw JSON value as bytes
    pub fn as_bytes(&self) -> Result<Vec<u8>, serde_json::Error> {
        serde_json::to_vec(&self.raw)
    }
}

impl std::fmt::Display for DynamicValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_name_value_pair_creation() {
        let pair = NameValuePair {
            name: "test_key".to_string(),
            value: Some(DynamicValue {
                raw: json!("test_value"),
            }),
            secret_key_ref: None,
            env_ref: None,
        };

        assert_eq!(pair.name, "test_key");
        assert!(pair.value.is_some());
        assert!(pair.secret_key_ref.is_none());
        assert!(pair.env_ref.is_none());
    }

    #[test]
    fn test_name_value_pair_has_value() {
        // Test with non-null value
        let pair_with_value = NameValuePair {
            name: "key".to_string(),
            value: Some(DynamicValue {
                raw: json!("value"),
            }),
            secret_key_ref: None,
            env_ref: None,
        };
        assert!(pair_with_value.has_value());

        // Test with null value
        let pair_with_null = NameValuePair {
            name: "key".to_string(),
            value: Some(DynamicValue { raw: json!(null) }),
            secret_key_ref: None,
            env_ref: None,
        };
        assert!(!pair_with_null.has_value());

        // Test with no value
        let pair_without_value = NameValuePair {
            name: "key".to_string(),
            value: None,
            secret_key_ref: None,
            env_ref: None,
        };
        assert!(!pair_without_value.has_value());
    }

    #[test]
    fn test_name_value_pair_set_value() {
        let mut pair = NameValuePair {
            name: "key".to_string(),
            value: None,
            secret_key_ref: None,
            env_ref: None,
        };

        let test_data = b"test data".to_vec();
        pair.set_value(test_data.as_slice());

        assert!(pair.value.is_some());
        let value = pair.value.unwrap();
        assert_eq!(value.raw, json!(test_data));
    }

    #[test]
    fn test_name_value_pair_serialization_with_object() {
        let pair = NameValuePair {
            name: "test_key".to_string(),
            value: Some(DynamicValue {
                raw: json!({"data": "test_value"}),
            }),
            secret_key_ref: Some(SecretKeyRef {
                name: "secret".to_string(),
                key: Some("key".to_string()),
            }),
            env_ref: Some("ENV_VAR".to_string()),
        };

        let serialized = serde_json::to_string(&pair).unwrap();
        let deserialized: NameValuePair = serde_json::from_str(&serialized).unwrap();

        assert_eq!(pair.name, deserialized.name);
        assert_eq!(pair.env_ref, deserialized.env_ref);
        assert_eq!(
            pair.value.as_ref().unwrap().raw,
            deserialized.value.as_ref().unwrap().raw
        );
    }

    #[test]
    fn test_name_value_pair_with_string_value() {
        // Test string values without serialization since flatten doesn't work with primitives
        let pair = NameValuePair {
            name: "test_key".to_string(),
            value: Some(DynamicValue {
                raw: json!("test_value"),
            }),
            secret_key_ref: None,
            env_ref: None,
        };

        assert_eq!(pair.name, "test_key");
        assert!(pair.has_value());
        assert_eq!(pair.value.as_ref().unwrap().to_str(), "test_value");
    }

    #[test]
    fn test_secret_key_ref_creation() {
        let secret_ref = SecretKeyRef {
            name: "my-secret".to_string(),
            key: Some("password".to_string()),
        };

        assert_eq!(secret_ref.name, "my-secret");
        assert_eq!(secret_ref.key, Some("password".to_string()));
    }

    #[test]
    fn test_secret_key_ref_without_key() {
        let secret_ref = SecretKeyRef {
            name: "my-secret".to_string(),
            key: None,
        };

        assert_eq!(secret_ref.name, "my-secret");
        assert!(secret_ref.key.is_none());
    }

    #[test]
    fn test_secret_key_ref_serialization() {
        let secret_ref = SecretKeyRef {
            name: "test-secret".to_string(),
            key: Some("test-key".to_string()),
        };

        let serialized = serde_json::to_string(&secret_ref).unwrap();
        let deserialized: SecretKeyRef = serde_json::from_str(&serialized).unwrap();

        assert_eq!(secret_ref.name, deserialized.name);
        assert_eq!(secret_ref.key, deserialized.key);
    }

    #[test]
    fn test_dynamic_value_to_str_with_string() {
        let value = DynamicValue {
            raw: json!("hello world"),
        };
        assert_eq!(value.to_str(), "hello world");
    }

    #[test]
    fn test_dynamic_value_to_str_with_json_encoded_string() {
        let value = DynamicValue {
            raw: json!("\"quoted string\""),
        };
        assert_eq!(value.to_str(), "quoted string");
    }

    #[test]
    fn test_dynamic_value_to_str_with_number() {
        let value = DynamicValue { raw: json!(42) };
        assert_eq!(value.to_str(), "42");
    }

    #[test]
    fn test_dynamic_value_to_str_with_boolean() {
        let value = DynamicValue { raw: json!(true) };
        assert_eq!(value.to_str(), "true");
    }

    #[test]
    fn test_dynamic_value_to_str_with_null() {
        let value = DynamicValue { raw: json!(null) };
        assert_eq!(value.to_str(), "null");
    }

    #[test]
    fn test_dynamic_value_to_str_with_object() {
        let value = DynamicValue {
            raw: json!({"key": "value"}),
        };
        assert_eq!(value.to_str(), "{\"key\":\"value\"}");
    }

    #[test]
    fn test_dynamic_value_from_bytes() {
        let json_bytes = b"{\"test\": \"value\"}".to_vec();
        let result = DynamicValue::from_bytes(&json_bytes).unwrap();

        assert_eq!(result.raw, json!({"test": "value"}));
    }

    #[test]
    fn test_dynamic_value_from_bytes_invalid_json() {
        let invalid_bytes = b"invalid json".to_vec();
        let result = DynamicValue::from_bytes(&invalid_bytes);

        assert!(result.is_err());
    }

    #[test]
    fn test_dynamic_value_as_bytes() {
        let value = DynamicValue {
            raw: json!({"key": "value"}),
        };

        let bytes = value.as_bytes().unwrap();
        let reconstructed: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

        assert_eq!(reconstructed, json!({"key": "value"}));
    }

    #[test]
    fn test_dynamic_value_display() {
        let value = DynamicValue {
            raw: json!("test string"),
        };

        assert_eq!(format!("{}", value), "test string");
    }

    #[test]
    fn test_dynamic_value_serialization() {
        let value = DynamicValue {
            raw: json!({"nested": {"key": "value"}}),
        };

        let serialized = serde_json::to_string(&value).unwrap();
        let deserialized: DynamicValue = serde_json::from_str(&serialized).unwrap();

        assert_eq!(value.raw, deserialized.raw);
    }

    #[test]
    fn test_name_value_pair_with_secret_key_ref() {
        let pair = NameValuePair {
            name: "db_password".to_string(),
            value: None,
            secret_key_ref: Some(SecretKeyRef {
                name: "db-secrets".to_string(),
                key: Some("password".to_string()),
            }),
            env_ref: None,
        };

        assert!(!pair.has_value());
        assert!(pair.secret_key_ref.is_some());

        let secret_ref = pair.secret_key_ref.unwrap();
        assert_eq!(secret_ref.name, "db-secrets");
        assert_eq!(secret_ref.key, Some("password".to_string()));
    }

    #[test]
    fn test_name_value_pair_with_env_ref() {
        let pair = NameValuePair {
            name: "api_key".to_string(),
            value: None,
            secret_key_ref: None,
            env_ref: Some("API_KEY".to_string()),
        };

        assert!(!pair.has_value());
        assert_eq!(pair.env_ref, Some("API_KEY".to_string()));
    }

    #[test]
    fn test_serde_rename_attributes() {
        let pair = NameValuePair {
            name: "test".to_string(),
            value: None,
            secret_key_ref: Some(SecretKeyRef {
                name: "secret".to_string(),
                key: None,
            }),
            env_ref: Some("ENV_VAR".to_string()),
        };

        let json = serde_json::to_value(&pair).unwrap();

        // Check that the renamed fields are present
        assert!(json.get("secretKeyRef").is_some());
        assert!(json.get("envRef").is_some());

        // Check that the original field names are not present
        assert!(json.get("secret_key_ref").is_none());
        assert!(json.get("env_ref").is_none());
    }
}
