//! provides diffing algorithm which returns patches
//!
use crate::{
    node::attribute::group_attributes_per_name,
    patch::{
        AddAttributes, AppendChildren, ChangeText, RemoveAttributes,
        RemoveNode, ReplaceNode,
    },
    Attribute, Element, Node, Patch, TreePath,
};
use keyed_elements::diff_keyed_elements;
use std::fmt::Debug;
use std::{cmp, mem};

mod keyed_elements;

/// Return the patches needed for `old_node` to have the same DOM as `new_node`
///
/// # Agruments
/// * old_node - the old virtual dom node
/// * new_node - the new virtual dom node
/// * key - the literal name of key attribute, ie: "key"
///
/// # Example
/// ```rust
/// use mt_dom::{diff::*, patch::*, *};
///
/// pub type MyNode =
///    Node<&'static str, &'static str, &'static str, &'static str>;
///
/// let old: MyNode = element(
///     "main",
///     vec![attr("class", "container")],
///     vec![
///         element("div", vec![attr("key", "1")], vec![]),
///         element("div", vec![attr("key", "2")], vec![]),
///     ],
/// );
///
/// let new: MyNode = element(
///     "main",
///     vec![attr("class", "container")],
///     vec![element("div", vec![attr("key", "2")], vec![])],
/// );
///
/// let diff = diff_with_key(&old, &new, &"key");
/// assert_eq!(
///     diff,
///     vec![RemoveNode::new(
///         Some(&"div"),
///         TreePath::start_at(1, vec![0, 0]),
///     )
///     .into()]
/// );
/// ```
pub fn diff_with_key<'a, NS, TAG, ATT, VAL>(
    old_node: &'a Node<NS, TAG, ATT, VAL>,
    new_node: &'a Node<NS, TAG, ATT, VAL>,
    key: &ATT,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    diff_recursive(
        old_node,
        new_node,
        &mut 0,
        &vec![0],
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
pub fn diff_with_functions<'a, NS, TAG, ATT, VAL, SKIP, REP>(
    old_node: &'a Node<NS, TAG, ATT, VAL>,
    new_node: &'a Node<NS, TAG, ATT, VAL>,
    key: &ATT,
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
    diff_recursive(old_node, new_node, &mut 0, &vec![0], key, skip, rep)
}

/// returns true if any of the node children has key in their attributes
fn is_any_children_keyed<'a, NS, TAG, ATT, VAL>(
    element: &'a Element<NS, TAG, ATT, VAL>,
    key: &ATT,
) -> bool
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    element
        .get_children()
        .iter()
        .any(|child| is_keyed_node(child, key))
}

/// returns true any attributes of this node attribute has key in it
fn is_keyed_node<'a, NS, TAG, ATT, VAL>(
    node: &'a Node<NS, TAG, ATT, VAL>,
    key: &ATT,
) -> bool
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    if let Some(attributes) = node.get_attributes() {
        attributes.iter().any(|att| att.name == *key)
    } else {
        false
    }
}

fn should_replace<'a, 'b, NS, TAG, ATT, VAL, REP>(
    old_node: &'a Node<NS, TAG, ATT, VAL>,
    new_node: &'a Node<NS, TAG, ATT, VAL>,
    key: &ATT,
    rep: &REP,
) -> bool
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    REP: Fn(&'a Node<NS, TAG, ATT, VAL>, &'a Node<NS, TAG, ATT, VAL>) -> bool,
{
    // replace if they have different enum variants
    if mem::discriminant(old_node) != mem::discriminant(new_node) {
        return true;
    }

    // handle explicit replace if the REP fn evaluates to true
    if rep(old_node, new_node) {
        return true;
    }

    // replace if the old key does not match the new key
    match (
        old_node.get_attribute_value(&key),
        new_node.get_attribute_value(&key),
    ) {
        (Some(old_key), Some(new_key)) => {
            if old_key != new_key {
                return true;
            }
        }
        _ => (),
    }
    // replace if they have different element tag
    if let (Node::Element(old_element), Node::Element(new_element)) =
        (old_node, new_node)
    {
        // Replace if there are different element tags
        if old_element.tag != new_element.tag {
            return true;
        }
    }
    false
}

fn diff_recursive<'a, 'b, NS, TAG, ATT, VAL, SKIP, REP>(
    old_node: &'a Node<NS, TAG, ATT, VAL>,
    new_node: &'a Node<NS, TAG, ATT, VAL>,
    cur_node_idx: &'b mut usize,
    cur_path: &Vec<usize>,
    key: &ATT,
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
    // skip diffing if the function evaluates to true
    if skip(old_node, new_node) {
        *cur_node_idx += old_node.descendant_node_count();
        return vec![];
    }

    let replace = should_replace(old_node, new_node, key, rep);

    let mut patches = vec![];

    if replace {
        patches.push(
            ReplaceNode::new(
                old_node.tag(),
                TreePath::start_at(*cur_node_idx, cur_path.clone()),
                new_node,
            )
            .into(),
        );
        *cur_node_idx += old_node.descendant_node_count();
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
                    old_text,
                    TreePath::start_at(*cur_node_idx, cur_path.clone()),
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
                    cur_path,
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
                    cur_path,
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
fn diff_non_keyed_elements<'a, 'b, NS, TAG, ATT, VAL, SKIP, REP>(
    old_element: &'a Element<NS, TAG, ATT, VAL>,
    new_element: &'a Element<NS, TAG, ATT, VAL>,
    key: &ATT,
    cur_node_idx: &'b mut usize,
    cur_path: &Vec<usize>,
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
    let this_cur_node_idx = *cur_node_idx;
    let this_cur_path = cur_path.clone();
    let mut patches = vec![];
    let attributes_patches = create_attribute_patches(
        old_element,
        new_element,
        *cur_node_idx,
        cur_path,
    );
    patches.extend(attributes_patches);

    let old_child_count = old_element.children.len();
    let new_child_count = new_element.children.len();

    let min_count = cmp::min(old_child_count, new_child_count);
    for index in 0..min_count {
        *cur_node_idx += 1;

        let mut cur_child_path = cur_path.clone();
        cur_child_path.push(index);
        println!(
            "\t we just added index: {} ... cur_child_path is now: {:?}",
            index, cur_child_path
        );

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
            &mut cur_child_path,
            key,
            skip,
            rep,
        );
        patches.extend(more_patches);
    }

    // If there are more new child than old_node child, we make a patch to append the excess element
    // starting from old_child_count to the last item of the new_elements
    if new_child_count > old_child_count {
        let append_children_patch = create_append_children_patch(
            old_element,
            new_element,
            this_cur_node_idx,
            this_cur_path,
        );
        patches.push(append_children_patch);
    }

    if new_child_count < old_child_count {
        let remove_node_patches = create_remove_node_patch(
            old_element,
            new_element,
            cur_node_idx,
            cur_path,
        );
        patches.extend(remove_node_patches);
    }

    patches
}

fn create_append_children_patch<'a, 'b, NS, TAG, ATT, VAL>(
    old_element: &'a Element<NS, TAG, ATT, VAL>,
    new_element: &'a Element<NS, TAG, ATT, VAL>,
    this_cur_node_idx: usize,
    this_cur_path: Vec<usize>,
) -> Patch<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    let old_child_count = old_element.children.len();
    let mut append_patch: Vec<&'a Node<NS, TAG, ATT, VAL>> = vec![];

    for append_child in new_element.children.iter().skip(old_child_count) {
        append_patch.push(append_child);
    }

    AppendChildren::new(
        &old_element.tag,
        TreePath::start_at(this_cur_node_idx, this_cur_path.clone()),
        append_patch,
    )
    .into()
}

fn create_remove_node_patch<'a, 'b, NS, TAG, ATT, VAL>(
    old_element: &'a Element<NS, TAG, ATT, VAL>,
    new_element: &'a Element<NS, TAG, ATT, VAL>,
    cur_node_idx: &'b mut usize,
    cur_path: &Vec<usize>,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    let new_child_count = new_element.children.len();
    let mut patches = vec![];
    for (i, old_child) in old_element
        .get_children()
        .iter()
        .skip(new_child_count)
        .enumerate()
    {
        *cur_node_idx += 1;
        let mut child_cur_path = cur_path.clone();
        child_cur_path.push(new_child_count + i);
        let remove_node_patch = RemoveNode::new(
            old_child.tag(),
            TreePath::start_at(*cur_node_idx, child_cur_path),
        );
        patches.push(remove_node_patch.into());
        *cur_node_idx += old_child.descendant_node_count();
    }
    patches
}

/// diff the attributes of old_node element to the new element at this cur_node_idx
///
/// Note: The performance bottlenecks
///     - allocating new vec
///     - merging attributes of the same name
fn create_attribute_patches<'a, 'b, NS, TAG, ATT, VAL>(
    old_element: &'a Element<NS, TAG, ATT, VAL>,
    new_element: &'a Element<NS, TAG, ATT, VAL>,
    cur_node_idx: usize,
    cur_path: &Vec<usize>,
) -> Vec<Patch<'a, NS, TAG, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    let mut patches = vec![];
    let mut add_attributes: Vec<&Attribute<NS, ATT, VAL>> = vec![];
    let mut remove_attributes: Vec<&Attribute<NS, ATT, VAL>> = vec![];

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
                TreePath::start_at(cur_node_idx, cur_path.clone()),
                add_attributes,
            )
            .into(),
        );
    }
    if !remove_attributes.is_empty() {
        patches.push(
            RemoveAttributes::new(
                &old_element.tag,
                TreePath::start_at(cur_node_idx, cur_path.clone()),
                remove_attributes,
            )
            .into(),
        );
    }
    patches
}
