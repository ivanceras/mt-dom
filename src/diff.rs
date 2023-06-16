//! provides diffing algorithm which returns patches
use crate::{
    node::attribute::group_attributes_per_name, Attribute, Element, Node,
    Patch, TreePath,
};
use std::{cmp, fmt::Debug, mem};

mod keyed;

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
///    Node<&'static str, &'static str, &'static str, &'static str, &'static str>;
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
///         TreePath::new(vec![ 0]),
///     )
///     ]
/// );
/// ```
pub fn diff_with_key<'a, Ns, Tag, Leaf, Att, Val>(
    old_node: &'a Node<Ns, Tag, Leaf, Att, Val>,
    new_node: &'a Node<Ns, Tag, Leaf, Att, Val>,
    key: &Att,
) -> Vec<Patch<'a, Ns, Tag, Leaf, Att, Val>>
where
    Ns: PartialEq + Clone + Debug,
    Tag: PartialEq + Debug,
    Leaf: PartialEq + Clone + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
{
    diff_recursive(
        old_node,
        new_node,
        &TreePath::root(),
        key,
        &|_old, _new| false,
        &|_old, _new| false,
    )
}

/// calculate the difference of 2 nodes
/// if the skip function evaluates to true, then diffing of
/// the node and all of it's descendant will be skipped entirely and then proceed to the next node.
///
/// The Skip fn is passed to check whether the diffing of the old and new element should be
/// skipped, and assumed no changes. This is for optimization where the developer is sure that
/// the dom tree hasn't change.
///
/// Rep fn stands for replace function which decides if the new element should
/// just replace the old element without diffing
///
pub fn diff_with_functions<'a, Ns, Tag, Leaf, Att, Val, Skip, Rep>(
    old_node: &'a Node<Ns, Tag, Leaf, Att, Val>,
    new_node: &'a Node<Ns, Tag, Leaf, Att, Val>,
    key: &Att,
    skip: &Skip,
    rep: &Rep,
) -> Vec<Patch<'a, Ns, Tag, Leaf, Att, Val>>
where
    Ns: PartialEq + Clone + Debug,
    Tag: PartialEq + Debug,
    Leaf: PartialEq + Clone + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,

    Skip: Fn(
        &'a Node<Ns, Tag, Leaf, Att, Val>,
        &'a Node<Ns, Tag, Leaf, Att, Val>,
    ) -> bool,
    Rep: Fn(
        &'a Node<Ns, Tag, Leaf, Att, Val>,
        &'a Node<Ns, Tag, Leaf, Att, Val>,
    ) -> bool,
{
    diff_recursive(old_node, new_node, &TreePath::root(), key, skip, rep)
}

/// returns true if any of the children of this element has key in their attributes
fn is_any_children_keyed<Ns, Tag, Leaf, Att, Val>(
    element: &Element<Ns, Tag, Leaf, Att, Val>,
    key: &Att,
) -> bool
where
    Ns: PartialEq + Clone + Debug,
    Tag: PartialEq + Debug,
    Leaf: PartialEq + Clone + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
{
    element
        .children
        .iter()
        .any(|child| is_keyed_node(child, key))
}

/// returns true any attributes of this node attribute has key in it
fn is_keyed_node<Ns, Tag, Leaf, Att, Val>(
    node: &Node<Ns, Tag, Leaf, Att, Val>,
    key: &Att,
) -> bool
where
    Ns: PartialEq + Clone + Debug,
    Tag: PartialEq + Debug,
    Leaf: PartialEq + Clone + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
{
    if let Some(attributes) = node.get_attributes() {
        attributes.iter().any(|att| att.name == *key)
    } else {
        false
    }
}

fn should_replace<'a, 'b, Ns, Tag, Leaf, Att, Val, Rep>(
    old_node: &'a Node<Ns, Tag, Leaf, Att, Val>,
    new_node: &'a Node<Ns, Tag, Leaf, Att, Val>,
    key: &Att,
    rep: &Rep,
) -> bool
where
    Ns: PartialEq + Clone + Debug,
    Tag: PartialEq + Debug,
    Leaf: PartialEq + Clone + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
    Rep: Fn(
        &'a Node<Ns, Tag, Leaf, Att, Val>,
        &'a Node<Ns, Tag, Leaf, Att, Val>,
    ) -> bool,
{
    // replace if they have different enum variants
    if mem::discriminant(old_node) != mem::discriminant(new_node) {
        return true;
    }

    // handle explicit replace if the Rep fn evaluates to true
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

pub(crate) fn diff_recursive<'a, 'b, Ns, Tag, Leaf, Att, Val, Skip, Rep>(
    old_node: &'a Node<Ns, Tag, Leaf, Att, Val>,
    new_node: &'a Node<Ns, Tag, Leaf, Att, Val>,
    path: &TreePath,
    key: &Att,
    skip: &Skip,
    rep: &Rep,
) -> Vec<Patch<'a, Ns, Tag, Leaf, Att, Val>>
where
    Ns: PartialEq + Clone + Debug,
    Leaf: PartialEq + Clone + Debug,
    Tag: PartialEq + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
    Skip: Fn(
        &'a Node<Ns, Tag, Leaf, Att, Val>,
        &'a Node<Ns, Tag, Leaf, Att, Val>,
    ) -> bool,
    Rep: Fn(
        &'a Node<Ns, Tag, Leaf, Att, Val>,
        &'a Node<Ns, Tag, Leaf, Att, Val>,
    ) -> bool,
{
    // skip diffing if the function evaluates to true
    if skip(old_node, new_node) {
        return vec![];
    }

    // replace node and return early
    if should_replace(old_node, new_node, key, rep) {
        return vec![Patch::replace_node(
            old_node.tag(),
            path.clone(),
            new_node,
        )];
    }

    let mut patches = vec![];

    // The following comparison can only contain identical variants, other
    // cases have already been handled above by comparing variant
    // discriminants.
    match (old_node, new_node) {
        (Node::Leaf(old_leaf), Node::Leaf(new_leaf)) => {
            if old_leaf != new_leaf {
                let ct =
                    Patch::replace_node(old_node.tag(), path.clone(), new_node);
                patches.push(ct);
            }
        }
        // We're comparing two element nodes
        (Node::Element(old_element), Node::Element(new_element)) => {
            let diff_as_keyed = is_any_children_keyed(old_element, key)
                || is_any_children_keyed(new_element, key);

            if diff_as_keyed {
                /*
                let keyed_patches = keyed::diff_keyed_elements(
                    old_element,
                    new_element,
                    key,
                    path,
                    skip,
                    rep,
                );
                patches.extend(keyed_patches);
                */
                let keyed_patches = crate::diff_lis::diff_keyed_elements(
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
        (Node::NodeList(_old_elements), Node::NodeList(_new_elements)) => {
            panic!(
                "Node list must have already unrolled when creating an element"
            );
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
fn diff_non_keyed_elements<'a, 'b, Ns, Tag, Leaf, Att, Val, Skip, Rep>(
    old_element: &'a Element<Ns, Tag, Leaf, Att, Val>,
    new_element: &'a Element<Ns, Tag, Leaf, Att, Val>,
    key: &Att,
    path: &TreePath,
    skip: &Skip,
    rep: &Rep,
) -> Vec<Patch<'a, Ns, Tag, Leaf, Att, Val>>
where
    Ns: PartialEq + Clone + Debug,
    Leaf: PartialEq + Clone + Debug,
    Tag: PartialEq + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
    Skip: Fn(
        &'a Node<Ns, Tag, Leaf, Att, Val>,
        &'a Node<Ns, Tag, Leaf, Att, Val>,
    ) -> bool,
    Rep: Fn(
        &'a Node<Ns, Tag, Leaf, Att, Val>,
        &'a Node<Ns, Tag, Leaf, Att, Val>,
    ) -> bool,
{
    let mut patches = vec![];
    let attributes_patches =
        create_attribute_patches(old_element, new_element, path);
    patches.extend(attributes_patches);

    let more_patches = diff_non_keyed_children(
        &old_element.tag,
        &old_element.children,
        &new_element.children,
        key,
        path,
        skip,
        rep,
    );
    patches.extend(more_patches);
    patches
}

fn diff_non_keyed_children<'a, Ns, Tag, Leaf, Att, Val, Skip, Rep>(
    old_element_tag: &'a Tag,
    old_children: &'a Vec<Node<Ns, Tag, Leaf, Att, Val>>,
    new_children: &'a Vec<Node<Ns, Tag, Leaf, Att, Val>>,
    key: &Att,
    path: &TreePath,
    skip: &Skip,
    rep: &Rep,
) -> Vec<Patch<'a, Ns, Tag, Leaf, Att, Val>>
where
    Ns: PartialEq + Clone + Debug,
    Tag: PartialEq + Debug,
    Leaf: PartialEq + Clone + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
    Skip: Fn(
        &'a Node<Ns, Tag, Leaf, Att, Val>,
        &'a Node<Ns, Tag, Leaf, Att, Val>,
    ) -> bool,
    Rep: Fn(
        &'a Node<Ns, Tag, Leaf, Att, Val>,
        &'a Node<Ns, Tag, Leaf, Att, Val>,
    ) -> bool,
{
    let mut patches = vec![];
    let old_child_count = old_children.len();
    let new_child_count = new_children.len();

    let min_count = cmp::min(old_child_count, new_child_count);
    for index in 0..min_count {
        // if we iterate trough the old elements, a new child_path is created for that iteration
        let child_path = path.traverse(index);

        let old_child =
            &old_children.get(index).expect("No old_node child node");
        let new_child = &new_children.get(index).expect("No new child node");

        let more_patches =
            diff_recursive(old_child, new_child, &child_path, key, skip, rep);
        patches.extend(more_patches);
    }

    // If there are more new child than old_node child, we make a patch to append the excess element
    // starting from old_child_count to the last item of the new_elements
    if new_child_count > old_child_count {
        patches.push(Patch::append_children(
            old_element_tag,
            path.clone(),
            new_children.iter().skip(old_child_count).collect(),
        ));
    }

    if new_child_count < old_child_count {
        let remove_node_patches = old_children
            .iter()
            .skip(new_child_count)
            .enumerate()
            .map(|(i, old_child)| {
                Patch::remove_node(
                    old_child.tag(),
                    path.traverse(new_child_count + i),
                )
            })
            .collect::<Vec<_>>();

        patches.extend(remove_node_patches);
    }

    patches
}

///
/// Note: The performance bottlenecks
///     - allocating new vec
///     - merging attributes of the same name
fn create_attribute_patches<'a, Ns, Tag, Leaf, Att, Val>(
    old_element: &'a Element<Ns, Tag, Leaf, Att, Val>,
    new_element: &'a Element<Ns, Tag, Leaf, Att, Val>,
    path: &TreePath,
) -> Vec<Patch<'a, Ns, Tag, Leaf, Att, Val>>
where
    Ns: PartialEq + Clone + Debug,
    Leaf: PartialEq + Clone + Debug,
    Tag: PartialEq + Debug,
    Att: PartialEq + Clone + Debug,
    Val: PartialEq + Clone + Debug,
{
    let mut patches = vec![];
    let mut add_attributes: Vec<&Attribute<Ns, Att, Val>> = vec![];
    let mut remove_attributes: Vec<&Attribute<Ns, Att, Val>> = vec![];

    let new_attributes_grouped =
        group_attributes_per_name(new_element.get_attributes());
    let old_attributes_grouped =
        group_attributes_per_name(old_element.get_attributes());

    // for all new elements that doesn't exist in the old elements
    // or the values differ
    // add it to the AddAttribute patches
    for (new_attr_name, new_attrs) in new_attributes_grouped.iter() {
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
            path.clone(),
            add_attributes,
        ));
    }
    if !remove_attributes.is_empty() {
        patches.push(Patch::remove_attributes(
            &old_element.tag,
            path.clone(),
            remove_attributes,
        ));
    }
    patches
}
