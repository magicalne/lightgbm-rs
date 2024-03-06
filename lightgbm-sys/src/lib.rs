#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::redundant_static_lifetimes)]
#![allow(clippy::missing_safety_doc)]
#![allow(clippy::upper_case_acronyms)]

// manually maintained bindings
// delete useless lines form bindings.rs that break build
// The `binding.rs` is copied from build target directory
pub mod bindings;
pub use bindings::*;
