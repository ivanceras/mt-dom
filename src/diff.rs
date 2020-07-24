use crate::Attribute;
use crate::Element;
use crate::Node;
use log::*;
use std::cmp;
use std::fmt;
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
#[derive(PartialEq)]
pub enum Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG> {
    /// Insert a vector of child nodes to the current node being patch.
    InsertChildren(
        &'a TAG,
        NodeIdx,
        usize,
        Vec<&'a Node<NS, TAG, ATT, VAL, EVENT, MSG>>,
    ),
    /// Append a vector of child nodes to a parent node id.
    AppendChildren(
        &'a TAG,
        NodeIdx,
        Vec<&'a Node<NS, TAG, ATT, VAL, EVENT, MSG>>,
    ),
    /// remove the children with the indices of this node.
    RemoveChildren(&'a TAG, NodeIdx, Vec<usize>),
    /// Replace a node with another node. This typically happens when a node's tag changes.
    /// ex: <div> becomes <span>
    Replace(&'a TAG, NodeIdx, &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>),
    /// Add attributes that the new node has that the old node does not
    /// Note: the attributes is not a reference since attributes of same
    /// name are merged to produce a new unify attribute
    AddAttributes(
        &'a TAG,
        NodeIdx,
        Vec<&'a Attribute<NS, ATT, VAL, EVENT, MSG>>,
    ),
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
            Patch::InsertChildren(_tag, node_idx, _, _) => *node_idx,
            Patch::AppendChildren(_tag, node_idx, _) => *node_idx,
            Patch::RemoveChildren(_tag, node_idx, _) => *node_idx,
            Patch::Replace(_tag, node_idx, _) => *node_idx,
            Patch::AddAttributes(_tag, node_idx, _) => *node_idx,
            Patch::RemoveAttributes(_tag, node_idx, _) => *node_idx,
            Patch::ChangeText(node_idx, _) => *node_idx,
        }
    }

    /// return the tag of this patch
    pub fn tag(&self) -> Option<&TAG> {
        match self {
            Patch::InsertChildren(tag, _node_idx, _, _) => Some(tag),
            Patch::AppendChildren(tag, _node_idx, _) => Some(tag),
            Patch::RemoveChildren(tag, _node_idx, _) => Some(tag),
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
    TAG: PartialEq,
    ATT: PartialEq,
    NS: PartialEq,
    VAL: PartialEq,
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

/// returns true if any of the node children has key in their attributes
fn is_any_children_keyed<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    key: &ATT,
) -> bool
where
    ATT: PartialEq,
{
    element
        .get_children()
        .iter()
        .any(|child| is_keyed_node(child, key))
}

/// returns true any attributes of this node attribute has key in it
fn is_keyed_node<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    node: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    key: &ATT,
) -> bool
where
    ATT: PartialEq,
{
    if let Some(attributes) = node.get_attributes() {
        attributes.iter().any(|att| att.name == *key)
    } else {
        false
    }
}

fn diff_recursive<'a, 'b, NS, TAG, ATT, VAL, EVENT, MSG>(
    old: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    new: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    cur_node_idx: &'b mut usize,
    key: &ATT,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
where
    NS: PartialEq,
    TAG: PartialEq,
    ATT: PartialEq,
    VAL: PartialEq,
{
    let mut patches = vec![];

    // Different enum variants, replace!
    let mut replace = mem::discriminant(old) != mem::discriminant(new);

    if let (Node::Element(old_element), Node::Element(new_element)) = (old, new) {
        // Replace if there are different element tags
        if old_element.tag != new_element.tag {
            replace = true;
        }
    }

    // Handle replacing of a node
    if replace {
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
        return patches;
    }

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
            if is_any_children_keyed(old_element, key) || is_any_children_keyed(new_element, key) {
                println!("keyed comparison..");
                let keyed_patches =
                    diff_keyed_elements(old_element, new_element, key, cur_node_idx);
                patches.extend(keyed_patches);
            } else {
                let non_keyed_patches =
                    diff_non_keyed_elements(old_element, new_element, key, cur_node_idx);
                patches.extend(non_keyed_patches);
            }
        }
        (Node::Text(_), Node::Element(_)) | (Node::Element(_), Node::Text(_)) => {
            unreachable!("Unequal variant discriminants should already have been handled");
        }
    };

    patches
}

/// Reconciliation of keyed elements
///
/// # cases:
///  - A child node is removed at the start
///     - The old key is not on the new node keys anymore
///  - A new child node is inserted at the start of the new element
///     - This node doesn't match to the old node keys
///
/// # not handled
///  - elements that are reorder among their siblings. We only match forward for a straigh-forward algorithmn.
///
///
/// # Finding and matching the old keys
///  - For each new node, iterate through the old element child nodes and
///   match the new key to the old key.
///   If a key is found in the old child nodes, that child_index is take into notice.
///   child nodes that exist before this matching child index will be removed.
///
///  - If no key is matched from the old element children, the new children will be an
///  InsertChild patch
///
fn diff_keyed_elements<'a, 'b, NS, TAG, ATT, VAL, EVENT, MSG>(
    old_element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    new_element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    key: &ATT,
    cur_node_idx: &'b mut usize,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
where
    NS: PartialEq,
    TAG: PartialEq,
    ATT: PartialEq,
    VAL: PartialEq,
{
    let mut patches = vec![];

    let mut matching_keys: Vec<(usize, usize)> = vec![];
    for (new_idx, new_child) in new_element.get_children().iter().enumerate() {
        if let Some(new_child_key) = new_child.get_attribute_value(key) {
            let found_match =
                old_element
                    .get_children()
                    .iter()
                    .enumerate()
                    .find_map(|(old_idx, old_child)| {
                        if let Some(old_child_key) = old_child.get_attribute_value(key) {
                            if old_child_key == new_child_key {
                                Some(old_idx)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    });
            // If the new key_id is matched in the old children key_id,
            // remove the prior siblings at this element, prior to the found old child index.
            if let Some(old_idx) = found_match {
                matching_keys.push((old_idx, new_idx));
            }
        }
    }

    println!("matching keys: {:?}", matching_keys);

    // keep track of what's already included in the InsertChildren patch
    let mut inserted_new_idx = vec![];

    // INSERT the unmatched children
    for (old_idx, old_child) in old_element.get_children().iter().enumerate() {
        // if this old child element is matched, find the new child counter part
        if let Some(matched_new_idx) =
            matching_keys
                .iter()
                .find_map(|(old, new)| if *old == old_idx { Some(new) } else { None })
        {
            println!("processing matched_new_idx: {}", matched_new_idx);

            // but first, all the new_child idx before matched_new_idx will have to be inserted
            //
            // insert the new_child that is not on the matching keys
            // and has a index lesser than the matched_new_idx
            for (new_idx, new_child) in new_element.get_children().iter().enumerate() {
                if !matching_keys.iter().any(|(old, new)| *new == new_idx)
                    && !inserted_new_idx.contains(&new_idx)
                    && new_idx < *matched_new_idx
                {
                    patches.push(Patch::InsertChildren(
                        &old_element.tag,
                        *cur_node_idx,
                        old_idx,
                        vec![new_child],
                    ));
                    inserted_new_idx.push(new_idx);
                }
            }
        } else {
            println!("not matched: {}", old_idx);
            // if this old element was not matched remove it
            patches.push(Patch::RemoveChildren(
                &old_element.tag,
                *cur_node_idx,
                vec![old_idx],
            ));
        }
    }
    println!("here...");

    // APPEND the rest of the new child element that wasn't inserted and wasnt matched
    for (new_idx, new_child) in new_element.get_children().iter().enumerate() {
        println!("processing for append children...");
        if !matching_keys.iter().any(|(old, new)| *new == new_idx)
            && !inserted_new_idx.contains(&new_idx)
        {
            println!("appending children at cur_node_idx: {}", cur_node_idx);
            patches.push(Patch::AppendChildren(
                &old_element.tag,
                *cur_node_idx,
                vec![new_child],
            ));
            inserted_new_idx.push(new_idx);
        }
    }

    // patched the attributes of the matched_new_children
    for (old_idx, old_child) in old_element.get_children().iter().enumerate() {
        if let Some(matched_new_idx) =
            matching_keys
                .iter()
                .find_map(|(old, new)| if *old == old_idx { Some(new) } else { None })
        {
            let matched_new_child = new_element
                .get_children()
                .get(*matched_new_idx)
                .expect("the child must exist");

            let matched_element_patches =
                diff_recursive(old_child, matched_new_child, cur_node_idx, key);
            patches.extend(matched_element_patches);
        }
        *cur_node_idx += 1;
    }
    patches
}

fn diff_non_keyed_elements<'a, 'b, NS, TAG, ATT, VAL, EVENT, MSG>(
    old_element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    new_element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    key: &ATT,
    cur_node_idx: &'b mut usize,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
where
    NS: PartialEq,
    TAG: PartialEq,
    ATT: PartialEq,
    VAL: PartialEq,
{
    let mut patches = vec![];
    let attributes_patches = diff_attributes(old_element, new_element, cur_node_idx);
    patches.extend(attributes_patches);

    let old_child_count = old_element.children.len();
    let new_child_count = new_element.children.len();

    // If there are more new child than old child, we make a patch to append the excess element
    // starting from old_child_count to the last item of the new_elements
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
        patches.push(Patch::RemoveChildren(
            &old_element.tag,
            *cur_node_idx,
            (new_child_count..old_child_count).collect::<Vec<usize>>(),
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
    patches
}

/// diff the attributes of old element to the new element at this cur_node_idx
///
/// Note: The performance bottlenecks
///     - allocating new vec
///     - merging attributes of the same name
fn diff_attributes<'a, 'b, NS, TAG, ATT, VAL, EVENT, MSG>(
    old_element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    new_element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    cur_node_idx: &'b mut usize,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
where
    NS: PartialEq,
    ATT: PartialEq,
    VAL: PartialEq,
{
    let mut patches = vec![];
    let mut add_attributes: Vec<&Attribute<NS, ATT, VAL, EVENT, MSG>> = vec![];
    let mut remove_attributes: Vec<&Attribute<NS, ATT, VAL, EVENT, MSG>> = vec![];

    let new_attributes = new_element.get_attributes();
    let old_attributes = old_element.get_attributes();
    // for all new elements that doesn't exist in the old elements
    // or the values differ
    // add it to the AddAttribute patches
    for new_attr in new_attributes.iter() {
        let old_attr_value = old_attributes
            .iter()
            .find(|att| att.name == new_attr.name)
            .map(|att| &att.value);

        if let Some(old_attr_value) = old_attr_value {
            if *old_attr_value != new_attr.value {
                add_attributes.push(new_attr);
            }
        } else {
            add_attributes.push(new_attr);
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

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG> fmt::Debug for Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Patch::InsertChildren(tag, node_idx, child_index, nodes) => f
                .debug_tuple("InsertChildren")
                .field(tag)
                .field(node_idx)
                .field(child_index)
                .field(nodes)
                .finish(),
            Patch::AppendChildren(tag, node_idx, nodes) => f
                .debug_tuple("AppendChildren")
                .field(tag)
                .field(node_idx)
                .field(nodes)
                .finish(),
            Patch::RemoveChildren(tag, node_idx, child_indices) => f
                .debug_tuple("RemoveChildren")
                .field(tag)
                .field(node_idx)
                .field(child_indices)
                .finish(),
            Patch::Replace(tag, node_idx, node) => f
                .debug_tuple("Replace")
                .field(tag)
                .field(node_idx)
                .field(node)
                .finish(),
            Patch::AddAttributes(tag, node_idx, attrs) => f
                .debug_tuple("AddAttributes")
                .field(tag)
                .field(node_idx)
                .field(attrs)
                .finish(),
            Patch::RemoveAttributes(tag, node_idx, attrs) => f
                .debug_tuple("RemoveAttributes")
                .field(tag)
                .field(node_idx)
                .field(attrs)
                .finish(),

            Patch::ChangeText(node_idx, text) => f
                .debug_tuple("ChangeText")
                .field(node_idx)
                .field(text)
                .finish(),
        }
    }
}
