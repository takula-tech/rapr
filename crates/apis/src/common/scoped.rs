use serde::{Deserialize, Serialize};

/// Scoped is a base struct for components and other resources that can be scoped to apps.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Scoped {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub scopes: Vec<String>,
}

impl Scoped {
    /// Creates a new Scoped instance with empty scopes
    pub fn new() -> Self {
        Self { scopes: Vec::new() }
    }

    /// Creates a new Scoped instance with the given scopes
    pub fn with_scopes(scopes: Vec<String>) -> Self {
        Self { scopes }
    }

    /// Returns true if the app_id is allowed in the scopes for the resource.
    pub fn is_app_scoped(&self, app_id: &str) -> bool {
        if self.scopes.is_empty() {
            // If there are no scopes, then every app is allowed
            return true;
        }
        self.scopes.iter().any(|scope| scope == app_id)
    }
}

impl Default for Scoped {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_app_scoped_empty_scopes() {
        let scoped = Scoped::new();
        assert!(scoped.is_app_scoped("any-app"));
    }

    #[test]
    fn test_is_app_scoped_with_matching_scope() {
        let scoped = Scoped::with_scopes(vec!["app1".to_string(), "app2".to_string()]);
        assert!(scoped.is_app_scoped("app1"));
        assert!(scoped.is_app_scoped("app2"));
    }

    #[test]
    fn test_is_app_scoped_with_non_matching_scope() {
        let scoped = Scoped::with_scopes(vec!["app1".to_string(), "app2".to_string()]);
        assert!(!scoped.is_app_scoped("app3"));
    }

    #[test]
    fn test_serde_serialization() {
        let scoped = Scoped::with_scopes(vec!["app1".to_string()]);
        let json = serde_json::to_string(&scoped).unwrap();
        assert_eq!(json, r#"{"scopes":["app1"]}"#);
    }

    #[test]
    fn test_serde_serialization_empty_scopes() {
        let scoped = Scoped::new();
        let json = serde_json::to_string(&scoped).unwrap();
        assert_eq!(json, "{}"); // Empty scopes are skipped in serialization
    }
}
