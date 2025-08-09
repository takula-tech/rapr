#![allow(missing_docs)]
#![allow(dead_code)]

pub mod utils;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RaprMode {
    Kubernetes,
    Standalone,
}
