use mt_dom::{diff::*, patch::*, *};

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, &'static str>;
// should have no changes
#[test]
fn mixed_key_and_no_key_with_no_change() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            Patch::remove_node(Some(&"div"), TreePath::new(vec![0])),
            Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![1]),
                vec![&element("div", vec![], vec![leaf("1")])]
            ),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![2])),
            Patch::insert_after_node(
                Some(&"div"),
                TreePath::new(vec![1]),
                vec![&element("div", vec![], vec![leaf("3")])]
            ),
        ]
    );
}

#[test]
fn mixed_key_and_no_key_with_2_matched() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("3")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            Patch::replace_node(None, TreePath::new(vec![1, 0]), &leaf("1")),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![0])),
            Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![1]),
                vec![&element("div", vec![], vec![leaf("1")])]
            ),
            Patch::replace_node(None, TreePath::new(vec![2, 0]), &leaf("3")),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![3])),
            Patch::insert_after_node(
                Some(&"div"),
                TreePath::new(vec![2]),
                vec![&element("div", vec![], vec![leaf("3")])]
            ),
        ]
    );
}

#[test]
fn mixed_key_and_no_key_with_misordered_2_matched() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![], vec![leaf("1")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![attr("key", "2")], vec![leaf("2")]),
            element("div", vec![], vec![leaf("3")]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::insert_node(
                Some(&"div"),
                TreePath::new(vec![0]),
                &element("div", vec![], vec![leaf("1")]),
            ),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![1])),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![3])),
            Patch::insert_after_node(
                Some(&"div"),
                TreePath::new(vec![2]),
                vec![&element("div", vec![], vec![leaf("3")])],
            ),
        ]
    );
}
