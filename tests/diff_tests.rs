#![deny(warnings)]
use mt_dom::*;

pub type MyNode = Node<&'static str, &'static str, &'static str, &'static str, (), ()>;

#[test]
fn test_replace_node() {
    let old: MyNode = element("div", vec![], vec![]);
    let new = element("span", vec![], vec![]);

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![Patch::Replace(&"div", 0, &new)],
        "Should replace the first node"
    );
}

#[test]
fn test_no_changed() {
    let old: MyNode = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![],
    );

    let new: MyNode = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(diff, vec![])
}

#[test]
fn test_key_changed() {
    let old: MyNode = element(
        "div",
        vec![attr("key", "node1"), attr("class", "some-class")],
        vec![],
    );

    let new: MyNode = element(
        "div",
        vec![attr("key", "node2"), attr("class", "some-class")],
        vec![],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(diff, vec![Patch::Replace(&"div", 0, &new)])
}

#[test]
fn test_order_changed() {
    let old: MyNode = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![],
    );

    let new: MyNode = element(
        "div",
        vec![attr("class", "some-class"), attr("id", "some-id")],
        vec![],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(diff, vec![])
}

#[test]
fn test_class_changed() {
    let old: MyNode = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![],
    );

    let new = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class2")],
        vec![],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![Patch::AddAttributes(
            &"div",
            0,
            vec![attr("class", "some-class2")]
        )]
    )
}

#[test]
fn test_class_will_be_merged() {
    let old: MyNode = element("div", vec![], vec![]);

    let new = element(
        "div",
        vec![attr("class", "class1"), attr("class", "class2")],
        vec![],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![Patch::AddAttributes(
            &"div",
            0,
            vec![Attribute::with_multiple_values(
                None,
                "class",
                vec!["class1", "class2"]
            )]
        )]
    )
}

#[test]
fn test_class_removed() {
    let old: MyNode = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![],
    );

    let new = element("div", vec![attr("id", "some-id")], vec![]);

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![Patch::RemoveAttributes(
            &"div",
            0,
            vec![&attr("class", "some-class")]
        )]
    )
}
