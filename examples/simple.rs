use mt_dom::*;
use std::fmt;

#[derive(Clone)]
enum Value<'a> {
    Simple(String),
    Callback(&'a dyn FnMut(usize) -> String),
}

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Simple(this), Value::Simple(o)) => this == o,
            _ => true,
        }
    }
}

impl<'a> fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Callback(_) => f.debug_tuple("Callback").finish(),
            Value::Simple(s) => f.debug_tuple("Simple").field(s).finish(),
        }
    }
}

fn main() {
    println!("simple..");
    let elm1: Node<&'static str, &'static str, &'static str, Value> = element(
        "div",
        vec![
            attr("class", Value::Simple("container".to_string())),
            attr("id", Value::Simple("elm1".to_string())),
            attr("click", Value::Callback(&|x: usize| x.to_string())),
        ],
        vec![],
    );
    println!("elm1: {:#?}", elm1);

    let elm2: Node<&'static str, &'static str, &'static str, Value> = element(
        "div",
        vec![
            attr("class", Value::Simple("container".to_string())),
            attr("id", Value::Simple("elm2".to_string())),
            attr("click", Value::Callback(&|x: usize| x.to_string())),
        ],
        vec![],
    );

    let diff = diff_with_key(&elm1, &elm2, &"key");
    println!("patches: {:#?}", diff);
}
