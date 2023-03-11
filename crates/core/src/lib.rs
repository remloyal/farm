#![deny(clippy::all)]
#![allow(clippy::ptr_arg)]
#![feature(trivial_bounds)]
// #![feature(unsize)]
// #![feature(trait_upcasting)]

pub mod cache;
pub mod common;
pub mod config;
pub mod context;
pub mod error;
pub mod module;
pub mod plugin;
pub mod resource;
pub mod stats;

pub use farmfe_macro_cache_item::cache_item;

/// Version of this core crate, if the core data structures changed, and the changes will affect the memory layout,
/// like adding or removing a field, this version should be bumped. So plugin loader can recognize compatibility of the dynamic library plugins and the core.
pub const VERSION: &str = "0.1.0";

// re-export common external crates
pub use dashmap;
pub use glob;
pub use hashbrown;
pub use parking_lot;
pub use rayon;
pub use relative_path;
pub use rkyv;
pub use rkyv_dyn;
pub use rkyv_typename;
pub use serde;
pub use serde_json;
pub use swc_common;
pub use swc_css_ast;
pub use swc_ecma_ast;
pub use swc_ecma_parser;
pub use swc_html_ast;