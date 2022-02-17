use mt_dom::{diff::*, patch::*, *};

pub type MyNode = Node<&'static str, &'static str, &'static str, &'static str>;

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
        vec![Patch::insert_node(
            Some(&"main"),
            TreePath::new(vec![0, 0]),
            &element("div", vec![attr("key", "XXX")], vec![text("lineXXX")])
        )]
    );
}

#[test]
fn key_inserted_at_middle() {
    pretty_env_logger::try_init().ok();
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
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::insert_node(
            Some(&"main"),
            TreePath::new(vec![0, 5]),
            &element("div", vec![attr("key", "XXX")], vec![text("lineXXX")])
        )]
    );
}

#[test]
fn wrapped_elements() {
    pretty_env_logger::try_init().ok();
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
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![Patch::insert_node(
            Some(&"main"),
            TreePath::new(vec![0, 0, 5]),
            &element("div", vec![attr("key", "XXX")], vec![text("lineXXX")])
        )]
    );
}

#[test]
fn text_changed() {
    pretty_env_logger::try_init().ok();
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
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![Patch::change_text(
            TreePath::new(vec![0, 0, 6, 0]),
            &Text::new("line7"),
            &Text::new("line7_changed")
        )]
    );
}

#[test]
fn text_changed_non_keyed() {
    pretty_env_logger::try_init().ok();
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
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![Patch::change_text(
            TreePath::new(vec![0, 0, 6, 0]),
            &Text::new("line7"),
            &Text::new("line7_changed")
        )]
    );
}

#[test]
fn insert_one_line_at_start() {
    pretty_env_logger::try_init().ok();
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
    dbg!(&diff);
    assert_eq!(
        diff,
        vec![
            Patch::change_text(
                TreePath::new(vec![0, 0, 0, 0, 0]),
                &Text::new("1"),
                &Text::new("2")
            ),
            Patch::change_text(
                TreePath::new(vec![0, 0, 1, 0, 0]),
                &Text::new("2"),
                &Text::new("3")
            ),
            Patch::change_text(
                TreePath::new(vec![0, 0, 2, 0, 0]),
                &Text::new("3"),
                &Text::new("4")
            ),
            Patch::insert_node(
                Some(&"main"),
                TreePath::new(vec![0, 0, 0]),
                &element(
                    "div",
                    vec![attr("key", "hashXXX")],
                    vec![
                        element("div", vec![], vec![text(1)]),
                        element("div", vec![], vec![text("XXX")]),
                    ],
                ),
            )
        ]
    );
}

#[test]
fn insert_two_lines_at_start() {
    pretty_env_logger::try_init().ok();
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
    dbg!(&diff);

    assert_eq!(
        diff,
        vec![
            Patch::change_text(
                TreePath::new(vec![0, 0, 0, 0, 0]),
                &Text::new("1"),
                &Text::new("3")
            ),
            Patch::change_text(
                TreePath::new(vec![0, 0, 1, 0, 0]),
                &Text::new("2"),
                &Text::new("4")
            ),
            Patch::change_text(
                TreePath::new(vec![0, 0, 2, 0, 0]),
                &Text::new("2"),
                &Text::new("5")
            ),
            Patch::insert_node(
                Some(&"main"),
                TreePath::new(vec![0, 0, 0]),
                &element(
                    "div",
                    vec![attr("key", "hashXXX")],
                    vec![
                        element("div", vec![], vec![text(1)]),
                        element("div", vec![], vec![text("XXX")]),
                    ],
                ),
            ),
            Patch::insert_node(
                Some(&"main"),
                TreePath::new(vec![0, 0, 0]),
                &element(
                    "div",
                    vec![attr("key", "hashYYY")],
                    vec![
                        element("div", vec![], vec![text(2)]),
                        element("div", vec![], vec![text("YYY")]),
                    ],
                )
            ),
        ]
    );
}
