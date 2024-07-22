#![cfg_attr(all(docsrs, feature = "nightly"), feature(doc_cfg, doc_auto_cfg))]

pub mod error;
pub mod i18n;
#[cfg(feature = "numeric")]
pub mod numeric;
pub mod prelude;
#[cfg(feature = "quick")]
pub mod quick;

mod types;
