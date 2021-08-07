use mt_dom::{diff::*, patch::*, *};

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, ()>;

#[test]
fn keyed_no_changed() {
    let old: MyNode = element(
        "div",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "1")], vec![])],
    );

    let new: MyNode = element(
        "div",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "1")], vec![])],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(diff, vec![]);
}

#[test]
fn key_1_removed_at_start() {
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

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![RemoveNode::new(
            Some(&"div"),
            PatchPath::old(TreePath::start_at(1, vec![0, 0]),),
        )
        .into()]
    );
}

#[test]
fn non_unique_keys_matched_at_old() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "2")], vec![]),
            element("div", vec![attr("key", "2")], vec![]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "2")], vec![])],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![RemoveNode::new(
            Some(&"div"),
            PatchPath::old(TreePath::start_at(2, vec![0, 1]),),
        )
        .into()]
    );
}

#[test]
fn key_2_removed_at_the_end() {
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
        vec![element("div", vec![attr("key", "1")], vec![])],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![RemoveNode::new(
            Some(&"div"),
            PatchPath::old(TreePath::start_at(2, vec![0, 1]),),
        )
        .into()]
    );
}

#[test]
fn key_2_removed_at_the_middle() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "2")], vec![]),
            element("div", vec![attr("key", "3")], vec![]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "3")], vec![]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![RemoveNode::new(
            Some(&"div"),
            PatchPath::old(TreePath::start_at(2, vec![0, 1]),),
        )
        .into()]
    );
}

#[test]
fn there_are_2_exact_same_keys_in_the_old() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![text(0)]),
            element("div", vec![attr("key", "1")], vec![text(1)]),
            element("div", vec![attr("key", "3")], vec![text(2)]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![text(1)]),
            element("div", vec![attr("key", "3")], vec![text(2)]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            ChangeText::new(
                &Text::new("0"),
                PatchPath::new(
                    TreePath::start_at(2, vec![0, 0, 0]),
                    TreePath::start_at(2, vec![0, 0, 0])
                ),
                &Text::new("1")
            )
            .into(),
            RemoveNode::new(
                Some(&"div"),
                PatchPath::old(TreePath::start_at(3, vec![0, 1]),),
            )
            .into()
        ]
    );
}

#[test]
fn there_are_2_exact_same_keys_in_the_new() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![text(0)]),
            element("div", vec![attr("key", "3")], vec![text(2)]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![text(1)]),
            element("div", vec![attr("key", "1")], vec![text(1)]),
            element("div", vec![attr("key", "3")], vec![text(2)]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            ChangeText::new(
                &Text::new("0"),
                PatchPath::new(
                    TreePath::start_at(2, vec![0, 0, 0]),
                    TreePath::start_at(2, vec![0, 0, 0])
                ),
                &Text::new("1")
            )
            .into(),
            InsertNode::new(
                Some(&"main"),
                PatchPath::new(
                    TreePath::start_at(3, vec![0, 1]),
                    TreePath::start_at(3, vec![0, 1])
                ),
                &element("div", vec![attr("key", "1")], vec![text(1)])
            )
            .into(),
        ]
    );
}

#[test]
fn there_are_2_exact_same_keys_in_both_old_and_new() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![text(0)]), //matched 1
            element("div", vec![attr("key", "3")], vec![text(1)]),
            element("div", vec![attr("key", "3")], vec![text(2)]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![text(1)]), //matched 1
            element("div", vec![attr("key", "1")], vec![text(2)]),
            element("div", vec![attr("key", "3")], vec![text(3)]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            ChangeText::new(
                &Text::new("0"),
                PatchPath::new(
                    TreePath::start_at(2, vec![0, 0, 0]),
                    TreePath::start_at(2, vec![0, 0, 0])
                ),
                &Text::new("1")
            )
            .into(),
            ChangeText::new(
                &Text::new("1"),
                PatchPath::new(
                    TreePath::start_at(4, vec![0, 1, 0]),
                    TreePath::start_at(6, vec![0, 1, 0])
                ),
                &Text::new("3")
            )
            .into(),
            InsertNode::new(
                Some(&"main"),
                PatchPath::new(
                    TreePath::start_at(3, vec![0, 1]),
                    TreePath::start_at(3, vec![0, 1])
                ),
                &element("div", vec![attr("key", "1")], vec![text(2)])
            )
            .into(),
            RemoveNode::new(
                Some(&"div"),
                PatchPath::old(TreePath::start_at(5, vec![0, 2]),),
            )
            .into(),
        ]
    );
}

#[test]
fn key_2_inserted_at_start() {
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

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![InsertNode::new(
            Some(&"main"),
            PatchPath::new(
                TreePath::start_at(1, vec![0, 0]),
                TreePath::start_at(1, vec![0, 0])
            ),
            &element("div", vec![attr("key", "2")], vec![])
        )
        .into()]
    );
}

#[test]
fn keyed_element_not_reused() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "1")], vec![])],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![element("div", vec![attr("key", "2")], vec![])],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![ReplaceNode::new(
            Some(&"div"),
            PatchPath::new(
                TreePath::start_at(1, vec![0, 0]),
                TreePath::start_at(1, vec![0, 0])
            ),
            &element("div", vec![attr("key", "2")], vec![])
        )
        .into()]
    );
}

#[test]
fn key_2_inserted_at_the_end() {
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

    let diff = diff_with_key(&old, &new, &"key");

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![AppendChildren::new(
            &"main",
            PatchPath::old(TreePath::start_at(0, vec![0]),),
            vec![(2, &element("div", vec![attr("key", "2")], vec![]))]
        )
        .into()]
    );
}

#[test]
fn test_append_at_sub_level() {
    let old: MyNode = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![element(
            "main",
            vec![],
            vec![element("div", vec![attr("key", "1")], vec![text(1)])],
        )],
    );

    let new: MyNode = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![element(
            "main",
            vec![],
            vec![
                element("div", vec![attr("key", "1")], vec![text(1)]),
                element("div", vec![attr("key", "2")], vec![text(2)]),
                element("div", vec![attr("key", "3")], vec![text(3)]),
            ],
        )],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            AppendChildren::new(
                &"main",
                PatchPath::old(TreePath::start_at(1, vec![0, 0]),),
                vec![(
                    4,
                    &element("div", vec![attr("key", "2")], vec![text(2)])
                ),],
            )
            .into(),
            AppendChildren::new(
                &"main",
                PatchPath::old(TreePath::start_at(1, vec![0, 0]),),
                vec![(
                    6,
                    &element("div", vec![attr("key", "3")], vec![text(3)])
                )],
            )
            .into()
        ]
    )
}

#[test]
fn key_2_inserted_in_the_middle() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "3")], vec![]),
        ],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![
            element("div", vec![attr("key", "1")], vec![]),
            element("div", vec![attr("key", "2")], vec![]),
            element("div", vec![attr("key", "3")], vec![]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");

    dbg!(&diff);

    assert_eq!(
        diff,
        vec![InsertNode::new(
            Some(&"main"),
            PatchPath::new(
                TreePath::start_at(2, vec![0, 1]),
                TreePath::start_at(2, vec![0, 1])
            ),
            &element("div", vec![attr("key", "2")], vec![])
        )
        .into()]
    );
}

#[test]
fn key1_removed_at_start_then_key2_has_additional_attributes() {
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
        vec![element(
            "div",
            vec![attr("key", "2"), attr("class", "some-class")],
            vec![],
        )],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);
    // we add attrubytes at NodeIdx 2, and this will become a NodeIdx 1
    assert_eq!(
        diff,
        vec![
            AddAttributes::new(
                &"div",
                PatchPath::new(
                    TreePath::start_at(2, vec![0, 1]),
                    TreePath::start_at(1, vec![0, 1])
                ),
                vec![&attr("class", "some-class").into()]
            )
            .into(),
            RemoveNode::new(
                Some(&"div"),
                PatchPath::old(TreePath::start_at(1, vec![0, 0]),),
            )
            .into(),
        ]
    );
}

#[test]
fn deep_nested_key1_removed_at_start_then_key2_has_additional_attributes() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "article",
            vec![],
            vec![
                element("div", vec![attr("key", "1")], vec![]),
                element("div", vec![attr("key", "2")], vec![]),
            ],
        )],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "article",
            vec![],
            vec![element(
                "div",
                vec![attr("key", "2"), attr("class", "some-class")],
                vec![],
            )],
        )],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            AddAttributes::new(
                &"div",
                PatchPath::new(
                    TreePath::start_at(3, vec![0, 0, 1]),
                    TreePath::start_at(2, vec![0, 0, 1])
                ),
                vec![&attr("class", "some-class").into()]
            )
            .into(),
            RemoveNode::new(
                Some(&"div"),
                PatchPath::old(TreePath::start_at(2, vec![0, 0, 0]),),
            )
            .into(),
        ]
    );
}

#[test]
fn deep_nested_more_children_key0_and_key1_removed_at_start_then_key2_has_additional_attributes(
) {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "article",
            vec![],
            vec![
                element("div", vec![attr("key", "0")], vec![]),
                element("div", vec![attr("key", "1")], vec![]),
                element("div", vec![attr("key", "2")], vec![]),
            ],
        )],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "article",
            vec![],
            vec![element(
                "div",
                vec![attr("key", "2"), attr("class", "some-class")],
                vec![],
            )],
        )],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            AddAttributes::new(
                &"div",
                PatchPath::new(
                    TreePath::start_at(4, vec![0, 0, 2]),
                    TreePath::start_at(2, vec![0, 0, 2])
                ),
                vec![&attr("class", "some-class").into()]
            )
            .into(),
            RemoveNode::new(
                Some(&"div"),
                PatchPath::old(TreePath::start_at(2, vec![0, 0, 0]),),
            )
            .into(),
            RemoveNode::new(
                Some(&"div"),
                PatchPath::old(TreePath::start_at(3, vec![0, 0, 1]),),
            )
            .into(),
        ]
    );
}

#[test]
fn deep_nested_keyed_with_non_keyed_children() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "article",
            vec![],
            vec![
                element("div", vec![attr("key", "0")], vec![]),
                element("div", vec![attr("key", "1")], vec![]),
                element(
                    "div",
                    vec![attr("key", "2")],
                    vec![
                        element("p", vec![], vec![text("paragraph1")]),
                        element(
                            "a",
                            vec![attr("href", "#link1")],
                            vec![text("Click here")],
                        ),
                    ],
                ),
            ],
        )],
    );

    let new: MyNode = element(
        "main",
        vec![attr("class", "container")],
        vec![element(
            "article",
            vec![],
            vec![element(
                "div",
                vec![attr("key", "2"), attr("class", "some-class")],
                vec![
                    element(
                        "p",
                        vec![],
                        vec![text("paragraph1, with added content")],
                    ),
                    element(
                        "a",
                        vec![attr("href", "#link1")],
                        vec![text("Click here to continue")],
                    ),
                ],
            )],
        )],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            AddAttributes::new(
                &"div",
                PatchPath::new(
                    TreePath::start_at(4, vec![0, 0, 2]),
                    TreePath::start_at(2, vec![0, 0, 2])
                ),
                vec![&attr("class", "some-class").into()]
            )
            .into(),
            ChangeText::new(
                &Text::new("paragraph1"),
                PatchPath::new(
                    TreePath::start_at(6, vec![0, 0, 2, 0, 0]),
                    TreePath::start_at(4, vec![0, 0, 2, 0, 0])
                ),
                &Text::new("paragraph1, with added content")
            )
            .into(),
            ChangeText::new(
                &Text::new("Click here"),
                PatchPath::new(
                    TreePath::start_at(8, vec![0, 0, 2, 1, 0]),
                    TreePath::start_at(6, vec![0, 0, 2, 1, 0])
                ),
                &Text::new("Click here to continue")
            )
            .into(),
            RemoveNode::new(
                Some(&"div"),
                PatchPath::old(TreePath::start_at(2, vec![0, 0, 0]),),
            )
            .into(),
            RemoveNode::new(
                Some(&"div"),
                PatchPath::old(TreePath::start_at(3, vec![0, 0, 1]),),
            )
            .into(),
        ]
    );
}

#[test]
fn text_changed_in_keyed_elements() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "test4")],
        vec![element(
            "section",
            vec![attr("class", "todo")],
            vec![
                element("article", vec![attr("key", "1")], vec![text("item1")]),
                element("article", vec![attr("key", "2")], vec![text("item2")]),
                element("article", vec![attr("key", "3")], vec![text("item3")]),
            ],
        )],
    );

    // we remove the key1, and change the text in item3
    let update1: MyNode = element(
        "main",
        vec![attr("class", "test4")],
        vec![element(
            "section",
            vec![attr("class", "todo")],
            vec![
                element("article", vec![attr("key", "2")], vec![text("item2")]),
                element(
                    "article",
                    vec![attr("key", "3")],
                    vec![text("item3 with changes")],
                ),
            ],
        )],
    );

    let patch = diff_with_key(&old, &update1, &"key");
    dbg!(&patch);

    assert_eq!(
        patch,
        vec![
            ChangeText::new(
                &Text::new("item3"),
                PatchPath::new(
                    TreePath::start_at(7, vec![0, 0, 2, 0]),
                    TreePath::start_at(5, vec![0, 0, 2, 0])
                ),
                &Text::new("item3 with changes")
            )
            .into(),
            RemoveNode::new(
                Some(&"article"),
                PatchPath::old(TreePath::start_at(2, vec![0, 0, 0]),),
            )
            .into()
        ]
    );
}

#[test]
fn text_changed_in_mixed_keyed_and_non_keyed_elements() {
    let old: MyNode = element(
        "main",
        vec![attr("class", "test4")],
        vec![
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

    let patch = diff_with_key(&old, &update1, &"key");
    dbg!(&patch);
    assert_eq!(
        patch,
        vec![
            ChangeText::new(
                &Text::new("item3"),
                PatchPath::new(
                    TreePath::start_at(7, vec![0, 0, 2, 0]),
                    TreePath::start_at(5, vec![0, 0, 2, 0])
                ),
                &Text::new("item3 with changes")
            )
            .into(),
            RemoveNode::new(
                Some(&"article"),
                PatchPath::old(TreePath::start_at(2, vec![0, 0, 0]),),
            )
            .into(),
            ChangeText::new(
                &Text::new("3 items left"),
                PatchPath::new(
                    TreePath::start_at(9, vec![0, 1, 0]),
                    TreePath::start_at(7, vec![0, 1, 0])
                ),
                &Text::new("2 items left")
            )
            .into(),
        ]
    );
}

/// mixed of keyed and non-keyed elements
#[test]
fn test12() {
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

    let patch = diff_with_key(&old, &update1, &"key");
    dbg!(&patch);
    assert_eq!(
        patch,
        vec![
            ChangeText::new(
                &Text::new("item3"),
                PatchPath::new(
                    TreePath::start_at(9, vec![0, 1, 2, 0]),
                    TreePath::start_at(7, vec![0, 1, 2, 0])
                ),
                &Text::new("item3 with changes")
            )
            .into(),
            RemoveNode::new(
                Some(&"article"),
                PatchPath::old(TreePath::start_at(4, vec![0, 1, 0]),),
            )
            .into(),
            ChangeText::new(
                &Text::new("3 items left"),
                PatchPath::new(
                    TreePath::start_at(11, vec![0, 2, 0]),
                    TreePath::start_at(9, vec![0, 2, 0])
                ),
                &Text::new("2 items left")
            )
            .into(),
        ]
    );
}

#[test]
fn remove_first() {
    let old: MyNode = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![
            element("div", vec![attr("key", "1")], vec![text(1)]),
            element("div", vec![attr("key", "2")], vec![text(2)]),
            element("div", vec![attr("key", "3")], vec![text(3)]),
        ],
    );

    let new: MyNode = element(
        "div",
        vec![attr("id", "some-id"), attr("class", "some-class")],
        vec![
            element("div", vec![attr("key", "2")], vec![text(2)]),
            element("div", vec![attr("key", "3")], vec![text(3)]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![RemoveNode::new(
            Some(&"div"),
            PatchPath::old(TreePath::start_at(1, vec![0, 0]),),
        )
        .into()]
    )
}
