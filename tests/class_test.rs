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

#[test]
fn parent_of_matching_keyed_are_ignored() {
    let old: MyNode = element(
        "ul",
        [attr("class", "original")],
        [
            element("li", [attr("key", "0")], [leaf("text0")]),
            element("li", [attr("key", "1")], [leaf("text1")]),
            element("li", [attr("key", "2")], [leaf("text2")]),
        ],
    );

    let new: MyNode = element(
        "ul",
        [attr("class", "changed")],
        [
            element("li", [attr("key", "0")], [leaf("text0")]),
            element("li", [attr("key", "1")], [leaf("text1")]),
            element("li", [attr("key", "2")], [leaf("text2")]),
        ],
    );

    let patches = diff_with_key(&old, &new, &"key");

    assert_eq!(
        patches,
        vec![Patch::add_attributes(
            &"ul",
            TreePath::new(vec![]),
            vec![&attr("class", "changed")]
        )],
        "Should add the new attributes"
    );
}
