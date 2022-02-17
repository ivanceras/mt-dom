#![allow(clippy::type_complexity)]
use super::{create_attribute_patches, diff_recursive};
use crate::{Element, Node, Patch, TreePath};
use std::collections::BTreeMap;
use std::fmt::Debug;

/// find the element and its node_idx which has this key
/// and its node_idx not in `not_in`
fn find_node_with_key<'a, NS, TAG, ATT, VAL>(
    hay_stack: &BTreeMap<usize, (Vec<&'a VAL>, &'a Node<NS, TAG, ATT, VAL>)>,
    find_key: &[&'a VAL],
    last_matched_node_idx: Option<usize>,
) -> Option<(usize, &'a Node<NS, TAG, ATT, VAL>)>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
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
fn find_matched_new_child<'a, NS, TAG, ATT, VAL>(
    matched_old_new_keyed: &BTreeMap<
        (usize, usize),
        (&'a Node<NS, TAG, ATT, VAL>, &'a Node<NS, TAG, ATT, VAL>),
    >,
    find_old_idx: usize,
) -> Option<(usize, &'a Node<NS, TAG, ATT, VAL>)>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
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
fn find_child_node_with_idx<'a, NS, TAG, ATT, VAL>(
    haystack: &[(usize, &'a Node<NS, TAG, ATT, VAL>)],
    node_idx: usize,
) -> Option<(usize, &'a Node<NS, TAG, ATT, VAL>)>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    haystack.iter().find_map(|(idx, node)| {
        if *idx == node_idx {
            Some((*idx, *node))
        } else {
            None
        }
    })
}

/// return a the matched (Vec<old_idx>, Vec<new_idx>)
fn get_matched_old_new_idx<'a, NS, TAG, ATT, VAL>(
    matched_old_new_keyed: &BTreeMap<
        (usize, usize),
        (&'a Node<NS, TAG, ATT, VAL>, &'a Node<NS, TAG, ATT, VAL>),
    >,
) -> (Vec<usize>, Vec<usize>)
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    matched_old_new_keyed
        .iter()
        .map(|((old_idx, new_idx), _)| (old_idx, new_idx))
        .unzip()
}

/// return the node_idx and node where it's node idx is not in the arg `matched_id`
fn get_unmatched_children_node_idx<NS, TAG, ATT, VAL>(
    child_nodes: &[Node<NS, TAG, ATT, VAL>],
    matched_idx: Vec<usize>,
) -> Vec<(usize, &Node<NS, TAG, ATT, VAL>)>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    child_nodes
        .iter()
        .enumerate()
        .filter(|(idx, _node)| !matched_idx.contains(idx))
        .collect()
}

fn build_keyed_elements<'a, NS, TAG, ATT, VAL>(
    element: &'a Element<NS, TAG, ATT, VAL>,
    key: &ATT,
) -> BTreeMap<usize, (Vec<&'a VAL>, &'a Node<NS, TAG, ATT, VAL>)>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    element
        .get_children()
        .iter()
        .enumerate()
        .filter_map(|(idx, child)| {
            child
                .get_attribute_value(key)
                .map(|child_key| (idx, (child_key, child)))
        })
        .collect()
}

fn build_matched_old_new_keyed<'a, NS, TAG, ATT, VAL>(
    old_keyed_elements: &BTreeMap<
        usize,
        (Vec<&'a VAL>, &'a Node<NS, TAG, ATT, VAL>),
    >,
    new_keyed_elements: &BTreeMap<
        usize,
        (Vec<&'a VAL>, &'a Node<NS, TAG, ATT, VAL>),
    >,
) -> BTreeMap<
    (usize, usize),
    (&'a Node<NS, TAG, ATT, VAL>, &'a Node<NS, TAG, ATT, VAL>),
>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    let mut matched_old_new_keyed = BTreeMap::new();
    let mut last_matched_old_idx = None;
    let mut last_matched_new_idx = None;
    // here, we need to processed both keyed element and non-keyed elements
    for (new_idx, (new_key, new_element)) in new_keyed_elements.iter() {
        // find the old element which has a key value which is the same with the new element key
        // `new_key`
        if let Some((old_idx, old_element)) = find_node_with_key(
            old_keyed_elements,
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
                    .insert((old_idx, *new_idx), (old_element, *new_element));
            } else {
                // we don't matched already passed elements
            }
        }
    }
    matched_old_new_keyed
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
pub fn diff_keyed_elements<'a, 'b, NS, TAG, ATT, VAL, SKIP, REP>(
    old_element: &'a Element<NS, TAG, ATT, VAL>,
    new_element: &'a Element<NS, TAG, ATT, VAL>,
    key: &ATT,
    path: &[usize],
    skip: &SKIP,
    rep: &REP,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    SKIP: Fn(&'a Node<NS, TAG, ATT, VAL>, &'a Node<NS, TAG, ATT, VAL>) -> bool,
    REP: Fn(&'a Node<NS, TAG, ATT, VAL>, &'a Node<NS, TAG, ATT, VAL>) -> bool,
{
    let mut patches = vec![];

    // create a map for both the old and new element
    // we can not use VAL as the key, since it is not Hash
    let old_keyed_elements = build_keyed_elements(old_element, key);
    let new_keyed_elements = build_keyed_elements(new_element, key);

    // compiles that matched old and new with
    // with their (old_idx, new_idx) as key and the value is (old_element, new_element)
    let matched_old_new_keyed =
        build_matched_old_new_keyed(&old_keyed_elements, &new_keyed_elements);

    // unmatched old and idx in pass 1
    let (matched_old_idx, matched_new_idx) =
        get_matched_old_new_idx(&matched_old_new_keyed);

    // these are the new children that didn't matched in the keyed elements pass
    let mut unmatched_new_child: Vec<(usize, &'a Node<NS, TAG, ATT, VAL>)> =
        vec![];

    for (idx, new_element) in new_element.get_children().iter().enumerate() {
        if !matched_new_idx.contains(&idx) {
            unmatched_new_child.push((idx, new_element));
        }
    }

    // these are the old children that didn't matched in the keyed elements pass
    let unmatched_old_child = get_unmatched_children_node_idx(
        old_element.get_children(),
        matched_old_idx,
    );

    // matched old and new element, not necessarily keyed
    let matched_old_new: BTreeMap<
        (usize, usize),
        (&'a Node<NS, TAG, ATT, VAL>, &'a Node<NS, TAG, ATT, VAL>),
    > =
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
            }).collect();

    // This is named all_matched_elements
    // since it both contains keyed and the not-necessarily keyed
    // Note: not-necessarily keyed means an element could be keyed
    // but not matched using the key value, therefore will be tried
    // to be match to pass2 which only checks for aligned idx to match each other.
    let all_matched_elements = {
        let mut tmp = matched_old_new_keyed.clone();
        tmp.extend(matched_old_new);
        tmp
    };

    // unmatched old and idx in pass 2
    let (_matched_old_idx_pass2, matched_new_idx_pass2) =
        get_matched_old_new_idx(&all_matched_elements);

    // this elements are for inserting, appending
    let mut unmatched_new_child_pass2: Vec<(
        usize,
        usize,
        &Node<NS, TAG, ATT, VAL>,
    )> = vec![];
    for (idx, new_child) in new_element.get_children().iter().enumerate() {
        if !matched_new_idx_pass2.contains(&idx) {
            unmatched_new_child_pass2.push((idx, idx, new_child));
        }
    }

    let mut matched_keyed_element_patches = vec![];
    let mut remove_node_patches = vec![];
    let mut insert_node_patches = vec![];

    // keeps track of new_idx that is part of the InsertNode
    let mut already_inserted = vec![];

    for (old_idx, old_child) in old_element.children.iter().enumerate() {
        let mut child_path = path.to_vec();
        child_path.push(old_idx);

        if let Some((new_idx, new_child)) =
            find_matched_new_child(&all_matched_elements, old_idx)
        {
            // insert unmatched new_child that is less than the matched new_idx
            insert_node_patches.extend(create_insert_node_patches(
                old_element,
                &mut already_inserted,
                &unmatched_new_child_pass2,
                new_idx,
                &child_path,
            ));

            matched_keyed_element_patches.extend(diff_recursive(
                old_child,
                new_child,
                &child_path,
                key,
                skip,
                rep,
            ));
        } else {
            remove_node_patches.push(Patch::remove_node(
                old_child.tag(),
                TreePath::new(child_path.to_vec()),
            ));
        }
    }

    // the node that are to be appended are nodes
    // that are not matched, and not already part of the InsertNode
    let append_children_patches = create_append_children_patches(
        old_element,
        &mut already_inserted,
        &unmatched_new_child_pass2,
        path,
    );

    let attributes_patches =
        create_attribute_patches(old_element, new_element, path);

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

fn create_insert_node_patches<'a, NS, TAG, ATT, VAL>(
    old_element: &'a Element<NS, TAG, ATT, VAL>,
    already_inserted: &mut Vec<usize>,
    unmatched_new_child_pass2: &[(usize, usize, &'a Node<NS, TAG, ATT, VAL>)],
    new_idx: usize,
    child_path: &[usize],
) -> Vec<Patch<'a, NS, TAG, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    let mut insert_node_patches = vec![];
    for (idx, _new_element_node_idx, unmatched) in unmatched_new_child_pass2
        .iter()
        .filter(|(idx, _, _)| *idx < new_idx)
    {
        if !already_inserted.contains(idx) {
            insert_node_patches.push(Patch::insert_node(
                Some(&old_element.tag),
                TreePath::new(child_path.to_vec()),
                unmatched,
            ));
            already_inserted.push(*idx);
        }
    }

    insert_node_patches
}

fn create_append_children_patches<'a, NS, TAG, ATT, VAL>(
    old_element: &'a Element<NS, TAG, ATT, VAL>,
    already_inserted: &mut Vec<usize>,
    unmatched_new_child_pass2: &[(usize, usize, &'a Node<NS, TAG, ATT, VAL>)],
    path: &[usize],
) -> Vec<Patch<'a, NS, TAG, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    let mut append_children_patches = vec![];
    for (new_idx, _new_element_node_idx, new_child) in
        unmatched_new_child_pass2.iter()
    {
        if !already_inserted.contains(new_idx) {
            append_children_patches.push(Patch::append_children(
                &old_element.tag,
                TreePath::new(path.to_vec()),
                vec![new_child],
            ));
            already_inserted.push(*new_idx);
        }
    }
    append_children_patches
}
