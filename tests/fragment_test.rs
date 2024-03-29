use mt_dom::{diff::*, patch::*, *};

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, &'static str>;

#[test]
fn using_fragments() {
    let old: MyNode = fragment(vec![
        element("div", vec![attr("key", "1")], vec![leaf("line1")]),
        element("div", vec![attr("key", "2")], vec![leaf("line2")]),
        element("div", vec![attr("key", "3")], vec![leaf("line3")]),
        element("div", vec![attr("key", "4")], vec![leaf("line4")]),
        element("div", vec![attr("key", "5")], vec![leaf("line5")]),
        element("div", vec![attr("key", "6")], vec![leaf("line6")]),
        element("div", vec![attr("key", "7")], vec![leaf("line7")]),
        element("div", vec![attr("key", "8")], vec![leaf("line8")]),
        element("div", vec![attr("key", "9")], vec![leaf("line9")]),
    ]);

    let new: MyNode = fragment(vec![
        element("div", vec![attr("key", "XXX")], vec![leaf("lineXXX")]),
        element("div", vec![attr("key", "1")], vec![leaf("line1")]),
        element("div", vec![attr("key", "2")], vec![leaf("line2")]),
        element("div", vec![attr("key", "3")], vec![leaf("line3")]),
        element("div", vec![attr("key", "4")], vec![leaf("line4")]),
        element("div", vec![attr("key", "5")], vec![leaf("line5")]),
        element("div", vec![attr("key", "6")], vec![leaf("line6")]),
        element("div", vec![attr("key", "7")], vec![leaf("line7")]),
        element("div", vec![attr("key", "8")], vec![leaf("line8")]),
        element("div", vec![attr("key", "9")], vec![leaf("line9")]),
    ]);

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![Patch::insert_before_node(
            Some(&"div"),
            TreePath::new(vec![0]),
            vec![&element(
                "div",
                vec![attr("key", "XXX")],
                vec![leaf("lineXXX")]
            )]
        )]
    );
}
