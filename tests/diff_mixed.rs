use mt_dom::{
    diff::*,
    patch::*,
    *,
};

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, (), ()>;
// should have no changes
#[test]
fn mixed_key_and_no_key_with_no_change() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![text(1)]),
            element("div", vec![attr("key", "2")], vec![text(2)]),
            element("div", vec![], vec![text(3)]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![text(1)]),
            element("div", vec![attr("key", "2")], vec![text(2)]),
            element("div", vec![], vec![text(3)]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(diff, vec![]);
}

#[test]
fn mixed_key_and_no_key_with_2_matched() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![text(1)]),
            element("div", vec![attr("key", "2")], vec![text(2)]),
            element("div", vec![attr("key", "2")], vec![text(2)]),
            element("div", vec![], vec![text(3)]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![text(1)]),
            element("div", vec![attr("key", "2")], vec![text(1)]),
            element("div", vec![attr("key", "2")], vec![text(3)]),
            element("div", vec![], vec![text(3)]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");

    assert_eq!(
        diff,
        vec![
            ChangeText::new(4, "2", "1").into(),
            ChangeText::new(6, "2", "3").into()
        ]
    );
}

#[test]
fn mixed_key_and_no_key_with_misordered_2_matched() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "2")], vec![text(2)]),
            element("div", vec![], vec![text(1)]),
            element("div", vec![attr("key", "2")], vec![text(2)]),
            element("div", vec![], vec![text(3)]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![text(1)]),
            element("div", vec![attr("key", "2")], vec![text(2)]),
            element("div", vec![attr("key", "2")], vec![text(2)]),
            element("div", vec![], vec![text(3)]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            InsertNode::new(
                Some(&"main"),
                1,
                &element("div", vec![], vec![text(1)]),
            )
            .into(),
            RemoveNode::new(Some(&"div"), 3).into(),
        ]
    );
}
