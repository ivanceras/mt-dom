use mt_dom::*;

pub type MyNode = Node<&'static str, &'static str, &'static str, &'static str, (), ()>;

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
    assert_eq!(diff, vec![Patch::RemoveChildren(&"main", 0, vec![0])]);
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
    assert_eq!(diff, vec![Patch::RemoveChildren(&"main", 0, vec![1])]);
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
    assert_eq!(diff, vec![Patch::RemoveChildren(&"main", 0, vec![1])]);
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
    assert_eq!(
        diff,
        vec![Patch::InsertChildren(
            &"main",
            0,
            0,
            vec![&element("div", vec![attr("key", "2")], vec![])]
        )]
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
        vec![Patch::AppendChildren(
            &"main",
            0,
            vec![&element("div", vec![attr("key", "2")], vec![])]
        )]
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
    assert_eq!(
        diff,
        vec![Patch::InsertChildren(
            &"main",
            0,
            1,
            vec![&element("div", vec![attr("key", "2")], vec![])]
        )]
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
            Patch::AddAttributes(&"div", 2, vec![&attr("class", "some-class")]),
            Patch::RemoveChildren(&"main", 0, vec![0]),
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
            Patch::AddAttributes(&"div", 3, vec![&attr("class", "some-class")]),
            Patch::RemoveChildren(&"article", 1, vec![0]),
        ]
    );
}

#[test]
fn deep_nested_more_children_key0_and_key1_removed_at_start_then_key2_has_additional_attributes() {
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
            Patch::AddAttributes(&"div", 4, vec![&attr("class", "some-class")]),
            Patch::RemoveChildren(&"article", 1, vec![0, 1]),
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
                        element("a", vec![attr("href", "#link1")], vec![text("Click here")]),
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
                    element("p", vec![], vec![text("paragraph1, with added content")]),
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
            Patch::AddAttributes(&"div", 4, vec![&attr("class", "some-class")]),
            Patch::ChangeText(6, "paragraph1, with added content"),
            Patch::ChangeText(8, "Click here to continue"),
            Patch::RemoveChildren(&"article", 1, vec![0, 1]),
        ]
    );
}
