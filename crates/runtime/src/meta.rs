use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta as Metav1Object;
use kube::Resource as ClientObject;
use rapr_k8s_apis::NameValuePair;

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
