use super::{
    diff_attributes, diff_recursive, increment_node_idx_to_descendant_count,
};
use crate::{
    patch::{AppendChildren, InsertNode, RemoveNode},
    Element, Node, NodeIdx, Patch, PatchPath, TreePath,
};
use std::fmt::Debug;
use std::{collections::BTreeMap, iter::FromIterator};

/// find the element and its node_idx which has this key
/// and its node_idx not in `not_in`
fn find_node_with_key<'a, NS, TAG, ATT, VAL, EVENT>(
    hay_stack: &BTreeMap<
        usize,
        (Vec<&'a VAL>, &'a Node<NS, TAG, ATT, VAL, EVENT>),
    >,
    find_key: &Vec<&'a VAL>,
    last_matched_node_idx: Option<usize>,
) -> Option<(usize, &'a Node<NS, TAG, ATT, VAL, EVENT>)>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
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
fn find_matched_new_child<'a, NS, TAG, ATT, VAL, EVENT>(
    matched_old_new_keyed: &BTreeMap<
        (usize, usize),
        (
            &'a Node<NS, TAG, ATT, VAL, EVENT>,
            (NodeIdx, &'a Node<NS, TAG, ATT, VAL, EVENT>),
        ),
    >,
    find_old_idx: usize,
) -> Option<(usize, (NodeIdx, &'a Node<NS, TAG, ATT, VAL, EVENT>))>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
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
fn find_child_node_with_idx<'a, NS, TAG, ATT, VAL, EVENT>(
    haystack: &Vec<(usize, &'a Node<NS, TAG, ATT, VAL, EVENT>)>,
    node_idx: usize,
) -> Option<(usize, &'a Node<NS, TAG, ATT, VAL, EVENT>)>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
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
fn get_matched_old_new_idx<'a, NS, TAG, ATT, VAL, EVENT>(
    matched_old_new_keyed: &BTreeMap<
        (usize, usize),
        (
            &'a Node<NS, TAG, ATT, VAL, EVENT>,
            (NodeIdx, &'a Node<NS, TAG, ATT, VAL, EVENT>),
        ),
    >,
) -> (Vec<usize>, Vec<usize>)
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    matched_old_new_keyed
        .iter()
        .map(|((old_idx, new_idx), _)| (old_idx, new_idx))
        .unzip()
}

/// return the node_idx and node where it's node idx is not in the arg `matched_id`
fn get_unmatched_children_node_idx<'a, NS, TAG, ATT, VAL, EVENT>(
    child_nodes: &'a [Node<NS, TAG, ATT, VAL, EVENT>],
    matched_idx: Vec<usize>,
) -> Vec<(usize, &'a Node<NS, TAG, ATT, VAL, EVENT>)>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
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
pub fn diff_keyed_elements<'a, 'b, NS, TAG, ATT, VAL, EVENT, SKIP, REP>(
    old_element: &'a Element<NS, TAG, ATT, VAL, EVENT>,
    new_element: &'a Element<NS, TAG, ATT, VAL, EVENT>,
    key: &ATT,
    cur_node_idx: &'b mut usize,
    new_node_idx: &'b mut usize,
    cur_path: &Vec<usize>,
    new_path: &Vec<usize>,
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
    let mut patches = vec![];

    let attributes_patches = diff_attributes(
        old_element,
        new_element,
        cur_node_idx,
        new_node_idx,
        cur_path,
        new_path,
    );
    // create a map for both the old and new element
    // we can not use VAL as the key, since it is not Hash
    let old_keyed_elements: BTreeMap<
        usize,
        (Vec<&VAL>, &Node<NS, TAG, ATT, VAL, EVENT>),
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

    let mut node_idx_new_elements: BTreeMap<
        NodeIdx,
        &Node<NS, TAG, ATT, VAL, EVENT>,
    > = BTreeMap::new();

    for new_child in new_element.get_children().iter() {
        *new_node_idx += 1;
        node_idx_new_elements.insert(*new_node_idx, new_child);
        increment_node_idx_to_descendant_count(new_child, new_node_idx);
    }

    let mut new_keyed_elements: BTreeMap<
        usize,
        (Vec<&VAL>, (NodeIdx, &Node<NS, TAG, ATT, VAL, EVENT>)),
    > = BTreeMap::new();

    for (new_idx, (new_element_node_idx, new_child)) in
        node_idx_new_elements.iter().enumerate()
    {
        if let Some(new_key) = new_child.get_attribute_value(key) {
            new_keyed_elements
                .insert(new_idx, (new_key, (*new_element_node_idx, new_child)));
        }
    }

    // compiles that matched old and new with
    // with their (old_idx, new_idx) as key and the value is (old_element, new_element)
    let mut matched_old_new_keyed: BTreeMap<
        (usize, usize),
        (
            &Node<NS, TAG, ATT, VAL, EVENT>,
            (NodeIdx, &Node<NS, TAG, ATT, VAL, EVENT>),
        ),
    > = BTreeMap::new();

    let mut last_matched_old_idx = None;
    let mut last_matched_new_idx = None;
    // here, we need to processed both keyed element and non-keyed elements
    for (new_idx, (new_key, new_element)) in new_keyed_elements.iter() {
        // find the old element which has a key value which is the same with the new element key
        // `new_key`
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
                    .insert((old_idx, *new_idx), (old_element, *new_element));
            } else {
                // we don't matched already passed elements
            }
        }
    }

    // unmatched old and idx in pass 1
    let (matched_old_idx, matched_new_idx) =
        get_matched_old_new_idx(&matched_old_new_keyed);

    // these are the new children that didn't matched in the keyed elements pass
    let mut unmatched_new_child: Vec<(
        usize,
        (NodeIdx, &'a Node<NS, TAG, ATT, VAL, EVENT>),
    )> = vec![];

    for (idx, (node_idx, new_element)) in
        node_idx_new_elements.iter().enumerate()
    {
        if !matched_new_idx.contains(&idx) {
            unmatched_new_child.push((idx, (*node_idx, new_element)));
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
        (
            &'a Node<NS, TAG, ATT, VAL, EVENT>,
            (NodeIdx, &'a Node<NS, TAG, ATT, VAL, EVENT>),
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
    let mut unmatched_new_child_pass2: Vec<(
        usize,
        NodeIdx,
        &Node<NS, TAG, ATT, VAL, EVENT>,
    )> = vec![];
    for (idx, (new_element_node_idx, new_child)) in
        node_idx_new_elements.iter().enumerate()
    {
        if !matched_new_idx_pass2.contains(&idx) {
            unmatched_new_child_pass2.push((
                idx,
                *new_element_node_idx,
                new_child,
            ));
        }
    }

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
        let mut child_cur_path = cur_path.clone();
        child_cur_path.push(old_idx);

        let mut child_new_path = new_path.clone();
        child_new_path.push(old_idx);

        if let Some((new_idx, (new_child_node_idx, new_child))) =
            find_matched_new_child(&matched_old_new_keyed, old_idx)
        {
            // insert unmatched new_child that is less than the matched new_idx
            for (idx, new_element_node_idx, unmatched) in
                unmatched_new_child_pass2
                    .iter()
                    .filter(|(idx, _, _)| *idx < new_idx)
            {
                if !already_inserted.contains(idx) {
                    insert_node_patches.push(
                        InsertNode::new(
                            Some(&old_element.tag),
                            PatchPath::new(
                                TreePath::start_at(
                                    *cur_node_idx,
                                    child_cur_path.clone(),
                                ),
                                TreePath::start_at(
                                    *new_element_node_idx,
                                    child_new_path.clone(),
                                ),
                            ),
                            unmatched,
                        )
                        .into(),
                    );
                    already_inserted.push(*idx);
                }
            }

            let mut this_new_child_node_idx = new_child_node_idx;

            matched_keyed_element_patches.extend(diff_recursive(
                old_child,
                new_child,
                cur_node_idx,
                &mut this_new_child_node_idx,
                &child_cur_path,
                &child_new_path,
                key,
                skip,
                rep,
            ));
        } else {
            remove_node_patches.push(
                RemoveNode::new(
                    old_child.tag(),
                    PatchPath::old(TreePath::start_at(
                        *cur_node_idx,
                        child_cur_path.clone(),
                    )),
                )
                .into(),
            );
            increment_node_idx_to_descendant_count(old_child, cur_node_idx);
        }
    }

    // the node that are to be appended are nodes
    // that are not matched, and not already part of the InsertNode
    let mut append_children_patches = vec![];

    for (new_idx, new_element_node_idx, new_child) in
        unmatched_new_child_pass2.iter()
    {
        if !already_inserted.contains(&new_idx) {
            append_children_patches.push(
                AppendChildren::new(
                    &old_element.tag,
                    PatchPath::old(TreePath::start_at(
                        new_child_excess_cur_node_idx,
                        cur_path.clone(),
                    )),
                    vec![(*new_element_node_idx, new_child)],
                )
                .into(),
            );
        }
    }

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
