//! provides diffing algorithm which returns patches
//!
use crate::{
    node::attribute::group_attributes_per_name,
    patch::{
        AddAttributes,
        AppendChildren,
        ChangeText,
        InsertChildren,
        RemoveAttributes,
        RemoveNode,
        ReplaceNode,
    },
    Attribute,
    Element,
    Node,
    Patch,
};
use keyed_elements::diff_keyed_elements;
use std::{
    cmp,
    collections::BTreeMap,
    fmt,
    hash::Hash,
    iter::FromIterator,
    mem,
};

mod keyed_elements;

/// calculate the difference of 2 nodes
/// the supplied key will be taken into account
/// that if the 2 keys differ, the element will be replaced without having to traverse the children
/// nodes
pub fn diff_with_key<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    old: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    new: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    key: &ATT,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
where
    TAG: PartialEq + fmt::Debug,
    ATT: PartialEq + fmt::Debug,
    NS: PartialEq + fmt::Debug,
    VAL: PartialEq + fmt::Debug,
{
    diff_recursive(old, new, &mut 0, key)
}

/// increment the cur_node_idx based on how many descendant it contains.
///
/// Note: This is not including the count of itself, since the node is being processed and the cur_node_idx is
/// incremented in the loop together with its siblings
pub(crate) fn increment_node_idx_to_descendant_count<
    NS,
    TAG,
    ATT,
    VAL,
    EVENT,
    MSG,
>(
    node: &Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    cur_node_idx: &mut usize,
) {
    match node {
        Node::Element(element_node) => {
            for child in element_node.get_children().iter() {
                *cur_node_idx += 1;
                increment_node_idx_to_descendant_count(&child, cur_node_idx);
            }
        }
        Node::Text(_txt) => {
            // as is
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
    NS: PartialEq + fmt::Debug,
    TAG: PartialEq + fmt::Debug,
    ATT: PartialEq + fmt::Debug,
    VAL: PartialEq + fmt::Debug,
{
    //log::trace!("entering diff_recursive at cur_node_idx: {}", cur_node_idx);
    let mut patches = vec![];
    // Different enum variants, replace!
    let mut replace = mem::discriminant(old) != mem::discriminant(new);

    if let (Node::Element(old_element), Node::Element(new_element)) = (old, new)
    {
        // Replace if there are different element tags
        if old_element.tag != new_element.tag {
            replace = true;
        }
    }

    // Handle replacing of a node
    if replace {
        patches.push(
            ReplaceNode::new(
                old.tag().expect("must have a tag"),
                *cur_node_idx,
                &new,
            )
            .into(),
        );
        increment_node_idx_to_descendant_count(old, cur_node_idx);
        return patches;
    }

    // The following comparison can only contain identical variants, other
    // cases have already been handled above by comparing variant
    // discriminants.
    match (old, new) {
        // We're comparing two text nodes
        (Node::Text(old_text), Node::Text(new_text)) => {
            if old_text != new_text {
                let ct = ChangeText::new(*cur_node_idx, old_text, new_text);
                patches.push(Patch::ChangeText(ct));
            }
        }

        // We're comparing two element nodes
        (Node::Element(old_element), Node::Element(new_element)) => {
            if is_any_children_keyed(old_element, key)
                || is_any_children_keyed(new_element, key)
            {
                let keyed_patches = diff_keyed_elements(
                    old_element,
                    new_element,
                    key,
                    cur_node_idx,
                );
                patches.extend(keyed_patches);
            } else {
                let non_keyed_patches = diff_non_keyed_elements(
                    old_element,
                    new_element,
                    key,
                    cur_node_idx,
                );
                patches.extend(non_keyed_patches);
            }
        }
        (Node::Text(_), Node::Element(_))
        | (Node::Element(_), Node::Text(_)) => {
            unreachable!("Unequal variant discriminants should already have been handled");
        }
    };

    patches
}

/// In diffing non_keyed elements,
///  we reuse existing DOM elements as much as possible
///
///  The algorithm used here is very simple.
///
///  If there are more children in the old_element than the new_element
///  the excess children is all removed.
///
///  If there are more children in the new_element than the old_element
///  it will be all appended in the old_element.
///
///
fn diff_non_keyed_elements<'a, 'b, NS, TAG, ATT, VAL, EVENT, MSG>(
    old_element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    new_element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    key: &ATT,
    cur_node_idx: &'b mut usize,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
where
    NS: PartialEq + fmt::Debug,
    TAG: PartialEq + fmt::Debug,
    ATT: PartialEq + fmt::Debug,
    VAL: PartialEq + fmt::Debug,
{
    //log::trace!(
    //    "entering diff_non_keyed_elements at cur_node_idx: {}",
    //    cur_node_idx
    //);
    let this_cur_node_idx = *cur_node_idx;

    let mut patches = vec![];
    let attributes_patches =
        diff_attributes(old_element, new_element, cur_node_idx);
    patches.extend(attributes_patches);

    let old_child_count = old_element.children.len();
    let new_child_count = new_element.children.len();

    // If there are more new child than old child, we make a patch to append the excess element
    // starting from old_child_count to the last item of the new_elements
    if new_child_count > old_child_count {
        let append_patch: Vec<&'a Node<NS, TAG, ATT, VAL, EVENT, MSG>> =
            new_element.children[old_child_count..].iter().collect();

        patches.push(
            AppendChildren::new(&old_element.tag, *cur_node_idx, append_patch)
                .into(),
        )
    }

    let min_count = cmp::min(old_child_count, new_child_count);
    for index in 0..min_count {
        *cur_node_idx += 1;

        let old_child =
            &old_element.children.get(index).expect("No old child node");
        let new_child =
            &new_element.children.get(index).expect("No new chold node");

        let more_patches =
            diff_recursive(old_child, new_child, cur_node_idx, key);
        patches.extend(more_patches);
    }

    if new_child_count < old_child_count {
        for old_child in old_element.get_children().iter().skip(new_child_count)
        {
            *cur_node_idx += 1;
            patches
                .push(RemoveNode::new(old_child.tag(), *cur_node_idx).into());
            increment_node_idx_to_descendant_count(old_child, cur_node_idx);
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
    NS: PartialEq + fmt::Debug,
    ATT: PartialEq + fmt::Debug,
    VAL: PartialEq + fmt::Debug,
{
    let mut patches = vec![];
    let mut add_attributes: Vec<&Attribute<NS, ATT, VAL, EVENT, MSG>> = vec![];
    let mut remove_attributes: Vec<&Attribute<NS, ATT, VAL, EVENT, MSG>> =
        vec![];

    let new_attributes_grouped =
        group_attributes_per_name(new_element.get_attributes());
    let old_attributes_grouped =
        group_attributes_per_name(old_element.get_attributes());
    // for all new elements that doesn't exist in the old elements
    // or the values differ
    // add it to the AddAttribute patches
    for (new_attr_name, new_attrs) in new_attributes_grouped.iter() {
        // Issue: only the first found attribute's value is returned
        // This could be problematic if there are multiple attributes of the same name
        let old_attr_values = old_attributes_grouped
            .iter()
            .find(|(att_name, _)| att_name == new_attr_name)
            .map(|(_, attrs)| {
                attrs.iter().map(|attr| &attr.value).collect::<Vec<_>>()
            });

        let new_attr_values = new_attributes_grouped
            .iter()
            .find(|(att_name, _)| att_name == new_attr_name)
            .map(|(_, attrs)| {
                attrs.iter().map(|attr| &attr.value).collect::<Vec<_>>()
            });

        if let Some(old_attr_values) = old_attr_values {
            let new_attr_values =
                new_attr_values.expect("must have new attr values");
            if old_attr_values != new_attr_values {
                add_attributes.extend(new_attrs);
            }
        } else {
            add_attributes.extend(new_attrs);
        }
    }

    // if this attribute name does not exist anymore
    // to the new element, remove it
    for (old_attr_name, old_attrs) in old_attributes_grouped.iter() {
        if let Some(_pre_attr) = new_attributes_grouped
            .iter()
            .find(|(new_attr_name, _)| new_attr_name == old_attr_name)
        {
            //
        } else {
            remove_attributes.extend(old_attrs);
        }
    }

    if !add_attributes.is_empty() {
        patches.push(
            AddAttributes::new(&old_element.tag, *cur_node_idx, add_attributes)
                .into(),
        );
    }
    if !remove_attributes.is_empty() {
        patches.push(
            RemoveAttributes::new(
                &old_element.tag,
                *cur_node_idx,
                remove_attributes,
            )
            .into(),
        );
    }
    patches
}
