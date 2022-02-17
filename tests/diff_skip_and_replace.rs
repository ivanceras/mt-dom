#![deny(warnings)]
use mt_dom::{diff::diff_with_functions, patch::*, *};

type MyNode = Node<&'static str, &'static str, &'static str, &'static str>;

#[test]
fn force_replace() {
    let old: MyNode =
        element("div", vec![attr("class", "[0]"), attr("id", "0")], vec![]);
    let new =
        element("div", vec![attr("class", "[0]"), attr("id", "0")], vec![]);

    let skip = |_old, _new| false;
    let replace = |_old, _new| true;

    let diff = diff_with_functions(&old, &new, &"key", &skip, &replace);
    assert_eq!(
        diff,
        vec![Patch::replace_node(
            Some(&"div"),
            TreePath::new(vec![0]),
            &new
        )],
    );
}

#[test]
fn force_skip() {
    let old: MyNode =
        element("div", vec![attr("class", "[0]"), attr("id", "0")], vec![]);
    let new =
        element("div", vec![attr("class", "[0]"), attr("id", "0")], vec![]);

    let skip = |_old, _new| true;
    let replace = |_old, _new| false;

    let diff = diff_with_functions(&old, &new, &"key", &skip, &replace);
    assert_eq!(diff, vec![],);
}

#[test]
fn skip_in_attribute() {
    let old: MyNode =
        element("div", vec![attr("class", "[0]"), attr("id", "0")], vec![]);
    let new = element(
        "div",
        vec![attr("skip", "true"), attr("class", "[1]"), attr("id", "1")],
        vec![],
    );

    let skip = |_old, new: &MyNode| {
        if let Some(attributes) = new.get_attributes() {
            attributes
                .iter()
                .filter(|a| a.name == "skip")
                .flat_map(|a| a.value())
                .any(|v| *v == "true")
        } else {
            false
        }
    };
    let replace = |_old, _new| false;

    let diff = diff_with_functions(&old, &new, &"key", &skip, &replace);
    assert_eq!(diff, vec![],);
}

#[test]
fn replace_true_in_attribute_must_replace_old_node_regardless() {
    let old: MyNode =
        element("div", vec![attr("class", "[0]"), attr("id", "0")], vec![]);
    let new = element(
        "div",
        vec![
            attr("replace", "true"),
            attr("class", "[1]"),
            attr("id", "1"),
        ],
        vec![],
    );

    let skip = |_old, _new| false;
    let replace = |_old, new: &MyNode| {
        if let Some(attributes) = new.get_attributes() {
            attributes
                .iter()
                .filter(|a| a.name == "replace")
                .flat_map(|a| a.value())
                .any(|v| *v == "true")
        } else {
            false
        }
    };

    let diff = diff_with_functions(&old, &new, &"key", &skip, &replace);
    assert_eq!(
        diff,
        vec![Patch::replace_node(
            Some(&"div"),
            TreePath::new(vec![0]),
            &new
        )],
    );
}

#[test]
fn replace_and_skip_in_sub_nodes() {
    let old: MyNode = element(
        "div",
        vec![attr("class", "[0]"), attr("id", "0")],
        vec![
            element(
                "div",
                vec![attr("class", "[0,0]"), attr("id", "1")],
                vec![
                    element(
                        "div",
                        vec![attr("class", "[0,0,0]"), attr("id", "2")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,0,1]"), attr("id", "3")],
                        vec![],
                    ),
                ],
            ),
            element(
                "div",
                vec![attr("class", "[0,1]"), attr("id", "4")],
                vec![
                    element(
                        "div",
                        vec![attr("class", "[0,1,0]"), attr("id", "5")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,1,1]"), attr("id", "6")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,1,2]"), attr("id", "7")],
                        vec![],
                    ),
                ],
            ),
        ],
    );

    let new: MyNode = element(
        "div",
        vec![attr("class", "[0]"), attr("id", "0")],
        vec![
            element(
                "div",
                vec![
                    attr("skip", "true"),
                    attr("class", "[0,0]-differs"),
                    attr("id", "1"),
                ],
                vec![
                    element(
                        "div",
                        vec![attr("class", "[0,0,0]"), attr("id", "2")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,0,1]"), attr("id", "3")],
                        vec![],
                    ),
                ],
            ),
            element(
                "div",
                vec![
                    attr("replace", "true"),
                    attr("class", "[0,1]"),
                    attr("id", "4"),
                ],
                vec![
                    element(
                        "div",
                        vec![attr("class", "[0,1,0]"), attr("id", "5")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,1,1]"), attr("id", "6")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,1,2]"), attr("id", "7")],
                        vec![],
                    ),
                ],
            ),
        ],
    );

    let skip = |_old, new: &MyNode| {
        if let Some(attributes) = new.get_attributes() {
            attributes
                .iter()
                .filter(|a| a.name == "skip")
                .flat_map(|a| a.value())
                .any(|v| *v == "true")
        } else {
            false
        }
    };
    let replace = |_old, new: &MyNode| {
        if let Some(attributes) = new.get_attributes() {
            attributes
                .iter()
                .filter(|a| a.name == "replace")
                .flat_map(|a| a.value())
                .any(|v| *v == "true")
        } else {
            false
        }
    };

    let diff = diff_with_functions(&old, &new, &"key", &skip, &replace);
    assert_eq!(
        diff,
        vec![Patch::replace_node(
            Some(&"div"),
            TreePath::new(vec![0, 1]),
            &element(
                "div",
                vec![
                    attr("replace", "true"),
                    attr("class", "[0,1]"),
                    attr("id", "4"),
                ],
                vec![
                    element(
                        "div",
                        vec![attr("class", "[0,1,0]"), attr("id", "5")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,1,1]"), attr("id", "6")],
                        vec![],
                    ),
                    element(
                        "div",
                        vec![attr("class", "[0,1,2]"), attr("id", "7")],
                        vec![],
                    ),
                ],
            )
        )],
    );
}
