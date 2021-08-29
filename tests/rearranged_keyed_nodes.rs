use mt_dom::{diff::*, patch::*, *};

pub type MyNode = Node<&'static str, &'static str, &'static str, &'static str>;

#[test]
fn text_changed_non_keyed() {
    pretty_env_logger::try_init().ok();
    let old: MyNode = element(
        "main",
        vec![attr("class", "container"), attr("key", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![text("line1")]),
            element("div", vec![attr("key", "2")], vec![text("line2")]),
            element("div", vec![attr("key", "3")], vec![text("line3")]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container"), attr("key", "container")],
        vec![
            element("div", vec![attr("key", "3")], vec![text("line3")]),
            element("div", vec![attr("key", "2")], vec![text("line2")]),
            element("div", vec![attr("key", "1")], vec![text("line1")]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);

    //FIXME:
    //
    // The patch should be: match3, Remove 1, 2 then append 2 and 1
    assert_eq!(
        diff,
        vec![
            AppendChildren::new(
                &"main",
                TreePath::new(vec![0]),
                vec![&element(
                    "div",
                    vec![attr("key", "2")],
                    vec![text("line2")]
                ),]
            )
            .into(),
            AppendChildren::new(
                &"main",
                TreePath::new(vec![0]),
                vec![&element(
                    "div",
                    vec![attr("key", "1")],
                    vec![text("line1")]
                )]
            )
            .into(),
            RemoveNode::new(Some(&"div"), TreePath::new(vec![0, 0])).into(),
            RemoveNode::new(Some(&"div"), TreePath::new(vec![0, 1])).into(),
        ]
    );
}
