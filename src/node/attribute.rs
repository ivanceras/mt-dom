#![allow(clippy::type_complexity)]
use alloc::vec;
use alloc::vec::Vec;
use core::fmt::Debug;

/// These are the plain attributes of an element
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Attribute<Ns, Att, Val>
where
    Ns: PartialEq + Clone + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
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

impl<Ns, Att, Val> Attribute<Ns, Att, Val>
where
    Ns: PartialEq + Clone + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
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
pub fn attr<Ns, Att, Val>(name: Att, value: Val) -> Attribute<Ns, Att, Val>
where
    Ns: PartialEq + Clone + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
{
    attr_ns(None, name, value)
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
pub fn attr_ns<Ns, Att, Val>(
    namespace: Option<Ns>,
    name: Att,
    value: Val,
) -> Attribute<Ns, Att, Val>
where
    Ns: PartialEq + Clone + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
{
    Attribute::new(namespace, name, value)
}

/// merge the values of attributes with the same name
#[doc(hidden)]
pub fn merge_attributes_of_same_name<Ns, Att, Val>(
    attributes: &[&Attribute<Ns, Att, Val>],
) -> Vec<Attribute<Ns, Att, Val>>
where
    Ns: PartialEq + Clone + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
{
    let mut merged: Vec<Attribute<Ns, Att, Val>> = vec![];
    for att in attributes {
        if let Some(existing) =
            merged.iter_mut().find(|m_att| m_att.name == att.name)
        {
            existing.value.extend(att.value.clone());
        } else {
            merged.push(Attribute {
                namespace: None,
                name: att.name.clone(),
                value: att.value.clone(),
            });
        }
    }
    merged
}

/// group attributes of the same name
#[doc(hidden)]
pub fn group_attributes_per_name<Ns, Att, Val>(
    attributes: &[Attribute<Ns, Att, Val>],
) -> Vec<(&Att, Vec<&Attribute<Ns, Att, Val>>)>
where
    Ns: PartialEq + Clone + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
{
    let mut grouped: Vec<(&Att, Vec<&Attribute<Ns, Att, Val>>)> = vec![];
    for attr in attributes {
        if let Some(existing) = grouped
            .iter_mut()
            .find(|(g_att, _)| **g_att == attr.name)
            .map(|(_, attr)| attr)
        {
            existing.push(attr);
        } else {
            grouped.push((&attr.name, vec![attr]))
        }
    }
    grouped
}
