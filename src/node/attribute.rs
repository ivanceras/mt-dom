#![allow(clippy::type_complexity)]
use std::fmt::Debug;
use indexmap::IndexMap;

/// The type of the Namspace
pub type Namespace = &'static str;
/// The type of the Tag
pub type Tag = &'static str;
/// The type of Attribute Name
pub type AttributeName = &'static str;
/// The type of the Attribute Value
pub type AttributeValue = String;

/// The key attribute
pub static KEY: &AttributeName = &"key";

/// These are the plain attributes of an element
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute {
    /// namespace of an attribute.
    /// This is specifically used by svg attributes
    /// such as xlink-href
    pub namespace: Option<Namespace>,
    /// the attribute name,
    /// optional since style attribute doesn't need to have an attribute name
    pub name: AttributeName,
    /// the attribute value, which could be a simple value, and event or a function call
    pub value: Vec<AttributeValue>,
}

impl Attribute {
    /// create a plain attribute with namespace
    pub fn new(namespace: Option<Namespace>, name: AttributeName, value: AttributeValue) -> Self {
        Attribute {
            name,
            value: vec![value],
            namespace,
        }
    }

    /// create from multiple values
    pub fn with_multiple_values(
        namespace: Option<Namespace>,
        name: AttributeName,
        value: impl IntoIterator<Item = AttributeValue>,
    ) -> Self {
        Attribute {
            name,
            value: value.into_iter().collect(),
            namespace,
        }
    }

    /// return the name of this attribute
    pub fn name(&self) -> &AttributeName {
        &self.name
    }

    /// return the value of this attribute
    pub fn value(&self) -> &[AttributeValue] {
        &self.value
    }

    /// return the namespace of this attribute
    pub fn namespace(&self) -> Option<&Namespace> {
        self.namespace.as_ref()
    }
}

/// Create an attribute
/// # Example
/// ```rust
/// use mt_dom::{Attribute,attr};
/// let class: Attribute = attr("class", "container");
/// ```
#[inline]
pub fn attr(name: AttributeName, value: impl Into<AttributeValue>) -> Attribute {
    attr_ns(None, name, value)
}

/// Create an attribute with namespace
/// # Example
/// ```rust
/// use mt_dom::{Attribute,attr_ns};
///
/// let href: Attribute = attr_ns(Some("http://www.w3.org/1999/xlink"), "href", "cool-script.js");
/// ```
#[inline]
pub fn attr_ns(
    namespace: Option<Namespace>,
    name: AttributeName,
    value: impl Into<AttributeValue>,
) -> Attribute {
    Attribute::new(namespace, name, value.into())
}

/// merge the values of attributes with the same name
#[doc(hidden)]
pub fn merge_attributes_of_same_name(
    attributes: &[&Attribute],
) -> Vec<Attribute> {
    //let mut merged: Vec<Attribute> = vec![];
    let mut merged: IndexMap<&AttributeName, Attribute> =
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
) -> IndexMap<&AttributeName, Vec<&Attribute>> {
    let mut grouped: IndexMap<&AttributeName, Vec<&Attribute>> =
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
