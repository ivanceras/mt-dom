use super::{
    diff_attributes,
    diff_recursive,
    increment_node_idx_to_descendant_count,
};
use crate::{
    patch::{
        AppendChildren,
        InsertNode,
        RemoveNode,
    },
    Element,
    Node,
    Patch,
};
use std::{
    collections::BTreeMap,
    fmt,
    iter::FromIterator,
};

/// find the element and its node_idx which has this key
/// and its node_idx not in `not_in`
fn find_node_with_key<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    hay_stack: &BTreeMap<
        usize,
        (Vec<&'a VAL>, &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>),
    >,
    find_key: &Vec<&'a VAL>,
    last_matched_node_idx: Option<usize>,
) -> Option<(usize, &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>)>
where
    NS: PartialEq + fmt::Debug,
    TAG: PartialEq + fmt::Debug,
    ATT: PartialEq + fmt::Debug,
    VAL: PartialEq + fmt::Debug,
{
    hay_stack.iter().find_map(|(node_idx, (key, node))| {
        // also check if it hasn't been already matched
        // and also check if node_idx is greater than last_matched_node_idx
        if key == find_key {
            let last_matched_node_idx_val = last_matched_node_idx.unwrap_or(0);
            if last_matched_node_idx.is_none()
                || *node_idx > last_matched_node_idx_val
            {
                Some((*node_idx, *node))
            } else {
                None
            }
        } else {
            None
        }
    })
}

/// find the new_child which matched this old_idx
fn find_matched_new_child<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    matched_old_new_keyed: &BTreeMap<
        (usize, usize),
        (
            &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
            &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
        ),
    >,
    find_old_idx: usize,
) -> Option<(usize, &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>)> {
    matched_old_new_keyed.iter().find_map(
        |((old_idx, new_idx), (_, new_child))| {
            if *old_idx == find_old_idx {
                Some((*new_idx, *new_child))
            } else {
                None
            }
        },
    )
}

/// find the old_child which has this old_idx
fn find_child_node_with_idx<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    haystack: &Vec<(usize, &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>)>,
    node_idx: usize,
) -> Option<(usize, &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>)> {
    haystack.iter().find_map(|(idx, node)| {
        if *idx == node_idx {
            Some((*idx, *node))
        } else {
            None
        }
    })
}

/// return a the matched (Vec<old_idx>, Vec<new_idx>)
fn get_matched_old_new_idx<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    matched_old_new_keyed: &BTreeMap<
        (usize, usize),
        (
            &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
            &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
        ),
    >,
) -> (Vec<usize>, Vec<usize>) {
    matched_old_new_keyed
        .iter()
        .map(|((old_idx, new_idx), _)| (old_idx, new_idx))
        .unzip()
}

/// return the node_idx and node where it's node idx is not in the arg `matched_id`
fn get_unmatched_children_node_idx<'a, NS, TAG, ATT, VAL, EVENT, MSG>(
    child_nodes: &'a [Node<NS, TAG, ATT, VAL, EVENT, MSG>],
    matched_idx: Vec<usize>,
) -> Vec<(usize, &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>)> {
    child_nodes
        .iter()
        .enumerate()
        .filter(|(idx, _node)| !matched_idx.contains(&idx))
        .collect()
}

/// Reconciliation of keyed elements
///
/// algorithm:
///
/// Phase1
///   - prioritize matching elements that has the same key.
///   - match non-keyed elements according to their node_idx alignment
///
/// Phase2
///   - old child elements that are not matched will be removed
///   - new child elements that are not matched will be inserted or appended
///     - inserted if the child node_idx <= old elements children
///     - appended if the child node_idx is > old elements children
///
pub fn diff_keyed_elements<'a, 'b, NS, TAG, ATT, VAL, EVENT, MSG, SKIP>(
    old_element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    new_element: &'a Element<NS, TAG, ATT, VAL, EVENT, MSG>,
    key: &ATT,
    cur_node_idx: &'b mut usize,
    skip: &SKIP,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
where
    NS: PartialEq + fmt::Debug,
    TAG: PartialEq + fmt::Debug,
    ATT: PartialEq + fmt::Debug,
    VAL: PartialEq + fmt::Debug,
    SKIP: Fn(
        &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
        &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    ) -> bool,
{
    let mut patches = vec![];

    let attributes_patches =
        diff_attributes(old_element, new_element, cur_node_idx);
    // create a map for both the old and new element
    // we can not use VAL as the key, since it is not Hash
    let old_keyed_elements: BTreeMap<
        usize,
        (Vec<&VAL>, &Node<NS, TAG, ATT, VAL, EVENT, MSG>),
    > = BTreeMap::from_iter(
        old_element.get_children().iter().enumerate().filter_map(
            |(old_idx, old_child)| {
                if let Some(old_key) = old_child.get_attribute_value(key) {
                    Some((old_idx, (old_key, old_child)))
                } else {
                    None
                }
            },
        ),
    );

    // a map for new_keyed elements for quick lookup matching to the old_keyed_elements
    let new_keyed_elements: BTreeMap<
        usize,
        (Vec<&VAL>, &Node<NS, TAG, ATT, VAL, EVENT, MSG>),
    > = BTreeMap::from_iter(
        new_element.get_children().iter().enumerate().filter_map(
            |(new_idx, new_child)| {
                if let Some(new_key) = new_child.get_attribute_value(key) {
                    Some((new_idx, (new_key, new_child)))
                } else {
                    None
                }
            },
        ),
    );

    // compiles that matched old and new with
    // with their (old_idx, new_idx) as key and the value is (old_element, new_element)
    let mut matched_old_new_keyed: BTreeMap<
        (usize, usize),
        (
            &Node<NS, TAG, ATT, VAL, EVENT, MSG>,
            &Node<NS, TAG, ATT, VAL, EVENT, MSG>,
        ),
    > = BTreeMap::new();

    let mut last_matched_old_idx = None;
    let mut last_matched_new_idx = None;
    // here, we need to processed both keyed element and non-keyed elements
    for (new_idx, (new_key, new_element)) in new_keyed_elements.iter() {
        if let Some((old_idx, old_element)) = find_node_with_key(
            &old_keyed_elements,
            new_key,
            last_matched_old_idx,
        ) {
            let last_matched_new_idx_val = last_matched_new_idx.unwrap_or(0);
            // matching should be always on forward direction
            if last_matched_new_idx.is_none()
                || *new_idx > last_matched_new_idx_val
            {
                last_matched_old_idx = Some(old_idx);
                last_matched_new_idx = Some(*new_idx);
                matched_old_new_keyed
                    .insert((old_idx, *new_idx), (old_element, new_element));
            } else {
                // we don't matched already passed elements
            }
        }
    }

    // unmatched old and idx in pass 1
    let (matched_old_idx, matched_new_idx) =
        get_matched_old_new_idx(&matched_old_new_keyed);
    // these are the new children that didn't matched in the keyed elements pass
    let unmatched_new_child = get_unmatched_children_node_idx(
        new_element.get_children(),
        matched_new_idx,
    );

    // thse are the old children that didn't matched in the keyed elements pass
    let unmatched_old_child = get_unmatched_children_node_idx(
        old_element.get_children(),
        matched_old_idx,
    );

    // matched old and new element, not necessarily keyed
    let matched_old_new: BTreeMap<
        (usize, usize),
        (
            &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
            &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
        ),
    > = BTreeMap::from_iter(
        // try to match unmatched new child from the unmatched old child
        // using the their idx, most likely aligned nodes (ie: old and new node in the same index)
        unmatched_new_child
            .iter()
            .filter_map(|(new_idx, new_child)| {
                if let Some((old_idx, old_child)) =
                    find_child_node_with_idx(&unmatched_old_child, *new_idx)
                {
                    Some(((old_idx, *new_idx), (old_child, *new_child)))
                } else {
                    None
                }
            }),
    );

    // merge both
    // TODO: rename `matched_old_new_keyed` to `matched_old_new_all`
    // since it both contains keyed and the not-necessarily keyed
    // Note: not-necessarily keyed means an element could be keyed
    // but not matched using the key value, therefore will be tried
    // to be match to pass2 which only checks for aligned idx to match each other.
    matched_old_new_keyed.extend(matched_old_new);

    // unmatched old and idx in pass 2
    let (_matched_old_idx_pass2, matched_new_idx_pass2) =
        get_matched_old_new_idx(&matched_old_new_keyed);

    // this elements are for inserting, appending
    let unmatched_new_child_pass2 = get_unmatched_children_node_idx(
        new_element.get_children(),
        matched_new_idx_pass2,
    );

    // process this last so as not to move the cur_node_idx forward
    // and without creating a snapshot for cur_node_idx for other patch types
    let mut matched_keyed_element_patches = vec![];
    let mut remove_node_patches = vec![];
    let mut insert_node_patches = vec![];

    // keeps track of new_idx that is part of the InsertNode
    let mut already_inserted = vec![];

    let new_child_excess_cur_node_idx = *cur_node_idx;

    for (old_idx, old_child) in old_element.children.iter().enumerate() {
        *cur_node_idx += 1;
        if let Some((new_idx, new_child)) =
            find_matched_new_child(&matched_old_new_keyed, old_idx)
        {
            // insert unmatched new_child that is less than the matched new_idx
            for (idx, unmatched) in unmatched_new_child_pass2
                .iter()
                .filter(|(idx, _)| *idx < new_idx)
            {
                if !already_inserted.contains(idx) {
                    insert_node_patches.push(
                        InsertNode::new(
                            Some(&old_element.tag),
                            *cur_node_idx,
                            unmatched,
                        )
                        .into(),
                    );
                    already_inserted.push(*idx);
                }
            }

            matched_keyed_element_patches.extend(diff_recursive(
                old_child,
                new_child,
                cur_node_idx,
                key,
                skip,
            ));
        } else {
            remove_node_patches
                .push(RemoveNode::new(old_child.tag(), *cur_node_idx).into());
            increment_node_idx_to_descendant_count(old_child, cur_node_idx);
        }
    }

    // the node that are to be appended are nodes
    // that are not matched, and not already part of the InsertNode
    let append_children_patches = unmatched_new_child_pass2
        .iter()
        .filter(|(new_idx, _)| !already_inserted.contains(&new_idx))
        .map(|(_, new_child)| {
            AppendChildren::new(
                &old_element.tag,
                new_child_excess_cur_node_idx,
                vec![new_child],
            )
            .into()
        })
        .collect::<Vec<_>>();

    // patch order matters here
    // apply changes to the matched element first,
    // since it creates new changes to the child index nodes
    patches.extend(attributes_patches);
    patches.extend(matched_keyed_element_patches);
    patches.extend(insert_node_patches);
    patches.extend(append_children_patches);
    patches.extend(remove_node_patches);
    patches
}
