//! provides diffing algorithm which returns patches
#![allow(clippy::type_complexity)]
use crate::{
    node::attribute::group_attributes_per_name, Attribute, Element, Node,
    Patch, TreePath,
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
///     vec![Patch::remove_node(
///         Some(&"div"),
///         TreePath::new(vec![0, 0]),
///     )
///     ]
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
        &[0],
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
    diff_recursive(old_node, new_node, &[0], key, skip, rep)
}

/// returns true if any of the node children has key in their attributes
fn is_any_children_keyed<NS, TAG, ATT, VAL>(
    element: &Element<NS, TAG, ATT, VAL>,
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
fn is_keyed_node<NS, TAG, ATT, VAL>(
    node: &Node<NS, TAG, ATT, VAL>,
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
    if let (Some(old_key), Some(new_key)) = (
        old_node.get_attribute_value(key),
        new_node.get_attribute_value(key),
    ) {
        if old_key != new_key {
            return true;
        }
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
    path: &[usize],
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
        return vec![];
    }

    let replace = should_replace(old_node, new_node, key, rep);

    let mut patches = vec![];

    if replace {
        patches.push(Patch::replace_node(
            old_node.tag(),
            TreePath::new(path.to_vec()),
            new_node,
        ));
        return patches;
    }

    // The following comparison can only contain identical variants, other
    // cases have already been handled above by comparing variant
    // discriminants.
    match (old_node, new_node) {
        // We're comparing two text nodes
        (Node::Text(old_text), Node::Text(new_text)) => {
            if old_text != new_text {
                let ct = Patch::change_text(
                    TreePath::new(path.to_vec()),
                    old_text,
                    new_text,
                );
                patches.push(ct);
            }
        }
        (Node::Comment(old_comment), Node::Comment(new_comment)) => {
            if old_comment != new_comment {
                let ct = Patch::change_comment(
                    TreePath::new(path.to_vec()),
                    old_comment,
                    new_comment,
                );
                patches.push(ct);
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
                    path,
                    skip,
                    rep,
                );
                patches.extend(keyed_patches);
            } else {
                let non_keyed_patches = diff_non_keyed_elements(
                    old_element,
                    new_element,
                    key,
                    path,
                    skip,
                    rep,
                );
                patches.extend(non_keyed_patches);
            }
        }
        _ => {
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
    let this_cur_path = path.to_vec();
    let mut patches = vec![];
    let attributes_patches =
        create_attribute_patches(old_element, new_element, path);
    patches.extend(attributes_patches);

    let old_child_count = old_element.children.len();
    let new_child_count = new_element.children.len();

    let min_count = cmp::min(old_child_count, new_child_count);
    for index in 0..min_count {
        let mut cur_child_path = path.to_vec();
        cur_child_path.push(index);

        let old_child = &old_element
            .children
            .get(index)
            .expect("No old_node child node");
        let new_child =
            &new_element.children.get(index).expect("No new chold node");

        let more_patches = diff_recursive(
            old_child,
            new_child,
            &cur_child_path,
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
            this_cur_path,
        );
        patches.push(append_children_patch);
    }

    if new_child_count < old_child_count {
        let remove_node_patches =
            create_remove_node_patch(old_element, new_element, path);
        patches.extend(remove_node_patches);
    }

    patches
}

fn create_append_children_patch<'a, NS, TAG, ATT, VAL>(
    old_element: &'a Element<NS, TAG, ATT, VAL>,
    new_element: &'a Element<NS, TAG, ATT, VAL>,
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

    Patch::append_children(
        &old_element.tag,
        TreePath::new(this_cur_path.to_vec()),
        append_patch,
    )
}

fn create_remove_node_patch<'a, NS, TAG, ATT, VAL>(
    old_element: &'a Element<NS, TAG, ATT, VAL>,
    new_element: &'a Element<NS, TAG, ATT, VAL>,
    path: &[usize],
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
        let mut child_path = path.to_vec();
        child_path.push(new_child_count + i);
        let remove_node_patch =
            Patch::remove_node(old_child.tag(), TreePath::new(child_path));
        patches.push(remove_node_patch);
    }
    patches
}

///
/// Note: The performance bottlenecks
///     - allocating new vec
///     - merging attributes of the same name
fn create_attribute_patches<'a, NS, TAG, ATT, VAL>(
    old_element: &'a Element<NS, TAG, ATT, VAL>,
    new_element: &'a Element<NS, TAG, ATT, VAL>,
    path: &[usize],
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
        patches.push(Patch::add_attributes(
            &old_element.tag,
            TreePath::new(path.to_vec()),
            add_attributes,
        ));
    }
    if !remove_attributes.is_empty() {
        patches.push(Patch::remove_attributes(
            &old_element.tag,
            TreePath::new(path.to_vec()),
            remove_attributes,
        ));
    }
    patches
}
