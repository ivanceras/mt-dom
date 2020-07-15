use std::{convert::Into, fmt, rc::Rc};

/// A generic sized representation of a function that can be
/// attached to a Node. The callback will essentially be owned by the element
pub struct Callback<EVENT, MSG>(Rc<dyn Fn(EVENT) -> MSG>);

impl<EVENT, F, MSG> From<F> for Callback<EVENT, MSG>
where
    F: Fn(EVENT) -> MSG,
    F: 'static,
{
    fn from(func: F) -> Self {
        Callback(Rc::new(func))
    }
}
impl<EVENT, MSG> fmt::Debug for Callback<EVENT, MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "||{{..}}")
    }
}

impl<EVENT, MSG> Callback<EVENT, MSG>
where
    EVENT: 'static,
    MSG: 'static,
{
    /// This method calls the actual callback.
    pub fn emit<T: Into<EVENT>>(&self, value: T) -> MSG {
        (self.0)(value.into())
    }

    /// map this callback using another callback such that
    /// MSG becomes MSG2
    pub fn map_msg<MSG2>(self, cb: Callback<MSG, MSG2>) -> Callback<EVENT, MSG2>
    where
        MSG2: 'static,
    {
        let func_wrap = move |input| {
            let out = self.emit(input);
            cb.emit(out)
        };
        Callback::from(func_wrap)
    }
}

impl<EVENT, MSG> Clone for Callback<EVENT, MSG> {
    fn clone(&self) -> Self {
        Callback(Rc::clone(&self.0))
    }
}

impl<EVENT, MSG> PartialEq for Callback<EVENT, MSG> {
    fn eq(&self, rhs: &Self) -> bool {
        // Comparing the callback is only applicable
        // when they are a clone to each other.
        // This defeats the purpose in logically comparing for the
        // diffing algorthmn since all callbacks are effectively called with the closure.into()
        // which are essentially not the same Callback even when they are derived from the same
        // function.
        Rc::ptr_eq(&self.0, &rhs.0)
    }
}
