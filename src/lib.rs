#![deny(
    warnings,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]
#![forbid(unsafe_code)]
#![deny(clippy::all)]
#![allow(clippy::type_complexity)]
//! mt-dom is a generic virtual dom implementation which doesn't specify the types of the data that
//! is being processed. It's up to the library user to specify those types
//!
//! The goal of this library is to provide virtual dom diffing functionality and return a portable
//! patches which the user can then use to apply those patches in their respective UI elements.
//!
//! mt-dom is not limited to be used in html base virtual-dom implementation, but can also be use
//! for native UI elements.
//!
extern crate alloc;
pub use diff::{diff_with_key, diff_recursive};
pub use node::{
    attribute::{
        attr, attr_ns, group_attributes_per_name, merge_attributes_of_same_name,
    },
    element, element_ns, fragment, leaf, node_list, Attribute, Element, Node,
};
pub use patch::{Patch, PatchType, TreePath};

pub mod diff;
mod diff_lis;
mod node;
pub mod patch;
