#![allow(warnings)]

//! derived from dioxus diff for keyed elements

use crate::diff::diff_recursive;
use crate::{Element, Node, Patch, TreePath};
use std::fmt::Debug;

fn get_key<'a, NS, TAG, LEAF, ATT, VAL>(
    node: &'a Node<NS, TAG, LEAF, ATT, VAL>,
    key: &'a ATT,
) -> Option<Vec<&'a VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    node.get_attribute_value(key)
}

// Diffing "keyed" children.
//
// With keyed children, we care about whether we delete, move, or create nodes
// versus mutate existing nodes in place. Presumably there is some sort of CSS
// transition animation that makes the virtual DOM diffing algorithm
// observable. By specifying keys for nodes, we know which virtual DOM nodes
// must reuse (or not reuse) the same physical DOM nodes.
//
// This is loosely based on Inferno's keyed patching implementation. However, we
// have to modify the algorithm since we are compiling the diff down into change
// list instructions that will be executed later, rather than applying the
// changes to the DOM directly as we compare virtual DOMs.
//
// https://github.com/infernojs/inferno/blob/36fd96/packages/inferno/src/DOM/patching.ts#L530-L739
//
// The stack is empty upon entry.
fn diff_keyed_children<'a, 'b, NS, TAG, LEAF, ATT, VAL, SKIP, REP>(
    old_element: &'a Element<NS, TAG, LEAF, ATT, VAL>,
    new_element: &'a Element<NS, TAG, LEAF, ATT, VAL>,
    key: &ATT,
    path: &[usize],
    skip: &SKIP,
    rep: &REP,
) -> Vec<Patch<'a, NS, TAG, LEAF, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    SKIP: Fn(
        &'a Node<NS, TAG, LEAF, ATT, VAL>,
        &'a Node<NS, TAG, LEAF, ATT, VAL>,
    ) -> bool,
    REP: Fn(
        &'a Node<NS, TAG, LEAF, ATT, VAL>,
        &'a Node<NS, TAG, LEAF, ATT, VAL>,
    ) -> bool,
{
    if cfg!(debug_assertions) {
        let mut keys = Vec::new();
        let mut assert_unique_keys =
            |children: &'a [Node<NS, TAG, LEAF, ATT, VAL>]| {
                keys.clear();
                for child in children {
                    let key = get_key(child, key);
                    debug_assert!(
                        key.is_some(),
                        "if any sibling is keyed, all siblings must be keyed"
                    );

                    if !keys.contains(&key) {
                        keys.push(key);
                    }
                }
                debug_assert_eq!(
                    children.len(),
                    keys.len(),
                    "keyed siblings must each have a unique key"
                );
            };
        assert_unique_keys(&old_element.children);
        assert_unique_keys(&new_element.children);
    }

    // First up, we diff all the nodes with the same key at the beginning of the
    // children.
    //
    // `shared_prefix_count` is the count of how many nodes at the start of
    // `new` and `old` share the same keys.

    let (offsets, more_patches) =
        diff_keyed_ends(old_element, new_element, key, path, skip, rep);

    if offsets.is_none() {
        return more_patches;
    }

    // proceed to process the offsets
    let (left_offset, right_offset) =
        offsets.expect("must have left offsets and right offsets");

    // Ok, we now hopefully have a smaller range of children in the middle
    // within which to re-order nodes with the same keys, remove old nodes with
    // now-unused keys, and create new nodes with fresh keys.

    let old_middle = &old_element.children
        [left_offset..(old_element.children.len() - right_offset)];
    let new_middle = &new_element.children
        [left_offset..(new_element.children.len() - right_offset)];

    debug_assert!(
        !((old_middle.len() == new_middle.len()) && old_middle.is_empty()),
        "keyed children must have the same number of children"
    );

    let start_range = left_offset;
    let new_end_range = new_element.children.len() - right_offset;
    let old_end_range = old_element.children.len() - right_offset;

    if new_middle.is_empty() {
        // remove the old elements
        create_remove_nodes_patch(
            &old_element.children,
            path,
            start_range,
            old_end_range,
        )
    } else if old_middle.is_empty() {
        // there were no old elements, so just create the new elements
        // we need to find the right "foothold" though - we shouldn't use the "append" at all
        if left_offset == 0 {
            // insert at the beginning of the old list
            let location = old_element.children.len() - right_offset;
            let mut location_path = path.to_vec();
            location_path.push(location);

            vec![Patch::insert_before_node(
                new_element.children[location].tag(),
                TreePath::new(location_path),
                new_element.children[start_range..new_end_range]
                    .iter()
                    .collect(),
            )]
        } else if right_offset == 0 {
            // insert at the end  the old list
            let location = old_element.children.len();
            let mut location_path = path.to_vec();
            location_path.push(location);

            vec![Patch::insert_after_node(
                new_element.children[location].tag(),
                TreePath::new(location_path),
                new_element.children[start_range..new_end_range]
                    .iter()
                    .collect(),
            )]
        } else {
            // inserting in the middle
            let location = left_offset - 1;

            let mut location_path = path.to_vec();
            location_path.push(location);

            vec![Patch::insert_after_node(
                new_element.children[location].tag(),
                TreePath::new(location_path),
                new_element.children[start_range..new_end_range]
                    .iter()
                    .collect(),
            )]
        }
    } else {
        diff_keyed_middle(
            old_element,
            new_element,
            key,
            path,
            skip,
            rep,
            (start_range, old_end_range),
            (start_range, new_end_range),
        )
    }
}

/// Diff both ends of the children that share keys.
///
/// Returns a left offset and right offset of that indicates a smaller section to pass onto the middle diffing.
///
/// If there is no offset, then this function returns None and the diffing is complete.
fn diff_keyed_ends<'a, 'b, NS, TAG, LEAF, ATT, VAL, SKIP, REP>(
    old_element: &'a Element<NS, TAG, LEAF, ATT, VAL>,
    new_element: &'a Element<NS, TAG, LEAF, ATT, VAL>,
    key: &ATT,
    path: &[usize],
    skip: &SKIP,
    rep: &REP,
) -> (
    Option<(usize, usize)>,
    Vec<Patch<'a, NS, TAG, LEAF, ATT, VAL>>,
)
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    SKIP: Fn(
        &'a Node<NS, TAG, LEAF, ATT, VAL>,
        &'a Node<NS, TAG, LEAF, ATT, VAL>,
    ) -> bool,
    REP: Fn(
        &'a Node<NS, TAG, LEAF, ATT, VAL>,
        &'a Node<NS, TAG, LEAF, ATT, VAL>,
    ) -> bool,
{
    let mut left_offset = 0;
    let mut patches = vec![];

    for (index, (old, new)) in old_element
        .children
        .iter()
        .zip(new_element.children.iter())
        .enumerate()
    {
        // if the elements are iterated, we create a child path for that iteration
        let mut child_path = path.to_vec();
        child_path.push(index);
        // abort early if we finally run into nodes with different keys
        if get_key(old, key) != get_key(new, key) {
            break;
        }
        let more_patches =
            diff_recursive(old, new, &child_path, key, skip, rep);
        patches.extend(more_patches);

        left_offset += 1;
    }

    // If that was all of the old children, then create and append the remaining
    // new children and we're finished.
    if left_offset == old_element.children.len() {
        let start_range = left_offset;
        let new_end_range = new_element.children.len();
        let location = old_element.children.len();

        let mut location_path = path.to_vec();
        location_path.push(location);

        let insert_path = Patch::insert_after_node(
            new_element.children[location].tag(),
            TreePath::new(location_path),
            new_element.children[start_range..new_end_range]
                .iter()
                .collect(),
        );
        patches.push(insert_path);

        return (None, patches);
    }

    // And if that was all of the new children, then remove all of the remaining
    // old children and we're finished.
    if left_offset == new_element.children.len() {
        let right_offset = old_element.children.len();

        let more_patches = create_remove_nodes_patch(
            &old_element.children,
            path,
            left_offset,
            right_offset,
        );
        patches.extend(more_patches);

        return (None, patches);
    }

    // if the shared prefix is less than either length, then we need to walk backwards
    let mut right_offset = 0;
    for (old, new) in old_element
        .children
        .iter()
        .rev()
        .zip(new_element.children.iter().rev())
    {
        // abort early if we finally run into nodes with different keys
        if get_key(old, key) != get_key(new, key) {
            break;
        }
        let more_patches = diff_recursive(old, new, path, key, skip, rep);
        patches.extend(more_patches);
        right_offset += 1;
    }

    (Some((left_offset, right_offset)), patches)
}

// The most-general, expensive code path for keyed children diffing.
//
// We find the longest subsequence within `old` of children that are relatively
// ordered the same way in `new` (via finding a longest-increasing-subsequence
// of the old child's index within `new`). The children that are elements of
// this subsequence will remain in place, minimizing the number of DOM moves we
// will have to do.
//
// Upon entry to this function, the change list stack must be empty.
//
// This function will load the appropriate nodes onto the stack and do diffing in place.
//
// Upon exit from this function, it will be restored to that same self.
#[allow(clippy::too_many_lines)]
fn diff_keyed_middle<'a, 'b, NS, TAG, LEAF, ATT, VAL, SKIP, REP>(
    old_element: &'a Element<NS, TAG, LEAF, ATT, VAL>,
    new_element: &'a Element<NS, TAG, LEAF, ATT, VAL>,
    key: &ATT,
    path: &[usize],
    skip: &SKIP,
    rep: &REP,
    old_range: (usize, usize),
    new_range: (usize, usize),
) -> Vec<Patch<'a, NS, TAG, LEAF, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    SKIP: Fn(
        &'a Node<NS, TAG, LEAF, ATT, VAL>,
        &'a Node<NS, TAG, LEAF, ATT, VAL>,
    ) -> bool,
    REP: Fn(
        &'a Node<NS, TAG, LEAF, ATT, VAL>,
        &'a Node<NS, TAG, LEAF, ATT, VAL>,
    ) -> bool,
{
    /*
    1. Map the old keys into a numerical ordering based on indices.
    2. Create a map of old key to its index
    3. Map each new key to the old key, carrying over the old index.
        - IE if we have ABCD becomes BACD, our sequence would be 1,0,2,3
        - if we have ABCD to ABDE, our sequence would be 0,1,3,MAX because E doesn't exist

    now, we should have a list of integers that indicates where in the old list the new items map to.

    4. Compute the LIS of this list
        - this indicates the longest list of new children that won't need to be moved.

    5. Identify which nodes need to be removed
    6. Identify which nodes will need to be diffed

    7. Going along each item in the new list, create it and insert it before the next closest item in the LIS.
        - if the item already existed, just move it to the right place.

    8. Finally, generate instructions to remove any old children.
    9. Generate instructions to finally diff children that are the same between both
    */

    let (old_start_range, old_end_range) = old_range;
    let (new_start_range, new_end_range) = new_range;

    let old_middle = &old_element.children[old_start_range..old_end_range];
    let new_middle = &new_element.children[new_start_range..new_end_range];

    // 0. Debug sanity checks
    // Should have already diffed the shared-key prefixes and suffixes.
    let map_get_key = |node| get_key(node, key);
    debug_assert_ne!(
        new_middle.first().map(map_get_key),
        old_middle.first().map(map_get_key)
    );
    debug_assert_ne!(
        new_middle.last().map(map_get_key),
        old_middle.last().map(map_get_key)
    );

    // 1. Map the old keys into a numerical ordering based on indices.
    // 2. Create a map of old key to its index
    // IE if the keys were A B C, then we would have (A, 1) (B, 2) (C, 3).
    let old_key_to_old_index: Vec<(Vec<&VAL>, usize)> = old_middle
        .iter()
        .enumerate()
        .map(|(idx, node)| (get_key(node, key).unwrap(), idx))
        .collect::<Vec<_>>();

    let mut shared_keys = Vec::new();

    // 3. Map each new key to the old key, carrying over the old index.
    let new_index_to_old_index = new_middle
        .iter()
        .map(|node| {
            let node_key: Vec<&VAL> = get_key(node, key).unwrap();
            if let Some(&index) =
                old_key_to_old_index.iter().find_map(|(k, idx)| {
                    if &node_key == k {
                        Some(idx)
                    } else {
                        None
                    }
                })
            {
                if !shared_keys.contains(&key) {
                    shared_keys.push(key);
                }
                index
            } else {
                u32::MAX as usize
            }
        })
        .collect::<Vec<_>>();

    let mut patches = vec![];

    // If none of the old keys are reused by the new children, then we remove all the remaining old children and
    // create the new children afresh.
    if shared_keys.is_empty() {
        if let Some(first_old) = old_middle.get(0) {
            let start_range = 1;
            let end_range = old_middle.len();

            let more_patches1 = create_remove_nodes_patch(
                old_middle,
                path,
                start_range,
                end_range,
            );
            patches.extend(more_patches1);

            let location = start_range;
            let mut location_path = path.to_vec();
            location_path.push(location);
            let replacement = &new_middle[start_range];

            let replace_patch = Patch::replace_node(
                new_element.children[location].tag(),
                TreePath::new(location_path),
                replacement,
            );

            patches.push(replace_patch);
        } else {
            // I think this is wrong - why are we appending?
            // only valid of the if there are no trailing elements
            let more_patches =
                create_and_append_children(old_element, new_middle, path);
            patches.extend(more_patches);
        }
        return patches;
    }

    // 4. Compute the LIS of this list
    let mut lis_sequence = Vec::default();
    lis_sequence.reserve(new_index_to_old_index.len());

    let mut predecessors = vec![0; new_index_to_old_index.len()];
    let mut starts = vec![0; new_index_to_old_index.len()];

    longest_increasing_subsequence::lis_with(
        &new_index_to_old_index,
        &mut lis_sequence,
        |a, b| a < b,
        &mut predecessors,
        &mut starts,
    );

    // the lis comes out backwards, I think. can't quite tell.
    lis_sequence.sort_unstable();

    // if a new node gets u32 max and is at the end, then it might be part of our LIS (because u32 max is a valid LIS)
    if lis_sequence.last().map(|f| new_index_to_old_index[*f])
        == Some(u32::MAX as usize)
    {
        lis_sequence.pop();
    }

    for idx in &lis_sequence {
        let more_patches = diff_recursive(
            &old_middle[new_index_to_old_index[*idx]],
            &new_middle[*idx],
            path,
            key,
            skip,
            rep,
        );
        patches.extend(more_patches);
    }

    let mut nodes_created = 0;

    // add mount instruction for the first items not covered by the lis
    let last = *lis_sequence.last().unwrap();
    if last < (new_middle.len() - 1) {
        let mut for_insert_after = vec![];
        for (idx, new_node) in new_middle[(last + 1)..].iter().enumerate() {
            let new_idx = idx + last + 1;
            let old_index = new_index_to_old_index[new_idx];
            if old_index == u32::MAX as usize {
                for_insert_after.push(new_node);
            } else {
                let more_patches1 = diff_recursive(
                    &old_middle[old_index],
                    new_node,
                    path,
                    key,
                    skip,
                    rep,
                );
                patches.extend(more_patches1);

                let (created, more_patches2) = push_all_nodes(new_node, path);
                patches.extend(more_patches2);

                nodes_created += created;
            }
        }

        let location = new_start_range + last;
        let mut location_path = path.to_vec();
        location_path.push(location);

        let insert_patch = Patch::insert_after_node(
            new_element.children[location].tag(),
            TreePath::new(location_path),
            for_insert_after,
        );

        patches.push(insert_patch);

        nodes_created = 0;
    }

    // for each spacing, generate a mount instruction
    let mut lis_iter = lis_sequence.iter().rev();
    let mut last = *lis_iter.next().unwrap();
    for next in lis_iter {
        if last - next > 1 {
            // vec of nodes that will be inserted before the location
            let mut for_insert_before = vec![];
            for (idx, new_node) in
                new_middle[(next + 1)..last].iter().enumerate()
            {
                let new_idx = idx + next + 1;
                let old_index = new_index_to_old_index[new_idx];
                if old_index == u32::MAX as usize {
                    for_insert_before.push(new_node);
                } else {
                    let mut more_patches1 = diff_recursive(
                        &old_middle[old_index],
                        new_node,
                        path,
                        key,
                        skip,
                        rep,
                    );
                    patches.extend(more_patches1);

                    let (created, more_patches2) =
                        push_all_nodes(new_node, path);
                    patches.extend(more_patches2);

                    nodes_created += created;
                }
            }

            let location = new_start_range + last;
            let mut location_path = path.to_vec();
            location_path.push(location);

            let insert_patch = Patch::insert_before_node(
                new_element.children[location].tag(),
                TreePath::new(location_path),
                for_insert_before,
            );
            patches.push(insert_patch);

            nodes_created = 0;
        }
        last = *next;
    }

    // add mount instruction for the last items not covered by the lis
    let first_lis = *lis_sequence.first().unwrap();
    if first_lis > 0 {
        let mut for_insert_before = vec![];
        for (idx, new_node) in new_middle[..first_lis].iter().enumerate() {
            let old_index = new_index_to_old_index[idx];
            if old_index == u32::MAX as usize {
                for_insert_before.push(new_node);
            } else {
                let mut more_patches1 = diff_recursive(
                    &old_middle[old_index],
                    new_node,
                    path,
                    key,
                    skip,
                    rep,
                );
                patches.extend(more_patches1);

                let (created, more_patches2) = push_all_nodes(new_node, path);
                patches.extend(more_patches2);

                nodes_created += created;
            }
        }

        let location = new_start_range + first_lis;
        let mut location_path = path.to_vec();
        location_path.push(location);
        let insert_patch = Patch::insert_before_node(
            new_element.children[location].tag(),
            TreePath::new(location_path),
            for_insert_before,
        );
        patches.push(insert_patch);
    }
    patches
}

/// create an RemoveNode patch, which will remove
/// all the nodes that is enumerated here
///
/// The patch path for each of the node is on left_offset_path and right_offset path
fn create_remove_nodes_patch<'a, NS, TAG, LEAF, ATT, VAL>(
    nodes: &'a [Node<NS, TAG, LEAF, ATT, VAL>],
    path: &[usize],
    start_range: usize,
    end_range: usize,
) -> Vec<Patch<'a, NS, TAG, LEAF, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    nodes[start_range..end_range]
        .iter()
        .enumerate()
        .map(|(index, node)| {
            let mut patch_path = path.to_vec();
            patch_path.push(start_range + index);
            Patch::remove_node(node.tag(), TreePath::new(patch_path))
        })
        .collect()
}

/// create the nodes and then append them, in the current path
fn create_and_append_children<'a, NS, TAG, LEAF, ATT, VAL>(
    old_element: &'a Element<NS, TAG, LEAF, ATT, VAL>,
    nodes: &'a [Node<NS, TAG, LEAF, ATT, VAL>],
    path: &[usize],
) -> Vec<Patch<'a, NS, TAG, LEAF, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    vec![Patch::append_children(
        old_element.tag(),
        TreePath::new(path.to_vec()),
        nodes.iter().collect(),
    )]
}

fn push_all_nodes<'a, NS, TAG, LEAF, ATT, VAL>(
    node: &'a Node<NS, TAG, LEAF, ATT, VAL>,
    path: &[usize],
) -> (usize, Vec<Patch<'a, NS, TAG, LEAF, ATT, VAL>>)
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    println!("I don't know what to do with push_all_nodes");
    (0, vec![])
}

fn create_children<'a, NS, TAG, LEAF, ATT, VAL>(
    nodes: &'a [Node<NS, TAG, LEAF, ATT, VAL>],
    path: &[usize],
) -> (usize, Vec<Patch<'a, NS, TAG, LEAF, ATT, VAL>>)
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    (0, vec![])
}

fn replace_inner<'a, NS, TAG, LEAF, ATT, VAL>(
    node: &'a Node<NS, TAG, LEAF, ATT, VAL>,
    n: usize,
    path: &[usize],
) -> Vec<Patch<'a, NS, TAG, LEAF, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    vec![]
}
