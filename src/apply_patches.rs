use crate::{
    Node,
    NodeIdx,
    Patch,
};
use std::{
    collections::HashMap,
    fmt,
    iter::FromIterator,
};

/// had to find the node each time, since rust does not allow multiple mutable borrows
/// ISSUE: once a destructive patch such as RemoveChildren, InsertChildren, ReplaceNode is applied
/// the Nodeidx is not synch with the root_node anymore.
///
/// To minimize this issue, destructive patches is applied last.
/// It doesn't elimiate the problem completely, and it will arise
/// when there are multiple destructive patch.
pub fn apply_patches<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    root_node: &mut Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    patches: &[Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>],
) where
    NS: Clone + fmt::Debug,
    TAG: Clone + fmt::Debug,
    ATT: Clone + fmt::Debug + PartialEq,
    VAL: Clone + fmt::Debug,
{
    for patch in patches {
        match patch {
            Patch::AppendChildren(ac) => {
                let target_node = find_node(root_node, ac.node_idx)
                    .expect("must have found the target node");
                let children: Vec<Node<NS, TAG, ATT, VAL, EVENT, MSG>> =
                    ac.children.iter().map(|c| *c).map(|c| c.clone()).collect();
                let target_element =
                    target_node.as_element_mut().expect("expecting an element");
                target_element.children.extend(children);
            }
            Patch::InsertChildren(ic) => {
                let target_node = find_node(root_node, ic.node_idx)
                    .expect("must have found the target node");
                let children: Vec<Node<NS, TAG, ATT, VAL, EVENT, MSG>> =
                    ic.children.iter().map(|c| *c).map(|c| c.clone()).collect();
                let target_element =
                    target_node.as_element_mut().expect("must be an element");

                // insert element starting from the last
                for child in children.into_iter().rev() {
                    target_element.children.insert(ic.target_index, child);
                }
            }
            Patch::RemoveChildren(rc) => {
                let target_node = find_node(root_node, rc.node_idx)
                    .expect("must have a target node");
                let target_element =
                    target_node.as_element_mut().expect("must be an element");
                for idx in rc.target_index.iter().rev() {
                    target_element.children.remove(*idx);
                }
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
                    *old_txt = ct.new.to_string();
                } else {
                    unreachable!("expecting a text node");
                }
            }
        }
    }
}

pub fn find_node<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    node: &'a mut Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    node_idx: NodeIdx,
) -> Option<&'a mut Node<NS, TAG, ATT, VAL, EVENT, MSG>>
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    find_node_recursive(node, node_idx, &mut 0)
}

fn find_node_recursive<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    node: &'a mut Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    node_idx: NodeIdx,
    cur_node_idx: &mut usize,
) -> Option<&'a mut Node<NS, TAG, ATT, VAL, EVENT, MSG>>
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
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

fn remove_node<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    node: &'a mut Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    node_idx: NodeIdx,
) -> bool
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    println!("to be removed node_idx: {}", node_idx);
    remove_node_recursive(node, node_idx, &mut 0)
}

//remove node, if the child matches the cur_node_idx remove it
fn remove_node_recursive<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    node: &'a mut Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    node_idx: NodeIdx,
    cur_node_idx: &mut usize,
) -> bool
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    if let Some(element) = node.as_element_mut() {
        let mut this_cur_node_idx = *cur_node_idx;
        let mut to_be_remove = None;
        // look ahead for remove
        for (idx, child) in element.children.iter().enumerate() {
            this_cur_node_idx += 1;
            println!(
                "\tfinding node_idx: {}, this_cur_node_idx: {} cur_node_idx: {}",
                node_idx, this_cur_node_idx, cur_node_idx
            );
            println!(
                "\tINCREMENTED finding node_idx: {}, this_cur_node_idx: {}",
                node_idx, this_cur_node_idx
            );
            if node_idx == this_cur_node_idx {
                println!(
                    "got something to be removed: {} child: {:?}",
                    idx, child
                );
                to_be_remove = Some(idx);
            } else {
                crate::diff::increment_node_idx_to_descendant_count(
                    child,
                    &mut this_cur_node_idx,
                );
            }
        }

        if let Some(remove_idx) = to_be_remove {
            let removed = element.children.remove(remove_idx);
            println!("removed: {:?}", removed);
            return true;
        } else {
            println!("to be removed is not found... trying out deeper..");
            for (idx, child) in element.children.iter_mut().enumerate() {
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

use std::{
    ops::DerefMut,
    sync::{
        Arc,
        Mutex,
    },
};

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    pub type MyNode =
        Node<&'static str, &'static str, &'static str, &'static str, (), ()>;

    #[test]
    fn test_find_node() {
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
