use mt_dom::*;

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, (), ()>;

#[test]
fn key_2_inserted_at_start() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![text("line1")]),
            element("div", vec![attr("key", "2")], vec![text("line2")]),
            element("div", vec![attr("key", "3")], vec![text("line3")]),
            element("div", vec![attr("key", "4")], vec![text("line4")]),
            element("div", vec![attr("key", "5")], vec![text("line5")]),
            element("div", vec![attr("key", "6")], vec![text("line6")]),
            element("div", vec![attr("key", "7")], vec![text("line7")]),
            element("div", vec![attr("key", "8")], vec![text("line8")]),
            element("div", vec![attr("key", "9")], vec![text("line9")]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "X")], vec![text("lineX")]),
            element("div", vec![attr("key", "1")], vec![text("line1")]),
            element("div", vec![attr("key", "2")], vec![text("line2")]),
            element("div", vec![attr("key", "3")], vec![text("line3")]),
            element("div", vec![attr("key", "4")], vec![text("line4")]),
            element("div", vec![attr("key", "5")], vec![text("line5")]),
            element("div", vec![attr("key", "6")], vec![text("line6")]),
            element("div", vec![attr("key", "7")], vec![text("line7")]),
            element("div", vec![attr("key", "8")], vec![text("line8")]),
            element("div", vec![attr("key", "9")], vec![text("line9")]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![Patch::InsertChildren(
            &"main",
            0,
            0,
            vec![&element("div", vec![attr("key", "X")], vec![text("lineX")])]
        )]
    );
}
