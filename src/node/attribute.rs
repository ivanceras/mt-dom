use std::fmt;
use std::fmt::Debug;

/// These are the plain attributes of an element
#[derive(Clone, Debug, PartialEq)]
pub struct Attribute<NS, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// namespace of an attribute.
    /// This is specifically used by svg attributes
    /// such as xlink-href
    pub namespace: Option<NS>,
    /// the attribute name,
    /// optional since style attribute doesn't need to have an attribute name
    pub name: ATT,
    /// the attribute value, which could be a simple value, and event or a function call
    pub value: Vec<AttValue<VAL, EVENT>>,
}

/// Attribute Value which can be a plain attribute or a callback
#[derive(Clone, Debug, PartialEq)]
pub enum AttValue<VAL, EVENT>
where
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// Plain value
    Plain(VAL),
    /// An event listener attribute
    Event(EVENT),
}

impl<VAL, EVENT> From<VAL> for AttValue<VAL, EVENT>
where
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    fn from(value: VAL) -> Self {
        AttValue::Plain(value)
    }
}

impl<NS, ATT, VAL, EVENT> Attribute<NS, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// create a plain attribute with namespace
    pub fn new(namespace: Option<NS>, name: ATT, value: VAL) -> Self {
        Attribute {
            name,
            value: vec![AttValue::from(value)],
            namespace,
        }
    }

    /// create from multiple values
    pub fn with_multiple_values(
        namespace: Option<NS>,
        name: ATT,
        value: Vec<VAL>,
    ) -> Self {
        Attribute {
            name,
            value: value.into_iter().map(AttValue::from).collect(),
            namespace,
        }
    }

    /// return the name of this attribute
    pub fn name(&self) -> &ATT {
        &self.name
    }

    /// return the value of this attribute
    pub fn value(&self) -> &[AttValue<VAL, EVENT>] {
        &self.value
    }

    /// return the namespace of this attribute
    pub fn namespace(&self) -> Option<&NS> {
        self.namespace.as_ref()
    }

    /// return the plain value if it is a plain value
    pub fn get_plain(&self) -> Vec<&VAL> {
        self.value.iter().filter_map(|v| v.get_plain()).collect()
    }
}

impl<VAL, EVENT> AttValue<VAL, EVENT>
where
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// return a reference to the plain value if it is a plain value
    pub fn get_plain(&self) -> Option<&VAL> {
        match self {
            AttValue::Plain(plain) => Some(plain),
            AttValue::Event(_) => None,
        }
    }
}

/// create an attribute from callback
pub fn on<NS, ATT, VAL, EVENT>(
    name: ATT,
    cb: EVENT,
) -> Attribute<NS, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    Attribute {
        namespace: None,
        name,
        value: vec![AttValue::Event(cb)],
    }
}

/// Create an attribute
#[inline]
pub fn attr<NS, ATT, VAL, EVENT>(
    name: ATT,
    value: VAL,
) -> Attribute<NS, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    attr_ns(None, name, value)
}

/// Create an attribute with namespace
#[inline]
pub fn attr_ns<NS, ATT, VAL, EVENT>(
    namespace: Option<NS>,
    name: ATT,
    value: VAL,
) -> Attribute<NS, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    Attribute::new(namespace, name, value)
}

/// merge the values of attributes with the same name
pub fn merge_attributes_of_same_name<NS, ATT, VAL, EVENT>(
    attributes: &[&Attribute<NS, ATT, VAL, EVENT>],
) -> Vec<Attribute<NS, ATT, VAL, EVENT>>
where
    NS: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    let mut merged: Vec<Attribute<NS, ATT, VAL, EVENT>> = vec![];
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
pub fn group_attributes_per_name<'a, NS, ATT, VAL, EVENT>(
    attributes: &'a [Attribute<NS, ATT, VAL, EVENT>],
) -> Vec<(&'a ATT, Vec<&'a Attribute<NS, ATT, VAL, EVENT>>)>
where
    NS: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    let mut grouped: Vec<(&ATT, Vec<&Attribute<NS, ATT, VAL, EVENT>>)> = vec![];
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
