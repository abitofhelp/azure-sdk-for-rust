#![allow(clippy::module_inception)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::ptr_arg)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::derive_partial_eq_without_eq)]
#![allow(clippy::new_without_default)]
#![allow(rustdoc::bare_urls)]
#![allow(rustdoc::invalid_html_tags)]
#![allow(rustdoc::broken_intra_doc_links)]
#[cfg(feature = "package-2018-11-01-preview")]
pub mod package_2018_11_01_preview;
#[cfg(feature = "package-2019-11-01")]
pub mod package_2019_11_01;
#[cfg(feature = "package-2020-09-01")]
pub mod package_2020_09_01;
#[cfg(feature = "package-2020-10-01-preview")]
pub mod package_2020_10_01_preview;
#[cfg(feature = "package-2021-08-01")]
pub mod package_2021_08_01;
#[cfg(all(feature = "default_tag", feature = "package-2020-09-01"))]
pub use package_2020_09_01::*;
