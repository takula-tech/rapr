#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "assets/logo.png",
    html_favicon_url = "assets/logo.png"
)]
#![allow(missing_docs)]
#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// NameValuePair is a name/value pair.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub fn set_value(&mut self, val: Vec<u8>) {
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
