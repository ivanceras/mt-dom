use super::{
    diff_recursive,
    increment_node_idx_to_descendant_count,
};
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
use std::{
    cmp,
    collections::BTreeMap,
    fmt,
    hash::Hash,
    iter::FromIterator,
    mem,
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
                log::warn!("key {:?} matched but skipping..", key);
                log::warn!(
                    "because node_idx: {} and last_matched_node_idx: {:?}",
                    node_idx,
                    last_matched_node_idx
                );

                eprintln!("key {:?} matched but skipping..", key);
                eprintln!(
                    "because node_idx: {} and last_matched_node_idx: {:?}",
                    node_idx, last_matched_node_idx
                );
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
) -> Option<&'a Node<NS, TAG, ATT, VAL, EVENT, MSG>> {
    matched_old_new_keyed
        .iter()
        .find_map(|((old_idx, _), (_, new_child))| {
            if *old_idx == find_old_idx {
                Some(*new_child)
            } else {
                None
            }
        })
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
pub fn diff_keyed_elements<'a, 'b, NS, TAG, ATT, VAL, EVENT, MSG>(
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
    let mut patches = vec![];
    // create a hashmap for both the old and new element
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
        println!("new_idx: {}", new_idx);
        if let Some((old_idx, old_element)) = find_node_with_key(
            &old_keyed_elements,
            new_key,
            last_matched_old_idx,
        ) {
            let last_matched_new_idx_val = last_matched_new_idx.unwrap_or(0);
            if last_matched_new_idx.is_none()
                || *new_idx > last_matched_new_idx_val
            {
                last_matched_old_idx = Some(old_idx);
                last_matched_new_idx = Some(*new_idx);
                println!(
                    "found match old_idx: {}, new_idx: {}",
                    old_idx, new_idx
                );
                matched_old_new_keyed
                    .insert((old_idx, *new_idx), (old_element, new_element));
            } else {
                log::warn!("matched new_key: {:?}, but skipping", new_key);
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

    let old_element_max_index = old_element.children.len() - 1;

    dbg!(&unmatched_new_child);
    dbg!(&unmatched_old_child);

    // matched old and new element, not necessarily keyed
    let matched_old_new: BTreeMap<
        (usize, usize),
        (
            &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
            &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
        ),
    > = BTreeMap::from_iter(
        // try to match unmatched new child from the unmatched old child
        // using the their idx
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
    matched_old_new_keyed.extend(matched_old_new);

    // unmatched old and idx in pass 2
    let (matched_old_idx_pass2, matched_new_idx_pass2) =
        get_matched_old_new_idx(&matched_old_new_keyed);

    // this elements are for inserting, appending
    let unmatched_new_child_pass2 = get_unmatched_children_node_idx(
        new_element.get_children(),
        matched_new_idx_pass2,
    );

    // this elements are for removal
    let unmatched_old_child_pass2 = get_unmatched_children_node_idx(
        old_element.get_children(),
        matched_old_idx_pass2,
    );

    // group consecutive children to be inserted in one InsertChildren patch
    let mut grouped_insert_children: BTreeMap<
        usize,
        Vec<&'a Node<NS, TAG, ATT, VAL, EVENT, MSG>>,
    > = BTreeMap::new();

    unmatched_new_child_pass2
        .iter()
        .filter(|(new_idx, _)| *new_idx <= old_element_max_index)
        .for_each(|(new_idx, new_child)| {
            if *new_idx > 0 {
                if let Some(existing_children) =
                    grouped_insert_children.get_mut(&(new_idx - 1))
                {
                    existing_children.push(new_child);
                } else {
                    grouped_insert_children.insert(*new_idx, vec![new_child]);
                }
            } else {
                grouped_insert_children.insert(*new_idx, vec![new_child]);
            }
        });

    let insert_children_patches = grouped_insert_children
        .into_iter()
        .map(|(new_idx, grouped_new_children)| {
            InsertChildren::new(
                &old_element.tag,
                *cur_node_idx,
                new_idx,
                grouped_new_children,
            )
            .into()
        })
        .collect::<Vec<_>>();

    let append_children_patches = unmatched_new_child
        .iter()
        .filter(|(new_idx, _)| *new_idx > old_element_max_index)
        .map(|(new_idx, new_child)| {
            AppendChildren::new(
                &old_element.tag,
                *cur_node_idx,
                vec![new_child],
            )
            .into()
        })
        .collect::<Vec<_>>();

    let for_removal_old_idx: Vec<usize> = unmatched_old_child_pass2
        .iter()
        .map(|(old_idx, _old_child)| *old_idx)
        .collect();

    /*
    let remove_children_patches = if !for_removal_old_idx.is_empty() {
        Some(
            RemoveChildren::new(
                &old_element.tag,
                *cur_node_idx,
                for_removal_old_idx,
            )
            .into(),
        )
    } else {
        None
    };
    */

    // process this last so as not to move the cur_node_idx forward
    // and without creating a snapshot for cur_node_idx for other patch types
    let mut matched_keyed_element_patches = vec![];
    let mut remove_node_patches = vec![];

    for (old_idx, old_child) in old_element.children.iter().enumerate() {
        *cur_node_idx += 1;
        if let Some(new_child) =
            find_matched_new_child(&matched_old_new_keyed, old_idx)
        {
            matched_keyed_element_patches.extend(diff_recursive(
                old_child,
                new_child,
                cur_node_idx,
                key,
            ));
        } else {
            remove_node_patches
                .push(RemoveNode::new(old_child.tag(), *cur_node_idx).into());
            increment_node_idx_to_descendant_count(old_child, cur_node_idx);
        }
    }
    // patch order matters here
    // apply changes to the matched element first,
    // since it creates new changes to the child index nodes
    patches.extend(matched_keyed_element_patches);
    patches.extend(insert_children_patches);
    patches.extend(append_children_patches);
    patches.extend(remove_node_patches);
    patches
}
