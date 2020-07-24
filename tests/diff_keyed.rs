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
            Patch::RemoveChildren(&"main", 0, vec![0]),
            Patch::AddAttributes(&"div", 0, vec![&attr("class", "some-class")])
        ]
    );
}
