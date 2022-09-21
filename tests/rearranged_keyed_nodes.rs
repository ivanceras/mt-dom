use mt_dom::{
    diff::*,
    patch::*,
    *,
};

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, &'static str>;

#[test]
fn text_changed_non_keyed() {
    pretty_env_logger::try_init().ok();
    let old: MyNode = element(
        "main",
        vec![attr("class", "container"), attr("key", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container"), attr("key", "container")],
        vec![
            element("div", vec![attr("key", "3")], vec![leaf("line3")]),
            element("div", vec![attr("key", "2")], vec![leaf("line2")]),
            element("div", vec![attr("key", "1")], vec![leaf("line1")]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::remove_node(Some(&"div"), TreePath::new(vec![0])),
            Patch::remove_node(Some(&"div"), TreePath::new(vec![1])),
            Patch::insert_after_node(
                Some(&"div"),
                TreePath::new(vec![2]),
                vec![
                    &element(
                        "div",
                        vec![attr("key", "2")],
                        vec![leaf("line2")]
                    ),
                    &element(
                        "div",
                        vec![attr("key", "1")],
                        vec![leaf("line1")]
                    )
                ]
            ),
        ]
    );
}
