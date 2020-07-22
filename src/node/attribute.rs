pub use callback::Callback;
use std::fmt;

mod callback;

/// These are the plain attributes of an element
pub struct Attribute<NS, ATT, VAL, EVENT, MSG> {
    /// namespace of an attribute.
    /// This is specifically used by svg attributes
    /// such as xlink-href
    pub(crate) namespace: Option<NS>,
    /// the attribute name,
    /// optional since style attribute doesn't need to have an attribute name
    pub(crate) name: ATT,
    /// the attribute value, which could be a simple value, and event or a function call
    pub(crate) value: Vec<AttValue<VAL, EVENT, MSG>>,
}

impl<NS, ATT, VAL, EVENT, MSG> Clone for Attribute<NS, ATT, VAL, EVENT, MSG>
where
    NS: Clone,
    ATT: Clone,
    VAL: Clone,
{
    fn clone(&self) -> Self {
        Attribute {
            namespace: self.namespace.clone(),
            name: self.name.clone(),
            value: self.value.clone(),
        }
    }
}

impl<NS, ATT, VAL, EVENT, MSG> PartialEq for Attribute<NS, ATT, VAL, EVENT, MSG>
where
    NS: PartialEq,
    ATT: PartialEq,
    VAL: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.namespace == other.namespace && self.name == other.name && self.value == other.value
    }
}

impl<NS, ATT, VAL, EVENT, MSG> fmt::Debug for Attribute<NS, ATT, VAL, EVENT, MSG>
where
    NS: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Attribute")
            .field("namespace", &self.namespace)
            .field("name", &self.name)
            .field("value", &self.value)
            .finish()
    }
}

/// Attribute Value which can be a plain attribute or a callback
pub enum AttValue<VAL, EVENT, MSG> {
    /// Plain value
    Plain(VAL),
    /// An event listener attribute
    Callback(Callback<EVENT, MSG>),
}

impl<VAL, EVENT, MSG> Clone for AttValue<VAL, EVENT, MSG>
where
    VAL: Clone,
{
    fn clone(&self) -> Self {
        match self {
            AttValue::Plain(value) => AttValue::Plain(value.clone()),
            AttValue::Callback(cb) => AttValue::Callback(cb.clone()),
        }
    }
}

impl<VAL, EVENT, MSG> fmt::Debug for AttValue<VAL, EVENT, MSG>
where
    VAL: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AttValue::Plain(value) => f.debug_tuple("Plain").field(value).finish(),
            AttValue::Callback(cb) => f.debug_tuple("Callback").field(cb).finish(),
        }
    }
}

impl<VAL, EVENT, MSG> PartialEq for AttValue<VAL, EVENT, MSG>
where
    VAL: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AttValue::Plain(val), AttValue::Plain(other)) => *val == *other,
            (AttValue::Callback(cb), AttValue::Callback(other)) => *cb == *other,
            _ => false,
        }
    }
}

impl<VAL, EVENT, MSG> From<VAL> for AttValue<VAL, EVENT, MSG> {
    fn from(value: VAL) -> Self {
        AttValue::Plain(value)
    }
}

impl<NS, ATT, VAL, EVENT, MSG> Attribute<NS, ATT, VAL, EVENT, MSG> {
    /// create a plain attribute with namespace
    pub fn new(namespace: Option<NS>, name: ATT, value: VAL) -> Self {
        Attribute {
            name,
            value: vec![AttValue::from(value)],
            namespace,
        }
    }

    /// create from multiple values
    pub fn with_multiple_values(namespace: Option<NS>, name: ATT, value: Vec<VAL>) -> Self {
        Attribute {
            name,
            value: value.into_iter().map(|v| AttValue::from(v)).collect(),
            namespace,
        }
    }

    /// return the name of this attribute
    pub fn name(&self) -> &ATT {
        &self.name
    }

    /// return the value of this attribute
    pub fn value(&self) -> &[AttValue<VAL, EVENT, MSG>] {
        &self.value
    }

    /// return the namespace of this attribute
    pub fn namespace(&self) -> Option<&NS> {
        self.namespace.as_ref()
    }
}

impl<NS, ATT, VAL, EVENT, MSG> Attribute<NS, ATT, VAL, EVENT, MSG>
where
    EVENT: 'static,
    MSG: 'static,
{
    /// transform the callback of this attribute
    pub fn map_callback<MSG2>(self, cb: Callback<MSG, MSG2>) -> Attribute<NS, ATT, VAL, EVENT, MSG2>
    where
        MSG2: 'static,
    {
        Attribute {
            name: self.name,
            value: self
                .value
                .into_iter()
                .map(|v| v.map_callback(cb.clone()))
                .collect(),
            namespace: self.namespace,
        }
    }

    /// return the plain value if it is a plain value
    pub fn get_plain(&self) -> Vec<&VAL> {
        self.value.iter().filter_map(|v| v.get_plain()).collect()
    }

    /// return the callback values of this attribute
    pub fn get_callback(&self) -> Vec<&Callback<EVENT, MSG>> {
        self.value.iter().filter_map(|v| v.get_callback()).collect()
    }
}

impl<VAL, EVENT, MSG> AttValue<VAL, EVENT, MSG>
where
    EVENT: 'static,
    MSG: 'static,
{
    /// transform att_value such that MSG becomes MSG2
    pub fn map_callback<MSG2>(self, cb: Callback<MSG, MSG2>) -> AttValue<VAL, EVENT, MSG2>
    where
        MSG2: 'static,
    {
        match self {
            AttValue::Plain(plain) => AttValue::Plain(plain),
            AttValue::Callback(att_cb) => AttValue::Callback(att_cb.map_callback(cb)),
        }
    }

    /// return a reference to the plain value if it is a plain value
    pub fn get_plain(&self) -> Option<&VAL> {
        match self {
            AttValue::Plain(plain) => Some(plain),
            AttValue::Callback(_) => None,
        }
    }

    /// return a reference to the callback if it is a callback
    pub fn get_callback(&self) -> Option<&Callback<EVENT, MSG>> {
        match self {
            AttValue::Plain(_) => None,
            AttValue::Callback(cb) => Some(cb),
        }
    }

    /// return true if this is a callback
    pub fn is_callback(&self) -> bool {
        match self {
            AttValue::Plain(_) => false,
            AttValue::Callback(_) => true,
        }
    }
}

/// create an attribute from callback
pub fn on<NS, ATT, VAL, EVENT, MSG>(
    name: ATT,
    cb: Callback<EVENT, MSG>,
) -> Attribute<NS, ATT, VAL, EVENT, MSG> {
    Attribute {
        namespace: None,
        name,
        value: vec![AttValue::Callback(cb)],
    }
}

/// Create an attribute
#[inline]
pub fn attr<NS, ATT, VAL, EVENT, MSG>(
    name: ATT,
    value: VAL,
) -> Attribute<NS, ATT, VAL, EVENT, MSG> {
    attr_ns(None, name, value)
}

/// Create an attribute with namespace
#[inline]
pub fn attr_ns<NS, ATT, VAL, EVENT, MSG>(
    namespace: Option<NS>,
    name: ATT,
    value: VAL,
) -> Attribute<NS, ATT, VAL, EVENT, MSG> {
    Attribute::new(namespace, name, value)
}
