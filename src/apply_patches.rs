//! apply patches for verifying the patches are correct when current_dom will be equal to the
//! target_dom when patches is applied.
//!
use crate::{Node, NodeIdx, Patch};
use std::fmt;
use std::fmt::Debug;

/// had to find the node each time, since rust does not allow multiple mutable borrows
/// ISSUE: once a destructive patch such as RemoveChildren, InsertNode, ReplaceNode is applied
/// the Nodeidx is not synch with the root_node anymore.
///
/// TODO: zipper might be feasible to use here
///
/// To minimize this issue, destructive patches is applied last.
/// It doesn't elimiate the problem completely, and it will arise
/// when there are multiple destructive patch.
pub fn apply_patches<'a, NS, TAG, ATT, VAL, EVENT>(
    root_node: &mut Node<NS, TAG, ATT, VAL, EVENT>,
    patches: &[Patch<'a, NS, TAG, ATT, VAL, EVENT>],
) where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    for patch in patches {
        match patch {
            Patch::AppendChildren(ac) => {
                let target_node = find_node(root_node, ac.node_idx)
                    .expect("must have found the target node");
                let children: Vec<Node<NS, TAG, ATT, VAL, EVENT>> = ac
                    .children
                    .iter()
                    .map(|(_idx, c)| *c)
                    .map(|c| c.clone())
                    .collect();
                let target_element =
                    target_node.as_element_mut().expect("expecting an element");
                target_element.children.extend(children);
            }
            Patch::InsertNode(ic) => {
                insert_node(root_node, ic.node_idx, ic.node);
            }
            Patch::RemoveNode(rn) => {
                remove_node(root_node, rn.node_idx);
            }
            Patch::ReplaceNode(rn) => {
                let target_node = find_node(root_node, rn.node_idx)
                    .expect("must have a target node");
                let target_element =
                    target_node.as_element_mut().expect("expecting an element");

                let re = rn
                    .replacement
                    .as_element_ref()
                    .expect("expecting an element");
                target_element.namespace = re.namespace.clone();
                target_element.tag = re.tag.clone();
                target_element.attrs = re.attrs.clone();
                target_element.children = re.children.clone();
                target_element.self_closing = re.self_closing;
            }
            Patch::AddAttributes(at) => {
                let target_node = find_node(root_node, at.node_idx)
                    .expect("must have a target node");
                let target_element =
                    target_node.as_element_mut().expect("expecting an element");

                target_element.add_attributes(
                    at.attrs.iter().map(|a| *a).map(|a| a.clone()).collect(),
                )
            }
            Patch::RemoveAttributes(rt) => {
                let target_node = find_node(root_node, rt.node_idx)
                    .expect("must have a target node");
                let target_element =
                    target_node.as_element_mut().expect("expecting an element");

                let remove_attrs_name =
                    rt.attrs.iter().map(|a| &a.name).collect::<Vec<_>>();

                let new_attrs = target_element
                    .attrs
                    .iter()
                    .filter(|a| !remove_attrs_name.contains(&&a.name))
                    .map(|a| a.clone())
                    .collect::<Vec<_>>();

                target_element.attrs = new_attrs;
            }
            Patch::ChangeText(ct) => {
                dbg!(&ct);
                let target_node =
                    find_node(root_node, ct.node_idx).expect("must find node");
                dbg!(&target_node);
                if let Node::Text(old_txt) = target_node {
                    old_txt.set_text(&ct.new.text);
                } else {
                    unreachable!("expecting a text node");
                }
            }
        }
    }
}

pub(crate) fn find_node<'a, NS, TAG, ATT, VAL, EVENT>(
    node: &'a mut Node<NS, TAG, ATT, VAL, EVENT>,
    node_idx: NodeIdx,
) -> Option<&'a mut Node<NS, TAG, ATT, VAL, EVENT>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    find_node_recursive(node, node_idx, &mut 0)
}

fn find_node_recursive<'a, NS, TAG, ATT, VAL, EVENT>(
    node: &'a mut Node<NS, TAG, ATT, VAL, EVENT>,
    node_idx: NodeIdx,
    cur_node_idx: &mut usize,
) -> Option<&'a mut Node<NS, TAG, ATT, VAL, EVENT>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    if node_idx == *cur_node_idx {
        Some(node)
    } else if let Some(children) = node.children_mut() {
        children.iter_mut().find_map(|child| {
            *cur_node_idx += 1;
            find_node_recursive(child, node_idx, cur_node_idx)
        })
    } else {
        None
    }
}

fn remove_node<'a, NS, TAG, ATT, VAL, EVENT>(
    node: &'a mut Node<NS, TAG, ATT, VAL, EVENT>,
    node_idx: NodeIdx,
) -> bool
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    println!("to be removed node_idx: {}", node_idx);
    remove_node_recursive(node, node_idx, &mut 0)
}

/// remove node, if the child matches the cur_node_idx remove it
/// Note: it is processed this way since we need a reference to the parent
/// node to remove the target node_idx
fn remove_node_recursive<'a, NS, TAG, ATT, VAL, EVENT>(
    node: &'a mut Node<NS, TAG, ATT, VAL, EVENT>,
    node_idx: NodeIdx,
    cur_node_idx: &mut usize,
) -> bool
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    if let Some(element) = node.as_element_mut() {
        let mut this_cur_node_idx = *cur_node_idx;
        let mut to_be_remove = None;
        // look ahead for remove
        for (idx, child) in element.children.iter().enumerate() {
            this_cur_node_idx += 1;
            if node_idx == this_cur_node_idx {
                to_be_remove = Some(idx);
            } else {
                crate::diff::increment_node_idx_to_descendant_count(
                    child,
                    &mut this_cur_node_idx,
                );
            }
        }

        if let Some(remove_idx) = to_be_remove {
            element.children.remove(remove_idx);
            true
        } else {
            for child in element.children.iter_mut() {
                *cur_node_idx += 1;
                if remove_node_recursive(child, node_idx, cur_node_idx) {
                    return true;
                }
            }
            false
        }
    } else {
        false
    }
}

fn insert_node<'a, NS, TAG, ATT, VAL, EVENT>(
    node: &'a mut Node<NS, TAG, ATT, VAL, EVENT>,
    node_idx: NodeIdx,
    for_insert: &'a Node<NS, TAG, ATT, VAL, EVENT>,
) -> bool
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    insert_node_recursive(node, node_idx, for_insert, &mut 0)
}

fn insert_node_recursive<'a, NS, TAG, ATT, VAL, EVENT>(
    node: &'a mut Node<NS, TAG, ATT, VAL, EVENT>,
    node_idx: NodeIdx,
    for_insert: &'a Node<NS, TAG, ATT, VAL, EVENT>,
    cur_node_idx: &mut usize,
) -> bool
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    if let Some(element) = node.as_element_mut() {
        let mut this_cur_node_idx = *cur_node_idx;
        let mut target_insert_idx = None;
        for (i, child) in element.children.iter().enumerate() {
            this_cur_node_idx += 1;
            if node_idx == this_cur_node_idx {
                target_insert_idx = Some(i);
            } else {
                crate::diff::increment_node_idx_to_descendant_count(
                    child,
                    &mut this_cur_node_idx,
                );
            }
        }
        if let Some(target_insert_idx) = target_insert_idx {
            println!("inserted node here at {}", target_insert_idx);
            element
                .children
                .insert(target_insert_idx, for_insert.clone());
            true
        } else {
            for child in element.children.iter_mut() {
                *cur_node_idx += 1;
                if insert_node_recursive(
                    child,
                    node_idx,
                    for_insert,
                    cur_node_idx,
                ) {
                    return true;
                }
            }
            false
        }
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    pub type MyNode =
        Node<&'static str, &'static str, &'static str, &'static str, ()>;

    #[test]
    fn test_find_node_simple() {
        let mut old: MyNode = element(
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
        let found = find_node(&mut old, 11);
        dbg!(&found);
        assert_eq!(found, Some(&mut text("3 items left")));
    }

    #[test]
    fn test_find_node_numbered() {
        let mut old: MyNode = element(
            "elm0",
            vec![attr("node", "0")],
            vec![
                element("elm1", vec![attr("node", "1")], vec![text("Elm2")]),
                element(
                    "elm3",
                    vec![attr("node", "3")],
                    vec![
                        element(
                            "elm4",
                            vec![attr("node", "4")],
                            vec![text("Elm5")],
                        ),
                        element(
                            "elm6",
                            vec![attr("node", "6")],
                            vec![text("Elm7")],
                        ),
                        element(
                            "elm8",
                            vec![attr("node", "8")],
                            vec![text("Elm9")],
                        ),
                    ],
                ),
                element("elm10", vec![attr("node", "10")], vec![text("Elm11")]),
            ],
        );

        let found = find_node(&mut old, 9).expect("must find the 9th node");
        dbg!(&found);
        assert_eq!(found, &text("Elm9"));
        assert_eq!(find_node(&mut old, 11).unwrap(), &text("Elm11"));
    }
}
