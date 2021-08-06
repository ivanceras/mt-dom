#![deny(
    //warnings,
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    trivial_numeric_casts,
    unstable_features,
    unused_import_braces
)]

//! mt-dom is a generic virtual dom implementation which doesn't specify the types of the data that
//! is being processed. It's up to the library user to specify those types
//!
//! The goal of this library is to provide virtual dom diffing functionality and return a portable
//! patches which the user can then use to apply those patches in their respective UI elements.
//!
//! mt-dom is not limited to be used in html base virtual-dom implementation, but can also be use
//! for native UI elements.
//!
pub use apply_patches::apply_patches;
pub use diff::diff_with_key;
pub use node::{
    attribute::{
        attr, attr_ns, group_attributes_per_name,
        merge_attributes_of_same_name, on, AttValue,
    },
    element, element_ns, text, Attribute, Element, Node, Text,
};
pub use patch::{NodeIdx, Patch};

pub mod apply_patches;
pub mod diff;
mod node;
pub mod patch;
mod zipper;
