use mt_dom::*;
use std::fmt;

enum Value<'a> {
    String(String),
    Function(&'a dyn FnMut(usize) -> String),
}

impl<'a> PartialEq for Value<'a> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::String(this), Value::String(o)) => this == o,
            _ => true,
        }
    }
}

impl<'a> fmt::Debug for Value<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Function(_) => write!(f, "||{{}}"),
            Value::String(s) => write!(f, "Value::String({})", s),
        }
    }
}

fn main() {
    println!("simple..");
    let elm1: Node<&'static str, &'static str, &'static str, Value> = element(
        "div",
        vec![
            attr("class", Value::String("container".to_string())),
            attr("id", Value::String("elm1".to_string())),
            attr("click", Value::Function(&|x: usize| x.to_string())),
        ],
        vec![],
    );

    let elm2: Node<&'static str, &'static str, &'static str, Value> = element(
        "div",
        vec![
            attr("class", Value::String("container".to_string())),
            attr("id", Value::String("elm2".to_string())),
            attr("click", Value::Function(&|x: usize| x.to_string())),
        ],
        vec![],
    );

    let diff = diff_with_key(&elm1, &elm2, &"key");
    println!("patches: {:#?}", diff);
}
