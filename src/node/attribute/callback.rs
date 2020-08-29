use std::{fmt, rc::Rc};

/// A generic sized representation of a function that can be
/// attached to a Node. The callback will essentially be owned by the element
///
/// Limitations:
/// The callback takes an Fn instead of FnMut,
/// therefore it can not mutate the environment variables
///
/// In effect callbacks attached to DOM events are limited
/// to only passing an MSG to the program and not complex statements.
///
///
pub struct Callback<'a, EVENT, MSG>(Rc<dyn Fn(EVENT) -> MSG + 'a>);

impl<'a, EVENT, F, MSG> From<F> for Callback<'a, EVENT, MSG>
where
    F: Fn(EVENT) -> MSG + 'a,
{
    fn from(func: F) -> Self {
        Callback(Rc::new(func))
    }
}

/// Note:
/// using the #[derive(Debug)] needs EVENT and MSG to also be Debug
///
/// The reason this is manually implemented is, so that EVENT and MSG
/// doesn't need to be Debug as it is part of the Callback objects and are not shown.
impl<'a, EVENT, MSG> fmt::Debug for Callback<'a, EVENT, MSG> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "||{{..}}")
    }
}

impl<'a, EVENT, MSG> Callback<'a, EVENT, MSG>
where
    EVENT: 'a,
    MSG: 'a,
{
    /// This method calls the actual callback.
    pub fn emit(&self, event: EVENT) -> MSG {
        (self.0)(event)
    }

    /// map this callback using another callback such that
    /// MSG becomes MSG2
    pub fn map_callback<MSG2>(
        self,
        cb: Callback<'a, MSG, MSG2>,
    ) -> Callback<EVENT, MSG2>
    where
        MSG2: 'a,
    {
        let func_wrap = move |input| {
            let out = self.emit(input);
            cb.emit(out)
        };
        Callback::from(func_wrap)
    }
}

/// Note:
/// using the #[derive(Clone)] needs EVENT and MSG to also be Clone
///
/// The reason this is manually implemented is, so that EVENT and MSG
/// doesn't need to be Clone as it is part of the Callback objects and cloning here
/// is just cloning the pointer of the actual callback function
impl<'a, EVENT, MSG> Clone for Callback<'a, EVENT, MSG> {
    fn clone(&self) -> Self {
        Callback(Rc::clone(&self.0))
    }
}

/// Note:
/// using the #[derive(PartialEq)] needs EVENT and MSG to also be PartialEq.
///
/// The reason this is manually implemented is, so that EVENT and MSG
/// doesn't need to be PartialEq as it is part of the Callback objects and are not compared
impl<'a, EVENT, MSG> PartialEq for Callback<'a, EVENT, MSG> {
    fn eq(&self, _rhs: &Self) -> bool {
        true
        // Comparing the callback is only applicable
        // when they are a clone to each other.
        // This defeats the purpose in logically comparing for the
        // diffing algorthmn since all callbacks are effectively called with the closure.into()
        // which are essentially not the same Callback even when they are derived from the same
        // function.
        //Rc::ptr_eq(&self.0, &rhs.0)
    }
}
