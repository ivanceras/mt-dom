pub use callback::Callback;

mod callback;

/// These are the plain attributes of an element
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute<NS, ATT, VAL, EVENT, MSG> {
    /// the attribute name,
    /// optional since style attribute doesn't need to have an attribute name
    pub(crate) name: ATT,
    /// the attribute value, which could be a simple value, and event or a function call
    pub(crate) value: AttValue<VAL, EVENT, MSG>,
    /// namespace of an attribute.
    /// This is specifically used by svg attributes
    /// such as xlink-href
    pub(crate) namespace: Option<NS>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AttValue<VAL, EVENT, MSG> {
    Plain(VAL),
    Callback(Callback<EVENT, MSG>),
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
            value: AttValue::from(value),
            namespace,
        }
    }

    /// return the name of this attribute
    pub fn name(&self) -> &ATT {
        &self.name
    }

    /// return the value of this attribute
    pub fn value(&self) -> &AttValue<VAL, EVENT, MSG> {
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
            value: self.value.map_callback(cb),
            namespace: self.namespace,
        }
    }
}

impl<VAL, EVENT, MSG> AttValue<VAL, EVENT, MSG>
where
    EVENT: 'static,
    MSG: 'static,
{
    pub fn map_callback<MSG2>(self, cb: Callback<MSG, MSG2>) -> AttValue<VAL, EVENT, MSG2>
    where
        MSG2: 'static,
    {
        match self {
            AttValue::Plain(plain) => AttValue::Plain(plain),
            AttValue::Callback(att_cb) => AttValue::Callback(att_cb.map_callback(cb)),
        }
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
