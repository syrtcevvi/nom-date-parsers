#![feature(doc_auto_cfg)]

pub mod error;
pub mod i18n;
#[cfg(feature = "numeric")]
pub mod numeric;
pub mod prelude;

mod types;
