#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![doc(
    html_logo_url = "assets/logo.png",
    html_favicon_url = "assets/logo.png"
)]
#![allow(missing_docs)]
#![allow(dead_code)]

pub(crate) enum RaprMode {
    Kubernetes,
    Standalone,
}

pub mod meta;
