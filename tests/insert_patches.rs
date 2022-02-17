use mt_dom::{diff::*, patch::*, *};

pub type MyNode = Node<&'static str, &'static str, &'static str, &'static str>;
#[test]
fn insert_on_deep_level_keyed() {
    let old: MyNode = element(
        "main",
        vec![attr("key", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![text(0)]),
            element("div", vec![attr("key", "3")], vec![text(2)]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("key", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![text(0)]),
            element("div", vec![attr("key", "2")], vec![text(1)]),
            element("div", vec![attr("key", "3")], vec![text(2)]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::insert_node(
            Some(&"main"),
            TreePath::new(vec![0, 1]),
            &element("div", vec![attr("key", "2")], vec![text(1)])
        ),]
    );
}

#[test]
fn insert_on_deep_multi_level_level_keyed() {
    let old: MyNode = element(
        "main",
        vec![attr("key", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![text(0)]),
            element(
                "div",
                vec![attr("key", "2")],
                vec![
                    element("div", vec![attr("key", "a")], vec![]),
                    element("div", vec![attr("key", "c")], vec![]),
                ],
            ),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("key", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![text(0)]),
            element(
                "div",
                vec![attr("key", "2")],
                vec![
                    element("div", vec![attr("key", "a")], vec![]),
                    element("div", vec![attr("key", "b")], vec![]),
                    element("div", vec![attr("key", "c")], vec![]),
                ],
            ),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::insert_node(
            Some(&"div"),
            TreePath::new(vec![0, 1, 1]),
            &element("div", vec![attr("key", "b")], vec![])
        ),]
    );
}

#[test]
fn insert_on_deep_multi_level_keyed_non_keyed_keyed() {
    let old: MyNode = element(
        "main",
        vec![attr("key", "container")],
        vec![
            element("div", vec![], vec![text(0)]),
            element(
                "div",
                vec![attr("key", "2")],
                vec![
                    element("div", vec![attr("key", "a")], vec![]),
                    element("div", vec![attr("key", "c")], vec![]),
                ],
            ),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("key", "container")],
        vec![
            element("div", vec![], vec![text(0)]),
            element(
                "div",
                vec![attr("key", "2")],
                vec![
                    element("div", vec![attr("key", "a")], vec![]),
                    element("div", vec![attr("key", "b")], vec![]),
                    element("div", vec![attr("key", "c")], vec![]),
                ],
            ),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::insert_node(
            Some(&"div"),
            TreePath::new(vec![0, 1, 1]),
            &element("div", vec![attr("key", "b")], vec![])
        ),]
    );
}

#[test]
fn insert_on_deep_level_non_keyed_container() {
    let old: MyNode = element(
        "main",
        vec![],
        vec![
            element("div", vec![attr("key", "1")], vec![text(0)]),
            element("div", vec![attr("key", "3")], vec![text(2)]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![],
        vec![
            element("div", vec![attr("key", "1")], vec![text(0)]),
            element("div", vec![attr("key", "2")], vec![text(1)]),
            element("div", vec![attr("key", "3")], vec![text(2)]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::insert_node(
            Some(&"main"),
            TreePath::new(vec![0, 1]),
            &element("div", vec![attr("key", "2")], vec![text(1)])
        ),]
    );
}
