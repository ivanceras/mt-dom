use mt_dom::{
    apply_patches,
    diff::*,
    patch::*,
    *,
};

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, (), ()>;

#[test]
fn append_children() {
    let mut old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "1")], vec![])],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "2")], vec![]),
        ],
    );

    let patches = diff_with_key(&old, &new, &"key");

    assert_eq!(
        patches,
        vec![AppendChildren::new(
            &"main",
            0,
            vec![&element("div", vec![attr("key", "2")], vec![])]
        )
        .into()]
    );
    let mut old_clone = old.clone();
    apply_patches(&mut old_clone, &patches);
    assert_eq!(&old_clone, &new);
}

#[test]
fn remove_children() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "2")], vec![]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "2")], vec![])],
    );

    let patches = diff_with_key(&old, &new, &"key");

    assert_eq!(
        patches,
        vec![RemoveChildren::new(&"main", 0, vec![0]).into()]
    );

    let mut old_clone = old.clone();
    apply_patches(&mut old_clone, &patches);
    assert_eq!(&old_clone, &new);
}

#[test]
fn test_replace_node() {
    let old: MyNode = element("div", vec![], vec![]);
    let new = element("span", vec![], vec![]);

    let patches = diff_with_key(&old, &new, &"key");
    assert_eq!(patches, vec![ReplaceNode::new(&"div", 0, &new).into()],);

    let mut old_clone = old.clone();
    apply_patches(&mut old_clone, &patches);
    assert_eq!(&old_clone, &new);
}

#[test]
fn change_text() {
    let old: MyNode = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![text("text1")],
    );

    let new = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![text("text2")],
    );

    let patches = diff_with_key(&old, &new, &"key");
    assert_eq!(
        patches,
        vec![Patch::ChangeText(ChangeText::new(1, "text1", "text2"))]
    );

    let mut old_clone = old.clone();
    apply_patches(&mut old_clone, &patches);
    assert_eq!(&old_clone, &new);
}

#[test]
fn remove_attributes() {
    let old: MyNode = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![],
    );

    let new = element("div", vec![attr("id", "some-id")], vec![]);

    let patches = diff_with_key(&old, &new, &"key");
    assert_eq!(
        patches,
        vec![RemoveAttributes::new(
            &"div",
            0,
            vec![&attr("class", "some-class")]
        )
        .into()]
    );

    let mut old_clone = old.clone();
    apply_patches(&mut old_clone, &patches);
    assert_eq!(&old_clone, &new);
}

#[test]
fn insert_children() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "1")], vec![])],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "2")], vec![]),
            element("div", vec![attr("key", "1")], vec![]),
        ],
    );

    let patches = diff_with_key(&old, &new, &"key");
    dbg!(&patches);

    assert_eq!(
        patches,
        vec![InsertChildren::new(
            &"main",
            0,
            0,
            vec![&element("div", vec![attr("key", "2")], vec![])]
        )
        .into()]
    );

    let mut old_clone = old.clone();
    apply_patches(&mut old_clone, &patches);
    assert_eq!(&old_clone, &new);
}
