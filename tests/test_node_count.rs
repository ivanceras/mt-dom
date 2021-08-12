#![deny(warnings)]
use mt_dom::*;

pub type MyNode = Node<&'static str, &'static str, &'static str, &'static str>;

#[test]
fn node_count1() {
    let old: MyNode = element("div", vec![], vec![]);

    assert_eq!(1, old.node_count());
    assert_eq!(0, old.descendant_node_count());
}

#[test]
fn node_count3() {
    let old: MyNode = element("div", vec![], vec![text("0"), text("1")]);

    assert_eq!(3, old.node_count());
}

#[test]
fn node_count5() {
    let old: MyNode = element(
        "div",
        vec![],
        vec![
            element(
                "b",
                vec![],
                vec![
                    element("i", vec![], vec![]),
                    element("i", vec![], vec![]),
                ],
            ),
            element("b", vec![], vec![]),
        ],
    );

    assert_eq!(5, old.node_count());
    assert_eq!(4, old.descendant_node_count());
}

#[test]
fn node_count6() {
    let old: MyNode = element(
        "div",
        vec![],
        vec![
            element(
                "b",
                vec![],
                vec![
                    element("i", vec![], vec![]),
                    element("i", vec![], vec![text("hi")]),
                ],
            ),
            element("b", vec![], vec![]),
        ],
    );

    assert_eq!(6, old.node_count());
    assert_eq!(5, old.descendant_node_count());
}
