#![deny(warnings)]
use mt_dom::*;

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, (), ()>;

#[test]
fn test_replace_node() {
    let old: MyNode = element("div", vec![], vec![]);
    let new = element("span", vec![], vec![]);

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(diff, vec![Patch::Replace(&"div", 0, &new)],);
}

#[test]
fn test_replace_node_in_child() {
    let old: MyNode =
        element("main", vec![], vec![element("div", vec![], vec![])]);
    let new = element("main", vec![], vec![element("span", vec![], vec![])]);

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![Patch::Replace(&"div", 1, &element("span", vec![], vec![]))],
        "Should replace the first node"
    );
}

#[test]
fn test_205() {
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
    ); //{ <div> <b> <i></i> <i></i> </b> <b></b> </div> },

    assert_eq!(5, old.node_count());
    let new = element(
        "div",
        vec![],
        vec![
            element("b", vec![], vec![element("i", vec![], vec![])]),
            element("i", vec![], vec![]),
        ],
    ); //{ <div> <b> <i></i> </b> <i></i> </div>},
    assert_eq!(
        diff_with_key(&old, &new, &"key"),
        vec![
            Patch::RemoveChildren(&"b", 1, vec![1]),
            Patch::Replace(&"b", 4, &element("i", vec![], vec![])),
        ],
    )
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
fn test_attribute_order_changed() {
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
            vec![&attr("class", "some-class2")]
        )]
    )
}

#[test]
fn text_node_changed() {
    let old: MyNode = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![text("text1")],
    );

    let new = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![text("text2")],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(diff, vec![Patch::ChangeText(1, "text2")])
}

#[test]
fn test_class_will_not_be_merged_on_different_calls() {
    let old: MyNode = element("div", vec![], vec![]);

    let new = element(
        "div",
        vec![attr("class", "class1"), attr("class", "class2")],
        vec![],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_ne!(
        diff,
        vec![Patch::AddAttributes(
            &"div",
            0,
            vec![&Attribute::with_multiple_values(
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

#[test]
fn test_multiple_calls_to_style() {
    let old: MyNode = element(
        "div",
        vec![
            attr("style", "display:flex"),
            attr("style", "width:100px;height:100px"),
        ],
        vec![],
    );

    let new = element(
        "div",
        vec![
            attr("style", "display:flex"),
            attr("style", "width:200px;height:200px"),
        ],
        vec![],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![Patch::AddAttributes(
            &"div",
            0,
            vec![
                &attr("style", "display:flex"),
                &attr("style", "width:200px;height:200px"),
            ]
        )]
    )
}
