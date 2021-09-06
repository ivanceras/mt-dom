use mt_dom::{patch::*, *};
pub type MyNode = Node<&'static str, &'static str, &'static str, &'static str>;

#[test]
fn comment_nodes() {
    let old: MyNode = comment("hello");
    let new: MyNode = comment("hi");

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![ChangeComment::new(
            &"hello".to_string(),
            TreePath::new(vec![0]),
            &"hi".to_string()
        )
        .into()]
    );
}

#[test]
fn two_text_siblings_will_be_comment_separated() {
    let old: MyNode =
        element("div", vec![], vec![text("hello"), text("world")]);
    let expected: MyNode = element(
        "div",
        vec![],
        vec![text("hello"), comment("separator"), text("world")],
    );

    assert_eq!(old, expected);
}
