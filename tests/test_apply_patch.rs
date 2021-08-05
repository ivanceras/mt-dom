use mt_dom::{apply_patches, diff::*, patch::*, *};

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, ()>;

#[test]
fn append_children() {
    let old: MyNode = element(
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
            vec![(2, &element("div", vec![attr("key", "2")], vec![]))]
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

    assert_eq!(patches, vec![RemoveNode::new(Some(&"div"), 1).into()]);

    let mut old_clone = old.clone();
    apply_patches(&mut old_clone, &patches);
    assert_eq!(&old_clone, &new);
}

#[test]
fn test_replace_node() {
    let old: MyNode = element("div", vec![], vec![]);
    let new = element("span", vec![], vec![]);

    let patches = diff_with_key(&old, &new, &"key");
    assert_eq!(
        patches,
        vec![ReplaceNode::new(Some(&"div"), 0, 0, &new).into()],
    );

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
        vec![Patch::ChangeText(ChangeText::new(
            1,
            &Text::new("text1"),
            1,
            &Text::new("text2")
        ))]
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
        vec![InsertNode::new(
            Some(&"main"),
            1,
            1,
            &element("div", vec![attr("key", "2")], vec![])
        )
        .into()]
    );

    let mut old_clone = old.clone();
    apply_patches(&mut old_clone, &patches);
    dbg!(&old_clone);
    dbg!(&new);
    assert_eq!(&old_clone, &new);
}

#[test]
fn test_multiple_patch_non_keyed() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "test4")],
        vec![
            element("header", vec![], vec![text("Items:")]),
            element(
                "section",
                vec![attr("class", "todo")],
                vec![
                    element("article", vec![], vec![text("item1")]),
                    element("article", vec![], vec![text("item2")]),
                    element("article", vec![], vec![text("item3")]),
                ],
            ),
            element("footer", vec![], vec![text("3 items left")]),
        ],
    );

    // we remove the key1, and change the text in item3
    let update1: MyNode = element(
        "main",
        vec![attr("class", "test4")],
        vec![
            element("header", vec![], vec![text("Items:")]),
            element(
                "section",
                vec![attr("class", "todo")],
                vec![
                    element("article", vec![], vec![text("item2")]),
                    element(
                        "article",
                        vec![],
                        vec![text("item3 with changes")],
                    ),
                ],
            ),
            element("footer", vec![], vec![text("2 items left")]),
        ],
    );

    let mut patch = diff_with_key(&old, &update1, &"key");
    patch.sort_by_key(|p| p.priority());
    dbg!(&patch);

    assert_eq!(
        patch,
        vec![
            ChangeText::new(5, &Text::new("item1"), 5, &Text::new("item2"))
                .into(),
            ChangeText::new(
                7,
                &Text::new("item2"),
                7,
                &Text::new("item3 with changes")
            )
            .into(),
            ChangeText::new(
                11,
                &Text::new("3 items left"),
                9,
                &Text::new("2 items left")
            )
            .into(),
            RemoveNode::new(Some(&"article"), 8).into(),
        ]
    );

    let mut old_clone = old.clone();
    dbg!(&update1);
    apply_patches(&mut old_clone, &patch);
    assert_eq!(&old_clone, &update1);
}

#[test]
fn test_multiple_patch_keyed() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "test4")],
        vec![
            element("header", vec![], vec![text("Items:")]),
            element(
                "section",
                vec![attr("class", "todo")],
                vec![
                    element(
                        "article",
                        vec![attr("key", "1")],
                        vec![text("item1")],
                    ),
                    element(
                        "article",
                        vec![attr("key", "2")],
                        vec![text("item2")],
                    ),
                    element(
                        "article",
                        vec![attr("key", "3")],
                        vec![text("item3")],
                    ),
                ],
            ),
            element("footer", vec![], vec![text("3 items left")]),
        ],
    );

    // we remove the key1, and change the text in item3
    let update1: MyNode = element(
        "main",
        vec![attr("class", "test4")],
        vec![
            element("header", vec![], vec![text("Items:")]),
            element(
                "section",
                vec![attr("class", "todo")],
                vec![
                    element(
                        "article",
                        vec![attr("key", "2")],
                        vec![text("item2")],
                    ),
                    element(
                        "article",
                        vec![attr("key", "3")],
                        vec![text("item3 with changes")],
                    ),
                ],
            ),
            element("footer", vec![], vec![text("2 items left")]),
        ],
    );

    let mut patch = diff_with_key(&old, &update1, &"key");
    patch.sort_by_key(|p| p.priority());
    dbg!(&patch);
    assert_eq!(
        patch,
        vec![
            ChangeText::new(
                9,
                &Text::new("item3"),
                7,
                &Text::new("item3 with changes")
            )
            .into(),
            ChangeText::new(
                11,
                &Text::new("3 items left"),
                9,
                &Text::new("2 items left")
            )
            .into(),
            RemoveNode::new(Some(&"article"), 4).into(),
        ]
    );

    let mut old_clone = old.clone();
    apply_patches(&mut old_clone, &patch);
    assert_eq!(&old_clone, &update1);
}
