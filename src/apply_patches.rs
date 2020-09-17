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

pub fn apply_patches<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    root_node: &mut Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    patches: &[Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>],
) where
    NS: Clone + fmt::Debug,
    TAG: Clone + fmt::Debug,
    ATT: Clone + fmt::Debug + PartialEq,
    VAL: Clone + fmt::Debug,
{
    let target_nodes_idx: HashMap<NodeIdx, Option<&TAG>> = HashMap::from_iter(
        patches.iter().map(|patch| (patch.node_idx(), patch.tag())),
    );

    let mut target_nodes =
        find_nodes_to_patch(root_node, &mut 0, &target_nodes_idx);

    dbg!(&target_nodes);
    for patch in patches {
        match patch {
            Patch::AppendChildren(ac) => {
                let target_node = target_nodes
                    .get_mut(&ac.node_idx)
                    .expect("must have found the target node");
                let children: Vec<Node<NS, TAG, ATT, VAL, EVENT, MSG>> =
                    ac.children.iter().map(|c| *c).map(|c| c.clone()).collect();
                let target_element =
                    target_node.as_element_mut().expect("expecting an element");
                target_element.children.extend(children);
            }
            Patch::InsertChildren(ic) => {
                let target_node = target_nodes
                    .get_mut(&ic.node_idx)
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
                let target_node = target_nodes
                    .get_mut(&rc.node_idx)
                    .expect("must have a target node");
                let target_element =
                    target_node.as_element_mut().expect("must be an element");
                for idx in rc.target_index.iter().rev() {
                    target_element.children.remove(*idx);
                }
            }
            Patch::ReplaceNode(rn) => {
                let target_node = target_nodes
                    .get_mut(&rn.node_idx)
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
                let target_node = target_nodes
                    .get_mut(&at.node_idx)
                    .expect("must have a target node");
                let target_element =
                    target_node.as_element_mut().expect("expecting an element");

                target_element.add_attributes(
                    at.attrs.iter().map(|a| *a).map(|a| a.clone()).collect(),
                )
            }
            Patch::RemoveAttributes(rt) => {
                let target_node = target_nodes
                    .get_mut(&rt.node_idx)
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
                let target_node = target_nodes
                    .get_mut(&ct.node_idx)
                    .expect("must have a target node");
                if let Node::Text(old_txt) = target_node {
                    *old_txt = ct.new.to_string();
                } else {
                    unreachable!("expecting a text node");
                }
            }
        }
    }
}

fn find_nodes_to_patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    node: &'a mut Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    cur_node_idx: &mut usize,
    target_nodes_idx: &HashMap<NodeIdx, Option<&TAG>>,
) -> HashMap<NodeIdx, &'a mut Node<NS, TAG, ATT, VAL, EVENT, MSG>>
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    let mut target_nodes = HashMap::new();
    if let Some(_tag) = target_nodes_idx.get(cur_node_idx) {
        target_nodes.insert(*cur_node_idx, node);
    } else {
        match node {
            Node::Element(element) => {
                for child in element.children_mut() {
                    *cur_node_idx += 1;
                    target_nodes.extend(find_nodes_to_patch(
                        child,
                        cur_node_idx,
                        target_nodes_idx,
                    ));
                }
            }
            Node::Text(text) => {}
        }
    }
    target_nodes
}
