use mt_dom::{diff::*, patch::*, *};

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, &'static str>;
#[test]
fn class_changed() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "class1")],
        vec![leaf("Content of class")],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "class2")],
        vec![leaf("Content of class")],
    );

    let diff = diff_with_key(&old, &new, &"key");

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::add_attributes(
            &"main",
            TreePath::new(vec![]),
            vec![&attr("class", "class2")]
        )]
    );
}
