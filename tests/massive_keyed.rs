use mt_dom::{
    diff::*,
    patch::*,
    *,
};

pub type MyNode =
    Node<&'static str, &'static str, &'static str, &'static str, (), ()>;

#[test]
fn key_inserted_at_start() {
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
            element("div", vec![attr("key", "XXX")], vec![text("lineXXX")]),
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
        vec![InsertNode::new(
            Some(&"main"),
            1,
            &element("div", vec![attr("key", "XXX")], vec![text("lineXXX")])
        )
        .into()]
    );
}

#[test]
fn key_inserted_at_middle() {
    pretty_env_logger::try_init();
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
            element("div", vec![attr("key", "1")], vec![text("line1")]),
            element("div", vec![attr("key", "2")], vec![text("line2")]),
            element("div", vec![attr("key", "3")], vec![text("line3")]),
            element("div", vec![attr("key", "4")], vec![text("line4")]),
            element("div", vec![attr("key", "5")], vec![text("line5")]),
            element("div", vec![attr("key", "XXX")], vec![text("lineXXX")]),
            element("div", vec![attr("key", "6")], vec![text("line6")]),
            element("div", vec![attr("key", "7")], vec![text("line7")]),
            element("div", vec![attr("key", "8")], vec![text("line8")]),
            element("div", vec![attr("key", "9")], vec![text("line9")]),
        ],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![InsertNode::new(
            Some(&"main"),
            11,
            &element("div", vec![attr("key", "XXX")], vec![text("lineXXX")])
        )
        .into()]
    );
}

#[test]
fn wrapped_elements() {
    pretty_env_logger::try_init();
    let old: MyNode = element(
        "article",
        vec![],
        vec![element(
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
        )],
    );

    let new: MyNode = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element("div", vec![attr("key", "1")], vec![text("line1")]),
                element("div", vec![attr("key", "2")], vec![text("line2")]),
                element("div", vec![attr("key", "3")], vec![text("line3")]),
                element("div", vec![attr("key", "4")], vec![text("line4")]),
                element("div", vec![attr("key", "5")], vec![text("line5")]),
                element("div", vec![attr("key", "XXX")], vec![text("lineXXX")]),
                element("div", vec![attr("key", "6")], vec![text("line6")]),
                element("div", vec![attr("key", "7")], vec![text("line7")]),
                element("div", vec![attr("key", "8")], vec![text("line8")]),
                element("div", vec![attr("key", "9")], vec![text("line9")]),
            ],
        )],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![InsertNode::new(
            Some(&"main"),
            12,
            &element("div", vec![attr("key", "XXX")], vec![text("lineXXX")])
        )
        .into()]
    );
}

#[test]
fn text_changed() {
    pretty_env_logger::try_init();
    let old: MyNode = element(
        "article",
        vec![],
        vec![element(
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
        )],
    );

    let new: MyNode = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element("div", vec![attr("key", "1")], vec![text("line1")]),
                element("div", vec![attr("key", "2")], vec![text("line2")]),
                element("div", vec![attr("key", "3")], vec![text("line3")]),
                element("div", vec![attr("key", "4")], vec![text("line4")]),
                element("div", vec![attr("key", "5")], vec![text("line5")]),
                element("div", vec![attr("key", "6")], vec![text("line6")]),
                element(
                    "div",
                    vec![attr("key", "7")],
                    vec![text("line7_changed")],
                ),
                element("div", vec![attr("key", "8")], vec![text("line8")]),
                element("div", vec![attr("key", "9")], vec![text("line9")]),
            ],
        )],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![ChangeText::new(15, "line7", "line7_changed").into()]
    );
}

#[test]
fn text_changed_non_keyed() {
    pretty_env_logger::try_init();
    let old: MyNode = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element("div", vec![], vec![text("line1")]),
                element("div", vec![], vec![text("line2")]),
                element("div", vec![], vec![text("line3")]),
                element("div", vec![], vec![text("line4")]),
                element("div", vec![], vec![text("line5")]),
                element("div", vec![], vec![text("line6")]),
                element("div", vec![], vec![text("line7")]),
                element("div", vec![], vec![text("line8")]),
                element("div", vec![], vec![text("line9")]),
            ],
        )],
    );

    let new: MyNode = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element("div", vec![], vec![text("line1")]),
                element("div", vec![], vec![text("line2")]),
                element("div", vec![], vec![text("line3")]),
                element("div", vec![], vec![text("line4")]),
                element("div", vec![], vec![text("line5")]),
                element("div", vec![], vec![text("line6")]),
                element("div", vec![], vec![text("line7_changed")]),
                element("div", vec![], vec![text("line8")]),
                element("div", vec![], vec![text("line9")]),
            ],
        )],
    );

    let diff = diff_with_key(&old, &new, &"key");
    assert_eq!(
        diff,
        vec![ChangeText::new(15, "line7", "line7_changed").into()]
    );
}

#[test]
fn insert_one_line_at_start() {
    pretty_env_logger::try_init();
    let old: MyNode = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element(
                    "div",
                    vec![attr("key", "hash1")],
                    vec![
                        element("div", vec![], vec![text(1)]),
                        element("div", vec![], vec![text("line1")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash2")],
                    vec![
                        element("div", vec![], vec![text(2)]),
                        element("div", vec![], vec![text("line3")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash3")],
                    vec![
                        element("div", vec![], vec![text(3)]),
                        element("div", vec![], vec![text("line3")]),
                    ],
                ),
            ],
        )],
    );

    let new: MyNode = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element(
                    "div",
                    vec![attr("key", "hashXXX")],
                    vec![
                        element("div", vec![], vec![text(1)]),
                        element("div", vec![], vec![text("XXX")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash1")],
                    vec![
                        element("div", vec![], vec![text(2)]),
                        element("div", vec![], vec![text("line1")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash2")],
                    vec![
                        element("div", vec![], vec![text(3)]),
                        element("div", vec![], vec![text("line3")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash3")],
                    vec![
                        element("div", vec![], vec![text(4)]),
                        element("div", vec![], vec![text("line3")]),
                    ],
                ),
            ],
        )],
    );

    let diff = diff_with_key(&old, &new, &"key");
    println!("{:#?}", diff);
    assert_eq!(
        diff,
        vec![
            ChangeText::new(4, "1", "2").into(),
            ChangeText::new(9, "2", "3").into(),
            ChangeText::new(14, "3", "4").into(),
            InsertNode::new(
                Some(&"main"),
                2,
                &element(
                    "div",
                    vec![attr("key", "hashXXX")],
                    vec![
                        element("div", vec![], vec![text(1)]),
                        element("div", vec![], vec![text("XXX")]),
                    ],
                ),
            )
            .into()
        ]
    );
}

#[test]
fn insert_two_lines_at_start() {
    pretty_env_logger::try_init();
    let old: MyNode = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element(
                    "div",
                    vec![attr("key", "hash1")],
                    vec![
                        element("div", vec![], vec![text(1)]),
                        element("div", vec![], vec![text("line1")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash2")],
                    vec![
                        element("div", vec![], vec![text(2)]),
                        element("div", vec![], vec![text("line2")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash3")],
                    vec![
                        element("div", vec![], vec![text(2)]),
                        element("div", vec![], vec![text("line3")]),
                    ],
                ),
            ],
        )],
    );

    let new: MyNode = element(
        "article",
        vec![],
        vec![element(
            "main",
            vec![attr("class", "container")],
            vec![
                element(
                    "div",
                    vec![attr("key", "hashXXX")],
                    vec![
                        element("div", vec![], vec![text(1)]),
                        element("div", vec![], vec![text("XXX")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hashYYY")],
                    vec![
                        element("div", vec![], vec![text(2)]),
                        element("div", vec![], vec![text("YYY")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash1")],
                    vec![
                        element("div", vec![], vec![text(3)]),
                        element("div", vec![], vec![text("line1")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash2")],
                    vec![
                        element("div", vec![], vec![text(4)]),
                        element("div", vec![], vec![text("line2")]),
                    ],
                ),
                element(
                    "div",
                    vec![attr("key", "hash3")],
                    vec![
                        element("div", vec![], vec![text(5)]),
                        element("div", vec![], vec![text("line3")]),
                    ],
                ),
            ],
        )],
    );

    let diff = diff_with_key(&old, &new, &"key");
    println!("{:#?}", diff);

    assert_eq!(
        diff,
        vec![
            ChangeText::new(4, "1", "3").into(),
            ChangeText::new(9, "2", "4").into(),
            ChangeText::new(14, "2", "5").into(),
            InsertNode::new(
                Some(&"main"),
                2,
                &element(
                    "div",
                    vec![attr("key", "hashXXX")],
                    vec![
                        element("div", vec![], vec![text(1)]),
                        element("div", vec![], vec![text("XXX")]),
                    ],
                ),
            )
            .into(),
            InsertNode::new(
                Some(&"main"),
                3,
                &element(
                    "div",
                    vec![attr("key", "hashYYY")],
                    vec![
                        element("div", vec![], vec![text(2)]),
                        element("div", vec![], vec![text("YYY")]),
                    ],
                )
            )
            .into(),
        ]
    );
}
