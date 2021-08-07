use mt_dom::{diff::*, patch::*, *};

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, ()>;
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
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            ChangeText::new(
                &Text::new("2"),
                PatchPath::new(
                    TreePath::start_at(4, vec![0, 1, 0]),
                    TreePath::start_at(4, vec![0, 1, 0])
                ),
                &Text::new("1")
            )
            .into(),
            ChangeText::new(
                &Text::new("2"),
                PatchPath::new(
                    TreePath::start_at(6, vec![0, 2, 0]),
                    TreePath::start_at(6, vec![0, 2, 0])
                ),
                &Text::new("3")
            )
            .into()
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
                PatchPath::new(
                    TreePath::start_at(1, vec![0, 0]),
                    TreePath::start_at(1, vec![0, 0])
                ),
                &element("div", vec![], vec![text(1)]),
            )
            .into(),
            RemoveNode::new(
                Some(&"div"),
                PatchPath::old(TreePath::start_at(3, vec![0, 1]),),
            )
            .into(),
        ]
    );
}
