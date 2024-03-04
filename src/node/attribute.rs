#![allow(clippy::type_complexity)]
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Debug;
use indexmap::IndexMap;

pub type Ns = &'static str;
pub type Tag = &'static str;
pub type Att = &'static str;
pub type Val = String;

pub static KEY: &Att = &"key";

/// These are the plain attributes of an element
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute
{
    /// namespace of an attribute.
    /// This is specifically used by svg attributes
    /// such as xlink-href
    pub namespace: Option<Ns>,
    /// the attribute name,
    /// optional since style attribute doesn't need to have an attribute name
    pub name: Att,
    /// the attribute value, which could be a simple value, and event or a function call
    pub value: Vec<Val>,
}

impl Attribute
{
    /// create a plain attribute with namespace
    pub fn new(namespace: Option<Ns>, name: Att, value: Val) -> Self {
        Attribute {
            name,
            value: vec![value],
            namespace,
        }
    }

    /// create from multiple values
    pub fn with_multiple_values(
        namespace: Option<Ns>,
        name: Att,
        value: impl IntoIterator<Item = Val>,
    ) -> Self {
        Attribute {
            name,
            value: value.into_iter().collect(),
            namespace,
        }
    }

    /// return the name of this attribute
    pub fn name(&self) -> &Att {
        &self.name
    }

    /// return the value of this attribute
    pub fn value(&self) -> &[Val] {
        &self.value
    }

    /// return the namespace of this attribute
    pub fn namespace(&self) -> Option<&Ns> {
        self.namespace.as_ref()
    }
}

/// Create an attribute
/// # Example
/// ```rust
/// use mt_dom::{Attribute,attr};
/// let class: Attribute<&'static str, &'static str, &'static str> =
///     attr("class", "container");
/// ```
#[inline]
pub fn attr(name: Att, value: impl Into<Val>) -> Attribute
{
    attr_ns(None, name, value.into())
}

/// Create an attribute with namespace
/// # Example
/// ```rust
/// use mt_dom::{Attribute,attr_ns};
///
/// let href: Attribute<&'static str, &'static str, &'static str> =
///     attr_ns(Some("http://www.w3.org/1999/xlink"), "href", "cool-script.js");
/// ```
#[inline]
pub fn attr_ns(
    namespace: Option<Ns>,
    name: Att,
    value: Val,
) -> Attribute
{
    Attribute::new(namespace, name, value)
}

/// merge the values of attributes with the same name
#[doc(hidden)]
pub fn merge_attributes_of_same_name(
    attributes: &[&Attribute],
) -> Vec<Attribute>
{
    //let mut merged: Vec<Attribute> = vec![];
    let mut merged: IndexMap<&Att, Attribute> =
        IndexMap::with_capacity(attributes.len());
    for att in attributes {
        if let Some(existing) = merged.get_mut(&att.name) {
            existing.value.extend(att.value.clone());
        } else {
            merged.insert(
                &att.name,
                Attribute {
                    namespace: None,
                    name: att.name,
                    value: att.value.clone(),
                },
            );
        }
    }
    merged.into_values().collect()
}

/// group attributes of the same name
#[doc(hidden)]
pub fn group_attributes_per_name(
    attributes: &[Attribute],
) -> IndexMap<&Att, Vec<&Attribute>>
{
    let mut grouped: IndexMap<&Att, Vec<&Attribute>> =
        IndexMap::with_capacity(attributes.len());
    for attr in attributes {
        if let Some(existing) = grouped.get_mut(&attr.name) {
            existing.push(attr);
        } else {
            grouped.insert(&attr.name, vec![attr]);
        }
    }
    grouped
}
