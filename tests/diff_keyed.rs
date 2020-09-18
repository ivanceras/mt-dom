use mt_dom::{
    diff::*,
    patch::*,
    *,
};

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, (), ()>;

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
    assert_eq!(diff, vec![RemoveNode::new(Some(&"div"), 1).into()]);
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
    assert_eq!(diff, vec![RemoveNode::new(Some(&"div"), 2).into()]);
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
    assert_eq!(diff, vec![RemoveNode::new(Some(&"div"), 2).into()]);
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
    assert_eq!(diff, vec![RemoveNode::new(Some(&"div"), 2).into()]);
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
            ChangeText::new(2, "0", "1").into(),
            RemoveNode::new(Some(&"div"), 3).into()
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
    assert_eq!(
        diff,
        vec![
            ChangeText::new(2, "0", "1").into(),
            InsertNode::new(
                Some(&"main"),
                3,
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
            ChangeText::new(2, "0", "1").into(),
            ChangeText::new(4, "1", "3").into(),
            InsertNode::new(
                Some(&"main"),
                3,
                &element("div", vec![attr("key", "1")], vec![text(2)])
            )
            .into(),
            RemoveNode::new(Some(&"div"), 5).into(),
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
            1,
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
    assert_eq!(
        diff,
        vec![AppendChildren::new(
            &"main",
            0,
            vec![&element("div", vec![attr("key", "2")], vec![])]
        )
        .into()]
    );
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
            2,
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
    assert_eq!(
        diff,
        vec![
            AddAttributes::new(
                &"div",
                2,
                vec![&attr("class", "some-class").into()]
            )
            .into(),
            RemoveNode::new(Some(&"div"), 1).into(),
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
    assert_eq!(
        diff,
        vec![
            AddAttributes::new(
                &"div",
                3,
                vec![&attr("class", "some-class").into()]
            )
            .into(),
            RemoveNode::new(Some(&"div"), 2).into(),
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
    assert_eq!(
        diff,
        vec![
            AddAttributes::new(
                &"div",
                4,
                vec![&attr("class", "some-class").into()]
            )
            .into(),
            RemoveNode::new(Some(&"div"), 2).into(),
            RemoveNode::new(Some(&"div"), 3).into(),
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
    assert_eq!(
        diff,
        vec![
            AddAttributes::new(
                &"div",
                4,
                vec![&attr("class", "some-class").into()]
            )
            .into(),
            ChangeText::new(6, "paragraph1", "paragraph1, with added content")
                .into(),
            ChangeText::new(8, "Click here", "Click here to continue").into(),
            RemoveNode::new(Some(&"div"), 2).into(),
            RemoveNode::new(Some(&"div"), 3).into(),
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
    assert_eq!(
        patch,
        vec![
            ChangeText::new(7, "item3", "item3 with changes").into(),
            RemoveNode::new(Some(&"article"), 2).into()
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
    assert_eq!(
        patch,
        vec![
            ChangeText::new(7, "item3", "item3 with changes").into(),
            RemoveNode::new(Some(&"article"), 2).into(),
            ChangeText::new(9, "3 items left", "2 items left").into(),
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
    assert_eq!(
        patch,
        vec![
            ChangeText::new(9, "item3", "item3 with changes").into(),
            RemoveNode::new(Some(&"article"), 4).into(),
            ChangeText::new(11, "3 items left", "2 items left").into(),
        ]
    );
}
