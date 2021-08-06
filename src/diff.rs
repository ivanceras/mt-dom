//! provides diffing algorithm which returns patches
//!
use crate::{
    node::attribute::group_attributes_per_name,
    patch::{
        AddAttributes, AppendChildren, ChangeText, RemoveAttributes,
        RemoveNode, ReplaceNode,
    },
    Attribute, Element, Node, Patch,
};
use keyed_elements::diff_keyed_elements;
use std::fmt::Debug;
use std::{cmp, fmt, mem};

mod keyed_elements;

/// calculate the difference of 2 nodes
pub fn diff_with_key<'a, NS, TAG, ATT, VAL, EVENT>(
    old_node: &'a Node<NS, TAG, ATT, VAL, EVENT>,
    new_node: &'a Node<NS, TAG, ATT, VAL, EVENT>,
    key: &ATT,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    diff_recursive(
        old_node,
        new_node,
        &mut 0,
        &mut 0,
        key,
        &|_old, _new| false,
        &|_old, _new| false,
    )
}

/// calculate the difference of 2 nodes
/// if the skip function evaluates to true diffinf of
/// the node will be skipped entirely
///
/// The SKIP fn is passed to check whether the diffing of the old and new element should be
/// skipped, and assumed no changes. This is for optimization where the developer is sure that
/// the dom tree hasn't change.
///
/// REP fn stands for replace function which decides if the new element should
/// just replace the old element without diffing
///
pub fn diff_with_functions<'a, NS, TAG, ATT, VAL, EVENT, SKIP, REP>(
    old_node: &'a Node<NS, TAG, ATT, VAL, EVENT>,
    new_node: &'a Node<NS, TAG, ATT, VAL, EVENT>,
    key: &ATT,
    skip: &SKIP,
    rep: &REP,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,

    SKIP: Fn(
        &'a Node<NS, TAG, ATT, VAL, EVENT>,
        &'a Node<NS, TAG, ATT, VAL, EVENT>,
    ) -> bool,
    REP: Fn(
        &'a Node<NS, TAG, ATT, VAL, EVENT>,
        &'a Node<NS, TAG, ATT, VAL, EVENT>,
    ) -> bool,
{
    diff_recursive(old_node, new_node, &mut 0, &mut 0, key, skip, rep)
}

/// increment the cur_node_idx based on how many descendant it contains.
///
/// Note: This is not including the count of itself, since the node is being processed and the cur_node_idx is
/// incremented in the loop together with its siblings
/// TODO: this can be optimize by adding the children count
/// and then descending into element node that has children only
pub fn increment_node_idx_to_descendant_count<NS, TAG, ATT, VAL, EVENT>(
    node: &Node<NS, TAG, ATT, VAL, EVENT>,
    cur_node_idx: &mut usize,
) where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
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
fn is_any_children_keyed<'a, NS, TAG, ATT, VAL, EVENT>(
    element: &'a Element<NS, TAG, ATT, VAL, EVENT>,
    key: &ATT,
) -> bool
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    element
        .get_children()
        .iter()
        .any(|child| is_keyed_node(child, key))
}

/// returns true any attributes of this node attribute has key in it
fn is_keyed_node<'a, NS, TAG, ATT, VAL, EVENT>(
    node: &'a Node<NS, TAG, ATT, VAL, EVENT>,
    key: &ATT,
) -> bool
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    if let Some(attributes) = node.get_attributes() {
        attributes.iter().any(|att| att.name == *key)
    } else {
        false
    }
}

fn diff_recursive<'a, 'b, NS, TAG, ATT, VAL, EVENT, SKIP, REP>(
    old_node: &'a Node<NS, TAG, ATT, VAL, EVENT>,
    new_node: &'a Node<NS, TAG, ATT, VAL, EVENT>,
    cur_node_idx: &'b mut usize,
    new_node_idx: &'b mut usize,
    key: &ATT,
    skip: &SKIP,
    rep: &REP,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
    SKIP: Fn(
        &'a Node<NS, TAG, ATT, VAL, EVENT>,
        &'a Node<NS, TAG, ATT, VAL, EVENT>,
    ) -> bool,
    REP: Fn(
        &'a Node<NS, TAG, ATT, VAL, EVENT>,
        &'a Node<NS, TAG, ATT, VAL, EVENT>,
    ) -> bool,
{
    // skip diffing if the function evaluates to true
    if skip(old_node, new_node) {
        increment_node_idx_to_descendant_count(old_node, cur_node_idx);
        increment_node_idx_to_descendant_count(new_node, new_node_idx);
        return vec![];
    }

    let mut replace =
        mem::discriminant(old_node) != mem::discriminant(new_node);

    let mut patches = vec![];

    // handle explicit replace if the REP fn evaluates to true
    if rep(old_node, new_node) {
        replace = true;
    }

    // replace if the old key does not match the new key
    match (
        old_node.get_attribute_value(&key),
        new_node.get_attribute_value(&key),
    ) {
        (Some(old_key), Some(new_key)) => {
            if old_key != new_key {
                replace = true;
            }
        }
        _ => (),
    }
    // Different enum variants, replace!
    if let (Node::Element(old_element), Node::Element(new_element)) =
        (old_node, new_node)
    {
        // Replace if there are different element tags
        if old_element.tag != new_element.tag {
            replace = true;
        }
    }

    // Handle implicit replace
    if replace {
        patches.push(
            ReplaceNode::new(
                old_node.tag(),
                *cur_node_idx,
                *new_node_idx,
                &new_node,
            )
            .into(),
        );
        increment_node_idx_to_descendant_count(old_node, cur_node_idx);
        increment_node_idx_to_descendant_count(new_node, new_node_idx);
        return patches;
    }

    // The following comparison can only contain identical variants, other
    // cases have already been handled above by comparing variant
    // discriminants.
    match (old_node, new_node) {
        // We're comparing two text nodes
        (Node::Text(old_text), Node::Text(new_text)) => {
            if old_text != new_text {
                let ct = ChangeText::new(
                    *cur_node_idx,
                    old_text,
                    *new_node_idx,
                    new_text,
                );
                patches.push(Patch::ChangeText(ct));
            }
        }

        // We're comparing two element nodes
        (Node::Element(old_element), Node::Element(new_element)) => {
            if is_any_children_keyed(old_element, key)
                || is_any_children_keyed(new_element, key)
            {
                // use diff_keyed_elements if the any of the old_element or new_element
                // wer are comparing contains a key as an attribute
                let keyed_patches = diff_keyed_elements(
                    old_element,
                    new_element,
                    key,
                    cur_node_idx,
                    new_node_idx,
                    skip,
                    rep,
                );
                patches.extend(keyed_patches);
            } else {
                let non_keyed_patches = diff_non_keyed_elements(
                    old_element,
                    new_element,
                    key,
                    cur_node_idx,
                    new_node_idx,
                    skip,
                    rep,
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
fn diff_non_keyed_elements<'a, 'b, NS, TAG, ATT, VAL, EVENT, SKIP, REP>(
    old_element: &'a Element<NS, TAG, ATT, VAL, EVENT>,
    new_element: &'a Element<NS, TAG, ATT, VAL, EVENT>,
    key: &ATT,
    cur_node_idx: &'b mut usize,
    new_node_idx: &'b mut usize,
    skip: &SKIP,
    rep: &REP,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
    SKIP: Fn(
        &'a Node<NS, TAG, ATT, VAL, EVENT>,
        &'a Node<NS, TAG, ATT, VAL, EVENT>,
    ) -> bool,
    REP: Fn(
        &'a Node<NS, TAG, ATT, VAL, EVENT>,
        &'a Node<NS, TAG, ATT, VAL, EVENT>,
    ) -> bool,
{
    let this_cur_node_idx = *cur_node_idx;
    let mut patches = vec![];
    let attributes_patches =
        diff_attributes(old_element, new_element, cur_node_idx, new_node_idx);
    patches.extend(attributes_patches);

    let old_child_count = old_element.children.len();
    let new_child_count = new_element.children.len();

    let min_count = cmp::min(old_child_count, new_child_count);
    for index in 0..min_count {
        *cur_node_idx += 1;
        *new_node_idx += 1;

        let old_child = &old_element
            .children
            .get(index)
            .expect("No old_node child node");
        let new_child =
            &new_element.children.get(index).expect("No new chold node");

        let more_patches = diff_recursive(
            old_child,
            new_child,
            cur_node_idx,
            new_node_idx,
            key,
            skip,
            rep,
        );
        patches.extend(more_patches);
        //increment_node_idx_to_descendant_count(new_child, new_node_idx);
    }

    // If there are more new child than old_node child, we make a patch to append the excess element
    // starting from old_child_count to the last item of the new_elements
    if new_child_count > old_child_count {
        let mut append_patch: Vec<(usize, &'a Node<NS, TAG, ATT, VAL, EVENT>)> =
            vec![];

        for append_child in new_element.children.iter().skip(old_child_count) {
            *new_node_idx += 1;
            append_patch.push((*new_node_idx, append_child));
            increment_node_idx_to_descendant_count(append_child, new_node_idx);
        }

        patches.push(
            AppendChildren::new(
                &old_element.tag,
                this_cur_node_idx,
                append_patch,
            )
            .into(),
        )
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

/// diff the attributes of old_node element to the new element at this cur_node_idx
///
/// Note: The performance bottlenecks
///     - allocating new vec
///     - merging attributes of the same name
fn diff_attributes<'a, 'b, NS, TAG, ATT, VAL, EVENT>(
    old_element: &'a Element<NS, TAG, ATT, VAL, EVENT>,
    new_element: &'a Element<NS, TAG, ATT, VAL, EVENT>,
    cur_node_idx: &'b mut usize,
    new_node_idx: &'b mut usize,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    let mut patches = vec![];
    let mut add_attributes: Vec<&Attribute<NS, ATT, VAL, EVENT>> = vec![];
    let mut remove_attributes: Vec<&Attribute<NS, ATT, VAL, EVENT>> = vec![];

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
            AddAttributes::new(
                &old_element.tag,
                *cur_node_idx,
                *new_node_idx,
                add_attributes,
            )
            .into(),
        );
    }
    if !remove_attributes.is_empty() {
        patches.push(
            RemoveAttributes::new(
                &old_element.tag,
                *cur_node_idx,
                *new_node_idx,
                remove_attributes,
            )
            .into(),
        );
    }
    patches
}
