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
        RemoveChildren,
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
fn increment_node_idx_to_descendant_count<NS, TAG, ATT, VAL, EVENT, MSG>(
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
fn diff_keyed_elements<'a, 'b, NS, TAG, ATT, VAL, EVENT, MSG>(
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
                matched_old_new_keyed
                    .insert((old_idx, *new_idx), (old_element, new_element));
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

    // process this last so as not to move the cur_node_idx forward
    // and without creating a snapshot for cur_node_idx for other patch types
    let mut matched_keyed_element_patches = vec![];

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
            increment_node_idx_to_descendant_count(old_child, cur_node_idx);
        }
    }
    // patch order matters here
    // apply changes to the matched element first,
    // since it creates new changes to the child index nodes
    patches.extend(matched_keyed_element_patches);
    patches.extend(insert_children_patches);
    patches.extend(append_children_patches);
    patches.extend(remove_children_patches);
    dbg!(&patches);
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
        patches.push(
            RemoveChildren::new(
                &old_element.tag,
                this_cur_node_idx,
                (new_child_count..old_child_count).collect::<Vec<usize>>(),
            )
            .into(),
        );

        for old_child in old_element.get_children().iter().skip(new_child_count)
        {
            *cur_node_idx += 1;
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
    NS: PartialEq,
    ATT: PartialEq,
    VAL: PartialEq,
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
