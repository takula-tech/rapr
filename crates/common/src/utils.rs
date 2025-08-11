#![allow(unsafe_code)]

use std::collections::HashMap;
use std::env;
use std::fs;
use std::os::unix::fs::FileTypeExt;
use std::path::Path;

pub const DOT_DELIMITER: &str = ".";

const LOG_NAME_FMT: &str = "{} ({})";
const LOG_NAME_VERSION_FMT: &str = "{} ({}/{})";

/// Contains reports whether v is present in s.
/// Similar to https://doc.rust-lang.org/std/vec/struct.Vec.html#method.contains
#[inline]
pub fn contains<T: PartialEq>(s: &[T], v: &T) -> bool {
    s.contains(v)
}

/// ContainsPrefixed reports whether v is prefixed by any of the strings in prefixes.
#[inline]
pub fn contains_prefixed(prefixes: &[String], v: &str) -> bool {
    prefixes.iter().any(|prefix| v.starts_with(prefix))
}

/// SetEnvVariables set variables to environment.
/// Returns an error if any key is empty or if setting the environment variable fails.
///
/// `Safety`: see [std::env::set_var]
#[inline]
pub fn set_env_variables(variables: &HashMap<String, String>) -> Result<(), std::io::Error> {
    for (key, value) in variables {
        if key.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Environment variable key cannot be empty",
            ));
        }
        unsafe { env::set_var(key, value) };
    }
    Ok(())
}

/// GetEnvOrElse get the value from the OS environment or use the else value if variable is not present.
#[inline]
pub fn get_env_or_else(name: &str, or_else: &str) -> String {
    env::var(name).unwrap_or_else(|_| or_else.to_string())
}

/// GetIntValOrDefault returns an int value if greater than 0 OR default value.
#[inline]
pub fn get_int_val_or_default(val: i32, default_value: i32) -> i32 {
    if val > 0 { val } else { default_value }
}

/// IsSocket returns if the given file is a unix socket.
#[inline]
pub fn is_socket(metadata: &fs::Metadata) -> bool {
    metadata.file_type().is_socket()
}

/// SocketExists returns true if the file in that path is a unix socket.
#[inline]
pub fn socket_exists<P: AsRef<Path>>(socket_path: P) -> bool {
    if let Ok(metadata) = fs::metadata(socket_path) {
        is_socket(&metadata)
    } else {
        false
    }
}

pub fn populate_metadata_for_bulk_publish_entry(
    req_meta: &HashMap<String, String>,
    entry_meta: &HashMap<String, String>,
) -> HashMap<String, String> {
    let mut res_meta = entry_meta.clone();

    for (k, v) in req_meta {
        if !res_meta.contains_key(k) {
            // Populate only metadata key that is already not present in the entry level metadata map
            res_meta.insert(k.clone(), v.clone());
        }
    }

    res_meta
}

/// Filter returns a new vector containing all items in the given slice that satisfy the given test.
#[inline]
pub fn filter<T, F>(items: &[T], test: F) -> Vec<T>
where
    T: Clone,
    F: Fn(&T) -> bool,
{
    items.iter().filter(|item| test(item)).cloned().collect()
}

/// MapToSlice is the inversion of SliceToMap. Order is not guaranteed as map retrieval order is not.
#[inline]
pub fn map_to_slice<T, V>(m: &HashMap<T, V>) -> Vec<T>
where
    T: Clone,
{
    m.keys().cloned().collect()
}

/// ComponentLogName returns the name of a component that can be used in logging.
#[inline]
pub fn component_log_name(name: &str, component_type: &str, version: &str) -> String {
    if version.is_empty() {
        format!("{name} ({component_type})")
    } else {
        format!("{name} ({component_type}/{version})")
    }
}

/// GetNamespaceOrDefault returns the namespace for Dapr, or the default namespace if it is not set.
#[inline]
pub fn get_namespace_or_default(default_namespace: &str) -> String {
    get_env_or_else("NAMESPACE", default_namespace)
}

#[inline]
pub fn parse_service_addr(val: &str) -> Vec<String> {
    val.split(',').map(|v| v.trim().to_string()).collect()
}

#[derive(Debug, PartialEq, Clone)]
struct CustomType {
    v1: String,
    v2: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contains() {
        // Test finding an item
        assert!(contains(&["item-1", "item"], &"item"));
        assert!(contains(&[1, 2, 3], &1));
        assert!(contains(
            &[
                CustomType {
                    v1: "first".to_string(),
                    v2: 1
                },
                CustomType {
                    v1: "second".to_string(),
                    v2: 2
                }
            ],
            &CustomType {
                v1: "second".to_string(),
                v2: 2
            }
        ));

        // Test not finding an item
        assert!(!contains(&["item-1", "item"], &"not-in-item"));
        assert!(!contains(&[] as &[&str], &"not-in-item"));
        assert!(!contains(&[1, 2, 3], &100));
        assert!(!contains(&[] as &[i32], &100));
        assert!(!contains(
            &[
                CustomType {
                    v1: "first".to_string(),
                    v2: 1
                },
                CustomType {
                    v1: "second".to_string(),
                    v2: 2
                }
            ],
            &CustomType {
                v1: "foo".to_string(),
                v2: 100
            }
        ));
        assert!(!contains(
            &[] as &[CustomType],
            &CustomType {
                v1: "foo".to_string(),
                v2: 100
            }
        ));
    }

    #[test]
    fn test_set_env_variables() {
        // Test setting environment variables successfully
        let mut env_vars = HashMap::new();
        env_vars.insert("testKey".to_string(), "testValue".to_string());

        let result = set_env_variables(&env_vars);
        assert!(result.is_ok());
        assert_eq!(env::var("testKey").unwrap(), "testValue");

        // Test setting environment variables with empty key (should return error)
        let mut env_vars = HashMap::new();
        env_vars.insert("".to_string(), "testValue".to_string());

        let result = set_env_variables(&env_vars);
        assert!(result.is_err());
        if let Err(e) = result {
            assert_eq!(e.kind(), std::io::ErrorKind::InvalidInput);
            assert_eq!(e.to_string(), "Environment variable key cannot be empty");
        }
    }

    #[test]
    fn test_get_int_val_or_default() {
        struct TestCase {
            name: &'static str,
            val: i32,
            def: i32,
            expected: i32,
        }

        let test_cases = vec![
            TestCase {
                name: "value is not provided by user, default value is used",
                val: 0,
                def: 5,
                expected: 5,
            },
            TestCase {
                name: "val is provided by user",
                val: 91,
                def: 5,
                expected: 91,
            },
        ];

        for tc in test_cases {
            let actual = get_int_val_or_default(tc.val, tc.def);
            assert_eq!(actual, tc.expected, "Test case: {}", tc.name);
        }
    }

    #[test]
    fn test_get_env_or_else() {
        // Test returning else value when env var is not present
        let else_value = "fakeValue";
        let fake_env_var = "envVarThatDoesNotExist";
        unsafe { env::remove_var(fake_env_var) };

        assert_eq!(get_env_or_else(fake_env_var, else_value), else_value);

        // Test returning env var value when env var is present
        let else_value = "fakeValue";
        let fake_env_var = "envVarThatExists";
        let fake_env_var_value = "envVarValue";

        unsafe { env::set_var(fake_env_var, fake_env_var_value) };
        assert_eq!(
            get_env_or_else(fake_env_var, else_value),
            fake_env_var_value
        );
        unsafe { env::remove_var(fake_env_var) };
    }

    // #[test]
    // #[cfg(unix)] // Unix Domain Socket does not work on Windows
    // fn test_socket_exists() {
    //     // Test socket exists should return false if file does not exist
    //     assert!(!socket_exists("/fake/path"));

    //     // Test socket exists should return false if file exists but it's not a socket
    //     let temp_file = NamedTempFile::new().unwrap();
    //     assert!(!socket_exists(temp_file.path().to_str().unwrap()));

    //     // Test socket exists should return true if file exists and it's a socket
    //     let socket_path = "/tmp/socket1234.sock";
    //     let _listener = UnixListener::bind(socket_path).unwrap();
    //     assert!(socket_exists(socket_path));
    //     let _ = fs::remove_file(socket_path);
    // }

    #[test]
    fn test_populate_metadata_for_bulk_publish_entry() {
        let mut entry_meta = HashMap::new();
        entry_meta.insert("key1".to_string(), "val1".to_string());
        entry_meta.insert("ttl".to_string(), "22s".to_string());

        // Test req Meta does not contain any key present in entryMeta
        let mut req_meta = HashMap::new();
        req_meta.insert("rawPayload".to_string(), "true".to_string());
        req_meta.insert("key2".to_string(), "val2".to_string());

        let res_meta = populate_metadata_for_bulk_publish_entry(&req_meta, &entry_meta);
        assert_eq!(res_meta.len(), 4, "expected length to match");
        assert!(res_meta.contains_key("key1"), "expected key to be present");
        assert_eq!(res_meta["key1"], "val1", "expected val to be equal");
        assert!(res_meta.contains_key("key2"), "expected key to be present");
        assert_eq!(res_meta["key2"], "val2", "expected val to be equal");
        assert!(res_meta.contains_key("ttl"), "expected key to be present");
        assert_eq!(res_meta["ttl"], "22s", "expected val to be equal");
        assert!(
            res_meta.contains_key("rawPayload"),
            "expected key to be present"
        );
        assert_eq!(res_meta["rawPayload"], "true", "expected val to be equal");

        // Test req Meta contains key present in entryMeta
        let mut req_meta = HashMap::new();
        req_meta.insert("ttl".to_string(), "1m".to_string());
        req_meta.insert("key2".to_string(), "val2".to_string());

        let res_meta = populate_metadata_for_bulk_publish_entry(&req_meta, &entry_meta);
        assert_eq!(res_meta.len(), 3, "expected length to match");
        assert!(res_meta.contains_key("key1"), "expected key to be present");
        assert_eq!(res_meta["key1"], "val1", "expected val to be equal");
        assert!(res_meta.contains_key("key2"), "expected key to be present");
        assert_eq!(res_meta["key2"], "val2", "expected val to be equal");
        assert!(res_meta.contains_key("ttl"), "expected key to be present");
        assert_eq!(res_meta["ttl"], "22s", "expected val to be equal");
    }

    #[test]
    fn test_filter() {
        // Test filtering out empty values
        let input = vec!["", "a", "", "b", "", "c"];
        let output = filter(&input, |s| !s.is_empty());
        assert_eq!(input.len(), 6);
        assert_eq!(output.len(), 3);
        assert_eq!(output, vec!["a", "b", "c"]);

        // Test filtering out empty values and return empty collection if all values are filtered out
        let input = vec!["", "", ""];
        let output = filter(&input, |s| !s.is_empty());
        assert_eq!(input.len(), 3);
        assert!(output.is_empty());
    }

    #[test]
    fn test_contains_prefixed() {
        struct TestCase {
            name: String,
            prefixes: Vec<String>,
            v: String,
            want: bool,
        }

        let test_cases = vec![
            TestCase {
                name: "empty".to_string(),
                prefixes: vec![],
                v: "some-service-account-name".to_string(),
                want: false,
            },
            TestCase {
                name: "notFound".to_string(),
                prefixes: vec![
                    "service-account-name".to_string(),
                    "other-service-account-name".to_string(),
                ],
                v: "some-service-account-name".to_string(),
                want: false,
            },
            TestCase {
                name: "one".to_string(),
                prefixes: vec![
                    "service-account-name".to_string(),
                    "some-service-account-name".to_string(),
                ],
                v: "some-service-account-name".to_string(),
                want: true,
            },
        ];

        for tc in test_cases {
            let result = contains_prefixed(&tc.prefixes, tc.v.as_str());
            assert_eq!(
                result, tc.want,
                "Test case: {} - contains_prefixed({:?}, {})",
                tc.name, tc.prefixes, tc.v
            );
        }
    }

    #[test]
    fn test_map_to_slice() {
        // Test map string to string
        let mut m = HashMap::new();
        m.insert("a".to_string(), "b".to_string());
        m.insert("c".to_string(), "d".to_string());
        m.insert("e".to_string(), "f".to_string());

        let mut got = map_to_slice(&m);
        got.sort(); // Sort for consistent comparison
        let mut expected = vec!["a".to_string(), "c".to_string(), "e".to_string()];
        expected.sort();
        assert_eq!(got, expected);

        // Test map string to unit struct
        let mut m = HashMap::new();
        m.insert("a".to_string(), ());
        m.insert("c".to_string(), ());
        m.insert("e".to_string(), ());

        let mut got = map_to_slice(&m);
        got.sort();
        let mut expected = vec!["a".to_string(), "c".to_string(), "e".to_string()];
        expected.sort();
        assert_eq!(got, expected);

        // Test map int to unit struct
        let mut m = HashMap::new();
        m.insert(1, ());
        m.insert(2, ());
        m.insert(3, ());

        let mut got = map_to_slice(&m);
        got.sort();
        let mut expected = vec![1, 2, 3];
        expected.sort();
        assert_eq!(got, expected);
    }

    #[test]
    fn test_get_namespace_or_default() {
        // Test namespace is empty
        unsafe { env::remove_var("NAMESPACE") };
        let ns = get_namespace_or_default("default");
        assert_eq!(ns, "default");
        // Test namespace is not empty
        unsafe { env::set_var("NAMESPACE", "testNs") };
        let ns = get_namespace_or_default("default");
        assert_eq!(ns, "testNs");
        unsafe { env::remove_var("NAMESPACE") };
    }

    #[test]
    fn test_parse_service_addr() {
        struct TestCase {
            addr: &'static str,
            out: Vec<&'static str>,
        }

        let test_cases = vec![
            TestCase {
                addr: "localhost:1020",
                out: vec!["localhost:1020"],
            },
            TestCase {
                addr: "placement1:50005,placement2:50005,placement3:50005",
                out: vec!["placement1:50005", "placement2:50005", "placement3:50005"],
            },
            TestCase {
                addr: "placement1:50005, placement2:50005, placement3:50005",
                out: vec!["placement1:50005", "placement2:50005", "placement3:50005"],
            },
        ];

        for tc in test_cases {
            let result = parse_service_addr(tc.addr);
            assert_eq!(result, tc.out, "Test case: {}", tc.addr);
        }
    }
}
