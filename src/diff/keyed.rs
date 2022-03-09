use super::{create_attribute_patches, diff_recursive};
use crate::{Element, Node, Patch, TreePath};
use std::collections::BTreeMap;
use std::fmt::Debug;
use std::iter::FromIterator;

/// diff the old element with new element
///
/// example case:
///
///  old            new
///
///  (-) 0              10   (+) (will be inserted at before key3)
///  (-) 1              key5 (+) (will be inserted at before key3, this is not matched at key5, since key3 is matched first and since this new key5 has passed new key3 position)
///  (-) 2       .----- key3 (*) (will not matched old key3 since we've gone past last matched (key5 at 5) }
///  (*) key3 <-'       12   (+) (will be inserted at after key3)
///  (*) key4 <-.       13   (+) (will be inserted at after key3)
///  (-) key5    `----- key4 (*) (will not matched old key4 since we've gone past last matched (key5 at )
///  (-) 6              14   (+) (will be inserted at after key4)
///    * key6 <-------- key6 *
///  (-) 8              16   (+) will be inserted at after key6)
///  (-) 9
///
/// Legend:
/// (-) means will be removed
/// (+) means will be inserted
/// (*) means will be patched
///
/// Algorithm flow:
/// - make a BTreeMap for old index and their old key (old_index_key)
/// - make a BTreeMap for new index and their new key (new_index_key)
///
/// - Use an old_index pointer to 0, this will be used to point to the index of the last matched old key (last_matched_old_index)
/// - Use an new_index pointer to 0, this will be used to point to the index of the last matched new key (last_matched_new_index)
///
/// - from the new_index_key, iterate through the new elements to find which old_index which matched the new key starting from  `last_matched_old_index` until it finds a matched.
///     - old_index_key[last_matched_old_index..]
///     - if a matched is found (old_key == new_key) take node of the `old_index` and `new_index`
///         - create a patch which will delete all the old elements from `old_index` to `last_matched_old_index`, using their own patch_path: [path + old_index].
///         - create a patch which will insert all the new elements from `new_index` to `last_matched_new_index`, using InsertBefore patch_path: [path + last_matched_old_index].
///         - set `last_matched_old_index` to `old_index`
///         - set `last_matched_new_index` to `new_index`
///     - if we have reached the end of the iteration (the last old_index that has a match)
///         - create a patch which will delete all the old_elements from `last_matched_old_index` to the last old elements.
///         - create a patch which will insert all the new_elements from `last_matched_new_index`, using InsertAfter patch_path: [path + last_matched_new_index].
pub fn diff_keyed_elements<'a, 'b, NS, TAG, LEAF, ATT, VAL, SKIP, REP>(
    old_element: &'a Element<NS, TAG, LEAF, ATT, VAL>,
    new_element: &'a Element<NS, TAG, LEAF, ATT, VAL>,
    key: &ATT,
    path: &[usize],
    skip: &SKIP,
    rep: &REP,
) -> Vec<Patch<'a, NS, TAG, LEAF, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Debug,
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
    let mut patches = vec![];

    // make a map of old_index -> old_key
    let old_key_index: BTreeMap<usize, Vec<&VAL>> = BTreeMap::from_iter(
        old_element.children.iter().enumerate().filter_map(
            |(old_index, old)| {
                old.get_attribute_value(key)
                    .map(|old_key| (old_index, old_key))
            },
        ),
    );

    // make a map of new_index -> new_key
    let new_key_index: BTreeMap<usize, Vec<&VAL>> = BTreeMap::from_iter(
        new_element.children.iter().enumerate().filter_map(
            |(new_index, new)| {
                new.get_attribute_value(key)
                    .map(|new_key| (new_index, new_key))
            },
        ),
    );

    // check if there is no match from the keys in new_element to the keys in old_elements
    // if indeed there is no match at all, create a remove_all node and append all children
    let has_match = new_key_index.iter().any(|(new_index, new_key)| {
        old_key_index
            .iter()
            .any(|(old_index, old_key)| new_key == old_key)
    });
    println!("has at least 1 match: {}", has_match);
    // return early if there new no matches
    if !has_match {
        let for_remove_patch = old_element
            .children
            .iter()
            .enumerate()
            .map(|(old_index, old)| {
                let mut old_child_path = TreePath::new(path.to_vec());
                old_child_path.push(old_index);
                Patch::remove_node(old.tag(), old_child_path)
            })
            .collect::<Vec<_>>();

        patches.extend(for_remove_patch);

        let for_append_children =
            new_element.children.iter().collect::<Vec<_>>();
        if !for_append_children.is_empty() {
            let for_append_patch = Patch::append_children(
                old_element.tag(),
                TreePath::new(path.to_vec()),
                for_append_children,
            );
            patches.push(for_append_patch);
        }

        // return early when there is no more matches
        return patches;
    }

    // a pointer to the last matched
    let mut last_matched_old_index = None;
    let mut last_matched_new_index = None;

    // iterate through new elements and find which old element index has the same key in this new
    // key
    for (new_index, new) in new_element.children.iter().enumerate() {
        let new_key = new.get_attribute_value(key);
        println!("new_key: {:?}", new_key);
        let matched_old_index: Option<usize> = if let Some(new_key) = new_key {
            old_key_index.iter().find_map(|(old_index, old_key)| {
                if is_forward(last_matched_old_index, *old_index)
                    && **old_key == new_key
                {
                    Some(*old_index)
                } else {
                    None
                }
            })
        } else {
            None
        };

        // if there is a matching old_index, create a patch that will remove all the nodes
        // from the old_elements from the `last_matched_old_index` to this `matched_old_index
        if let Some(matched_old_index) = matched_old_index {
            dbg!(&matched_old_index);

            let mut old_path = path.to_vec();
            old_path.push(matched_old_index);

            println!("going inside diff_recursive..");

            //dbg!(&old_element.children[matched_old_index]);
            //dbg!(&new);

            let patch_for_matched = diff_recursive(
                &old_element.children[matched_old_index],
                new,
                &old_path,
                key,
                skip,
                rep,
            );

            dbg!(&patch_for_matched);

            patches.extend(patch_for_matched);

            println!(
                "remove nodes from {:?} to {}",
                last_matched_old_index, matched_old_index
            );

            let for_remove_nodes = &old_element
                .children
                .iter()
                .enumerate()
                .filter_map(|(i, old)| {
                    if is_forward(last_matched_old_index, i)
                        && i < matched_old_index
                    {
                        Some(old)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if !for_remove_nodes.is_empty() {
                println!(
                    "remove nodes from {:?} to {}",
                    last_matched_old_index, matched_old_index
                );
                let for_remove_patches = for_remove_nodes
                    .iter()
                    .enumerate()
                    .map(|(i, old)| {
                        let mut old_path = TreePath::new(path.to_vec());
                        old_path.push(last_matched_old_index.unwrap_or(0) + i);
                        Patch::remove_node(old.tag(), old_path)
                    })
                    .collect::<Vec<_>>();

                patches.extend(for_remove_patches);
            }

            //assign this matched_old_index to the last_matched_old_index
            last_matched_old_index = Some(matched_old_index);

            let for_insert_nodes = new_element
                .children
                .iter()
                .enumerate()
                .filter_map(|(i, new)| {
                    if is_forward(last_matched_new_index, i) && i < new_index {
                        Some(new)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();

            if !for_insert_nodes.is_empty() {
                let mut old_path = TreePath::new(path.to_vec());

                let old_index_marked_node = last_matched_old_index.unwrap_or(0);
                old_path.push(old_index_marked_node);

                let old_tag = old_element.children[old_index_marked_node].tag();
                // create a patch that will insert the new elements from `last_matched_new_index` to
                // `new_index`
                let for_insert_patch = Patch::insert_before_node(
                    old_tag,
                    old_path,
                    for_insert_nodes,
                );

                patches.push(for_insert_patch);
            }

            //assign last matched_new_index to the new_index we are iterating on
            last_matched_new_index = Some(new_index);
        } else {
            println!("no matching..");
        }
    }

    // remove what's left in the old_elements after last_matched_old_index
    let remaining_old_nodes = &old_element
        .children
        .iter()
        .enumerate()
        .filter_map(|(i, old)| {
            if is_forward(last_matched_old_index, i) {
                Some(old)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if !remaining_old_nodes.is_empty() {
        println!(
            "remaining old_nodes is not empty... {:?}",
            last_matched_old_index
        );
        let for_remove_patches = remaining_old_nodes
            .iter()
            .enumerate()
            .map(|(i, old)| {
                let mut old_path = TreePath::new(path.to_vec());
                old_path.push(last_matched_old_index.unwrap_or(0) + i);

                Patch::remove_node(old.tag(), old_path)
            })
            .collect::<Vec<_>>();

        patches.extend(for_remove_patches);
    }

    let old_tag = old_element
        .children
        .get(last_matched_old_index.unwrap_or(0))
        .map(|n| n.tag())
        .flatten();
    let mut old_path = TreePath::new(path.to_vec());
    old_path.push(last_matched_old_index.unwrap_or(0));

    // insert all the elements after the last_matched_new_index, insert it before the
    // node at last_matched_old_index
    let remaining_new_nodes = new_element
        .children
        .iter()
        .enumerate()
        .filter_map(|(i, new)| {
            if is_forward(last_matched_new_index, i) {
                Some(new)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    if !remaining_new_nodes.is_empty() {
        println!(
            "remaining_new_nodes is not empty. {:?}",
            last_matched_new_index
        );
        let for_insert_after =
            Patch::insert_after_node(old_tag, old_path, remaining_new_nodes);
        patches.push(for_insert_after);
    }

    patches
}

// check if index is greater than the contained value of an index
fn is_forward(than: Option<usize>, i: usize) -> bool {
    match than {
        None => true,
        Some(than) => i > than,
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::*;

    pub type MyNode = Node<
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        &'static str,
    >;

    #[test]
    fn keyed_test_empty() {
        let old: MyNode = element("div", [], []);
        let new: MyNode = element("div", [], []);
        let patches = diff_keyed_elements(
            old.as_element_ref().unwrap(),
            new.as_element_ref().unwrap(),
            &"key",
            &[],
            &|_, _| false,
            &|_, _| false,
        );
        assert_eq!(patches, vec![]);
    }

    #[test]
    fn keyed_no_difference() {
        let old: MyNode = element("div", [attr("key", "1")], []);
        let new: MyNode = element("div", [attr("key", "1")], []);
        let patches = diff_keyed_elements(
            old.as_element_ref().unwrap(),
            new.as_element_ref().unwrap(),
            &"key",
            &[],
            &|_, _| false,
            &|_, _| false,
        );
        assert_eq!(patches, vec![]);
    }

    #[test]
    fn no_difference_matching_key() {
        let old: MyNode =
            element("main", [], vec![element("div", [attr("key", "1")], [])]);

        let new: MyNode =
            element("main", [], vec![element("div", [attr("key", "1")], [])]);

        let patches = diff_keyed_elements(
            &old.as_element_ref().unwrap(),
            &new.as_element_ref().unwrap(),
            &"key",
            &[],
            &|_, _| false,
            &|_, _| false,
        );
        dbg!(&patches);
        assert_eq!(patches, vec![]);
    }

    #[test]
    fn keyed_not_matched() {
        let old: MyNode =
            element("main", [], vec![element("div", [attr("key", "1")], [])]);

        let new: MyNode =
            element("main", [], vec![element("div", [attr("key", "2")], [])]);

        let patches = diff_keyed_elements(
            &old.as_element_ref().unwrap(),
            &new.as_element_ref().unwrap(),
            &"key",
            &[],
            &|_, _| false,
            &|_, _| false,
        );
        assert_eq!(
            patches,
            vec![
                Patch::remove_node(Some(&"div"), TreePath::new(vec![0])),
                Patch::append_children(
                    &"main",
                    TreePath::new(vec![]),
                    vec![&element("div", [attr("key", "2")], [])]
                )
            ]
        );
    }

    #[test]
    fn keyed_inserted_at_the_end() {
        let old: MyNode =
            element("main", [], vec![element("div", [attr("key", "1")], [])]);

        let new: MyNode = element(
            "main",
            [],
            vec![
                element("div", [attr("key", "1")], []),
                element("div", [attr("key", "2")], []),
            ],
        );

        let patches = diff_keyed_elements(
            &old.as_element_ref().unwrap(),
            &new.as_element_ref().unwrap(),
            &"key",
            &[],
            &|_, _| false,
            &|_, _| false,
        );
        assert_eq!(
            patches,
            vec![Patch::insert_after_node(
                Some(&"div"),
                TreePath::new(vec![0]),
                vec![&element("div", [attr("key", "2")], [])]
            )]
        );
    }

    #[test]
    fn keyed_remove_at_start() {
        let old: MyNode = element(
            "main",
            [],
            vec![
                element("div", [attr("key", "1")], []),
                element("div", [attr("key", "2")], []),
            ],
        );

        let new: MyNode =
            element("main", [], vec![element("div", [attr("key", "2")], [])]);

        let patches = diff_keyed_elements(
            &old.as_element_ref().unwrap(),
            &new.as_element_ref().unwrap(),
            &"key",
            &[],
            &|_, _| false,
            &|_, _| false,
        );
        assert_eq!(
            patches,
            vec![Patch::remove_node(Some(&"div"), TreePath::new(vec![0]),)]
        );
    }

    #[test]
    fn keyed_all_matched() {
        let old: MyNode = element(
            "main",
            [],
            vec![
                element("div", [attr("key", "1")], []),
                element("div", [attr("key", "2")], []),
                element("div", [attr("key", "3")], []),
            ],
        );

        let new: MyNode = element(
            "main",
            [],
            vec![
                element("div", [attr("key", "1")], []),
                element("div", [attr("key", "2")], []),
                element("div", [attr("key", "3")], []),
            ],
        );

        let patches = diff_keyed_elements(
            &old.as_element_ref().unwrap(),
            &new.as_element_ref().unwrap(),
            &"key",
            &[],
            &|_, _| false,
            &|_, _| false,
        );

        dbg!(&patches);

        assert_eq!(patches, vec![]);
    }

    #[test]
    fn keyed_inserted_at_the_middle() {
        let old: MyNode = element(
            "main",
            [],
            vec![
                element("div", [attr("key", "1")], []),
                element("div", [attr("key", "3")], []),
            ],
        );

        let new: MyNode = element(
            "main",
            [],
            vec![
                element("div", [attr("key", "1")], []),
                element("div", [attr("key", "2")], []),
                element("div", [attr("key", "3")], []),
            ],
        );

        let patches = diff_keyed_elements(
            &old.as_element_ref().unwrap(),
            &new.as_element_ref().unwrap(),
            &"key",
            &[],
            &|_, _| false,
            &|_, _| false,
        );

        dbg!(&patches);

        assert_eq!(
            patches,
            vec![Patch::insert_before_node(
                Some(&"div"),
                TreePath::new(vec![1]),
                vec![&element("div", [attr("key", "2")], [])]
            )]
        );
    }

    #[test]
    fn keyed_multiple_matches_start_and_end() {
        let old: MyNode = element(
            "main",
            [],
            vec![
                element("div", [attr("key", "1")], []),
                element("div", [attr("key", "2")], []),
                element("div", [attr("key", "3")], []),
                element("div", [attr("key", "4")], []),
            ],
        );

        let new: MyNode = element(
            "main",
            [],
            vec![
                element("div", [attr("key", "1")], []),
                element("div", [attr("key", "20")], []),
                element("div", [attr("key", "30")], []),
                element("div", [attr("key", "4")], []),
            ],
        );

        let patches = diff_keyed_elements(
            &old.as_element_ref().unwrap(),
            &new.as_element_ref().unwrap(),
            &"key",
            &[],
            &|_, _| false,
            &|_, _| false,
        );

        dbg!(&patches);

        assert_eq!(
            patches,
            vec![
                Patch::remove_node(Some(&"div"), TreePath::new(vec![0])),
                Patch::remove_node(Some(&"div"), TreePath::new(vec![1])),
                Patch::insert_before_node(
                    Some(&"div"),
                    TreePath::new(vec![3]),
                    vec![
                        &element("div", [attr("key", "20")], []),
                        &element("div", [attr("key", "30")], [])
                    ]
                )
            ]
        );
    }
}
