use crate::Attribute;
use crate::Element;
use crate::Node;
use std::cmp;
use std::fmt::Debug;
use std::mem;

/// A Patch encodes an operation that modifies a real DOM element or native UI element
///
/// To update the real DOM that a user sees you'll want to first diff your
/// old virtual dom and new virtual dom.
///
/// This diff operation will generate `Vec<Patch>` with zero or more patches that, when
/// applied to your real DOM, will make your real DOM look like your new virtual dom.
///
/// Each Patch has a usize node index that helps us identify the real DOM node that it applies to.
///
/// Our old virtual dom's nodes are indexed depth first, as shown in this illustration
/// (0 being the root node, 1 being it's first child, 2 being it's first child's first child).
///
/// ```ignore
///             .─.
///            ( 0 )
///             `-'
///            /   \
///           /     \
///          /       \
///         ▼         ▼
///        .─.         .─.
///       ( 1 )       ( 4 )
///        `-'         `-'
///       /  \          | \ '.
///      /    \         |  \  '.
///     ▼      ▼        |   \   '.
///   .─.      .─.      ▼    ▼     ▼
///  ( 2 )    ( 3 )    .─.   .─.   .─.
///   `─'      `─'    ( 5 ) ( 6 ) ( 7 )
///                    `─'   `─'   `─'
/// ```
///
///
#[derive(Debug, PartialEq)]
pub enum Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG> {
    /// Append a vector of child nodes to a parent node id.
    AppendChildren(
        &'a TAG,
        NodeIdx,
        Vec<&'a Node<NS, TAG, ATT, VAL, EVENT, MSG>>,
    ),
    /// remove all children besides the first `len`
    TruncateChildren(&'a TAG, NodeIdx, usize),
    /// Replace a node with another node. This typically happens when a node's tag changes.
    /// ex: <div> becomes <span>
    Replace(&'a TAG, NodeIdx, &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>),
    /// Add attributes that the new node has that the old node does not
    /// Note: the attributes is not a reference since attributes of same
    /// name are merged to produce a new unify attribute
    AddAttributes(&'a TAG, NodeIdx, Vec<Attribute<NS, ATT, VAL, EVENT, MSG>>),
    /// Remove attributes that the old node had that the new node doesn't
    RemoveAttributes(
        &'a TAG,
        NodeIdx,
        Vec<&'a Attribute<NS, ATT, VAL, EVENT, MSG>>,
    ),
    /// Change the text of a Text node.
    ChangeText(NodeIdx, &'a str),
}

type NodeIdx = usize;

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG> Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG> {
    /// Every Patch is meant to be applied to a specific node within the DOM. Get the
    /// index of the DOM node that this patch should apply to. DOM nodes are indexed
    /// depth first with the root node in the tree having index 0.
    pub fn node_idx(&self) -> NodeIdx {
        match self {
            Patch::AppendChildren(_tag, node_idx, _) => *node_idx,
            Patch::TruncateChildren(_tag, node_idx, _) => *node_idx,
            Patch::Replace(_tag, node_idx, _) => *node_idx,
            Patch::AddAttributes(_tag, node_idx, _) => *node_idx,
            Patch::RemoveAttributes(_tag, node_idx, _) => *node_idx,
            Patch::ChangeText(node_idx, _) => *node_idx,
        }
    }

    /// return the tag of this patch
    pub fn tag(&self) -> Option<&TAG> {
        match self {
            Patch::AppendChildren(tag, _node_idx, _) => Some(tag),
            Patch::TruncateChildren(tag, _node_idx, _) => Some(tag),
            Patch::Replace(tag, _node_idx, _) => Some(tag),
            Patch::AddAttributes(tag, _node_idx, _) => Some(tag),
            Patch::RemoveAttributes(tag, _node_idx, _) => Some(tag),
            Patch::ChangeText(_node_idx, _) => None,
        }
    }
}

/// calculate the difference of 2 nodes
/// the supplied key will be taken into account
/// that if the 2 keys differ, the element will be replaced without having to traverse the children
/// nodes
pub fn diff_with_key<'a, 'b, NS, TAG, ATT, VAL, EVENT, MSG>(
    old: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    new: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    key: &ATT,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
where
    TAG: PartialEq + Debug,
    ATT: PartialEq + Clone + Debug,
    NS: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    diff_recursive(old, new, &mut 0, key)
}

/// utility function to recursively increment the node_idx baed on the node tree which depends on the children
/// count
fn increment_node_idx_for_children<NS, TAG, ATT, VAL, EVENT, MSG>(
    old: &Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    cur_node_idx: &mut usize,
) {
    *cur_node_idx += 1;
    if let Node::Element(element_node) = old {
        for child in element_node.children.iter() {
            increment_node_idx_for_children(&child, cur_node_idx);
        }
    }
}

fn diff_recursive<'a, 'b, NS, TAG, ATT, VAL, EVENT, MSG>(
    old: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    new: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    cur_node_idx: &'b mut usize,
    key: &ATT,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    let mut patches = vec![];

    // Different enum variants, replace!
    let mut replace = mem::discriminant(old) != mem::discriminant(new);

    if let (Node::Element(old_element), Node::Element(new_element)) = (old, new) {
        // Replace if there are different element tags
        if old_element.tag != new_element.tag {
            replace = true;
        }

        let new_attributes: Vec<Attribute<NS, ATT, VAL, EVENT, MSG>> =
            new_element.merge_attributes();
        let old_attributes: Vec<Attribute<NS, ATT, VAL, EVENT, MSG>> =
            old_element.merge_attributes();

        // Replace if two elements have different keys
        let old_key_value = old_attributes.iter().find(|att| att.name == *key);
        let new_key_value = new_attributes.iter().find(|att| att.name == *key);

        dbg!(old_key_value);
        dbg!(new_key_value);

        match (old_key_value, new_key_value) {
            (Some(old_kv), Some(new_kv)) => {
                if old_kv != new_kv {
                    dbg!("will replace");
                    replace = true;
                    dbg!(replace);
                } else {
                    dbg!("will not replace..");
                }
            }
            _ => (),
        }
    }

    // Handle replacing of a node
    if replace {
        dbg!("yes it is a replace");
        patches.push(Patch::Replace(
            old.tag().expect("must have a tag"),
            *cur_node_idx,
            &new,
        ));
        if let Node::Element(old_element_node) = old {
            for child in old_element_node.children.iter() {
                increment_node_idx_for_children(child, cur_node_idx);
            }
        }
        dbg!("returning the patches..");
        return patches;
    }

    dbg!("no, it wasnt a replace... continuing...");

    // The following comparison can only contain identical variants, other
    // cases have already been handled above by comparing variant
    // discriminants.
    match (old, new) {
        // We're comparing two text nodes
        (Node::Text(old_text), Node::Text(new_text)) => {
            if old_text != new_text {
                patches.push(Patch::ChangeText(*cur_node_idx, &new_text));
            }
        }

        // We're comparing two element nodes
        (Node::Element(old_element), Node::Element(new_element)) => {
            let attributes_patches = diff_attributes(old_element, new_element, cur_node_idx);
            patches.extend(attributes_patches);

            let old_child_count = old_element.children.len();
            let new_child_count = new_element.children.len();

            if new_child_count > old_child_count {
                let append_patch: Vec<&'a Node<NS, TAG, ATT, VAL, EVENT, MSG>> =
                    new_element.children[old_child_count..].iter().collect();
                patches.push(Patch::AppendChildren(
                    &old_element.tag,
                    *cur_node_idx,
                    append_patch,
                ))
            }

            if new_child_count < old_child_count {
                patches.push(Patch::TruncateChildren(
                    &old_element.tag,
                    *cur_node_idx,
                    new_child_count,
                ))
            }

            let min_count = cmp::min(old_child_count, new_child_count);
            for index in 0..min_count {
                *cur_node_idx += 1;
                let old_child = &old_element.children.get(index).expect("No old child node");
                let new_child = &new_element.children.get(index).expect("No new chold node");
                patches.append(&mut diff_recursive(
                    &old_child,
                    &new_child,
                    cur_node_idx,
                    key,
                ))
            }
            if new_child_count < old_child_count {
                for child in old_element.children[min_count..].iter() {
                    increment_node_idx_for_children(child, cur_node_idx);
                }
            }
        }
        (Node::Text(_), Node::Element(_)) | (Node::Element(_), Node::Text(_)) => {
            unreachable!("Unequal variant discriminants should already have been handled");
        }
    };

    patches
}

/// check if set1 has the contains the items in set2 and vice versa, regardless of their order
fn is_same_set<T: PartialEq>(set1: &[T], set2: &[T]) -> bool {
    set1.iter().all(|item1| set2.contains(item1)) && set2.iter().all(|item2| set1.contains(item2))
}

/// diff the attributes of old element to the new element at this cur_node_idx
fn diff_attributes<'a, 'b, NS, TAG, ATT, VAL, EVENT, MSG>(
    old_element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    new_element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    cur_node_idx: &'b mut usize,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
where
    NS: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    let mut patches = vec![];
    let mut add_attributes: Vec<Attribute<NS, ATT, VAL, EVENT, MSG>> = vec![];
    let mut remove_attributes: Vec<&Attribute<NS, ATT, VAL, EVENT, MSG>> = vec![];

    let new_attributes: Vec<Attribute<NS, ATT, VAL, EVENT, MSG>> = new_element.merge_attributes();
    let old_attributes: Vec<Attribute<NS, ATT, VAL, EVENT, MSG>> = old_element.merge_attributes();
    // for all new elements that doesn't exist in the old elements
    // or the values differ
    // add it to the AddAttribute patches
    for new_attr in new_attributes.iter() {
        let old_attr_value = old_attributes
            .iter()
            .find(|att| att.name == new_attr.name)
            .map(|att| &att.value);

        if let Some(old_attr_value) = old_attr_value {
            //if old_attr_value != new_attr.value {
            if !is_same_set(old_attr_value, &new_attr.value) {
                add_attributes.push(new_attr.clone());
            }
        } else {
            add_attributes.push(new_attr.clone());
        }
    }

    // if this attribute name does not exist anymore
    // to the new element, remove it
    for old_attr in old_element.get_attributes().iter() {
        if let Some(_pre_attr) = new_attributes.iter().find(|att| att.name == old_attr.name) {
            //
        } else {
            remove_attributes.push(&old_attr);
        }
    }

    if !add_attributes.is_empty() {
        patches.push(Patch::AddAttributes(
            &old_element.tag,
            *cur_node_idx,
            add_attributes,
        ));
    }
    if !remove_attributes.is_empty() {
        patches.push(Patch::RemoveAttributes(
            &old_element.tag,
            *cur_node_idx,
            remove_attributes,
        ));
    }
    patches
}
