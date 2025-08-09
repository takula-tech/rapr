#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "assets/logo.png",
    html_favicon_url = "assets/logo.png"
)]
#![allow(missing_docs)]
#![allow(dead_code)]

pub mod common;
pub mod components;

pub use kube::core::ListMeta as K8sListMetaV1;
pub use kube::core::ObjectMeta as K8sObjectMetaV1;
pub use kube::core::TypeMeta as K8sTypeMetaV1;
