#![deny(warnings)]
use mt_dom::{patch::*, *};


#[test]
fn test_node_list() {
    let old_list = node_list([
        element("li", [attr("key", "1")], []),
        element("li", [attr("key", "2")], []),
    ]);

    let old: Node = element(
        "div",
        [],
        [
            element("li", [attr("key", "0")], []),
            old_list,
            element("li", [attr("key", "3")], []),
        ],
    );

    let new_list = node_list([element("li", [attr("key", "1")], [])]);
    let new: Node = element(
        "div",
        [],
        [
            element("li", [attr("key", "0")], []),
            new_list,
            element("li", [attr("key", "3")], []),
        ],
    );

    println!("old: {:#?}", old);

    let diff = diff_with_key(&old, &new);
    assert_eq!(
        diff,
        vec![Patch::remove_node(Some(&"li"), TreePath::new(vec![2]),)],
    );
}
