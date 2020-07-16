pub use callback::Callback;

mod callback;

/// These are the plain attributes of an element
#[derive(Debug, Clone, PartialEq)]
pub struct Attribute<NS, ATT, VAL, EVENT, MSG> {
    /// namespace of an attribute.
    /// This is specifically used by svg attributes
    /// such as xlink-href
    pub(crate) namespace: Option<NS>,
    /// the attribute name,
    /// optional since style attribute doesn't need to have an attribute name
    pub(crate) name: ATT,
    /// the attribute value, which could be a simple value, and event or a function call
    pub(crate) value: AttValue<VAL, EVENT, MSG>,
}

/// Attribute Value which can be a plain attribute or a callback
#[derive(Debug, Clone)]
pub enum AttValue<VAL, EVENT, MSG> {
    /// Plain value
    Plain(VAL),
    /// An event listener attribute
    Callback(Callback<EVENT, MSG>),
}

impl<VAL, EVENT, MSG> PartialEq for AttValue<VAL, EVENT, MSG>
where
    VAL: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (AttValue::Plain(val), AttValue::Plain(other)) => *val == *other,
            _ => true,
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

    /// return if it is a callback
    pub fn get_callback(&self) -> Option<&Callback<EVENT, MSG>> {
        self.value.get_callback()
    }

    /// consume self and return callback if it is a callback
    pub fn take_callback(self) -> Option<Callback<EVENT, MSG>> {
        self.value.take_callback()
    }

    /// return the plain value if it is a plain value
    pub fn get_plain(&self) -> Option<&VAL> {
        self.value.get_plain()
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

    /// return if it is a callback
    pub fn get_callback(&self) -> Option<&Callback<EVENT, MSG>> {
        match self {
            AttValue::Plain(_) => None,
            AttValue::Callback(cb) => Some(cb),
        }
    }

    /// consume self and return callback if it is a callback
    pub fn take_callback(self) -> Option<Callback<EVENT, MSG>> {
        match self {
            AttValue::Plain(_) => None,
            AttValue::Callback(cb) => Some(cb),
        }
    }

    /// return a reference to the plain value if it is a plain value
    pub fn get_plain(&self) -> Option<&VAL> {
        match self {
            AttValue::Plain(plain) => Some(plain),
            AttValue::Callback(_) => None,
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
        value: AttValue::Callback(cb),
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
