use mt_dom::{diff::*, patch::*, *};

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, &'static str>;

#[test]
fn swap_rows_non_keyed() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("class", "1")], vec![leaf("line1")]),
            element("div", vec![attr("class", "2")], vec![leaf("line2")]),
            element("div", vec![attr("class", "3")], vec![leaf("line3")]),
            element("div", vec![attr("class", "4")], vec![leaf("line4")]),
            element("div", vec![attr("class", "5")], vec![leaf("line5")]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("class", "1")], vec![leaf("line1")]),
            element("div", vec![attr("class", "4")], vec![leaf("line4")]),
            element("div", vec![attr("class", "3")], vec![leaf("line3")]),
            element("div", vec![attr("class", "2")], vec![leaf("line2")]),
            element("div", vec![attr("class", "5")], vec![leaf("line5")]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::add_attributes(
                &"div",
                TreePath::new([1]),
                vec![&attr("class", "4")],
            ),
            Patch::replace_node(
                None,
                TreePath::new([1, 0]),
                vec![&leaf("line4")]
            ),
            Patch::add_attributes(
                &"div",
                TreePath::new([3],),
                [&attr("class", "2")],
            ),
            Patch::replace_node(None, TreePath::new([3, 0],), [&leaf("line2")],)
        ]
    );
}

#[test]
fn swap_rows_keyed() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "7")], vec![leaf("line7")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "4")], vec![leaf("line4")]),
            element("div", vec![attr("key", "5")], vec![leaf("line5")]),
            element("div", vec![attr("key", "6")], vec![leaf("line6")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "8")], vec![leaf("line8")]),
            element("div", vec![attr("key", "9")], vec![leaf("line9")]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::move_after_node(
                Some(&"div",),
                TreePath::new([1]),
                TreePath::new([6])
            ),
            Patch::move_before_node(
                Some(&"div"),
                TreePath::new([6]),
                TreePath::new([2])
            ),
        ]
    );
}
