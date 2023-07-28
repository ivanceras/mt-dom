# Changelog

## 0.57.0
- feat: **breaking** `Node::children` now returns a slice, ie: `&[Node]` instead of `Option<&[Node]>`,
   non-element node variant will return an empty slice automatically.

## 0.56.1
- fix: and improve performance in `diff_lis::diff_keyed_middle`, by iterating only up to `n` instead of `n^2`
    - [76c7d](https://github.com/ivanceras/mt-dom/commit/76c7de7dc9709e8d78327da3f199e4d225674af4)

## 0.56.0

- fragile fix: change the order for the generated patch, `before_node` should come first than `after_node` as it is desctructive and order is important to that variant
- refactor: rename `test/diff_lis.rs` to `test/test_diff_lis.rs` so as not to confused with filenames with the actual algorithm code
- feat: add `Patch::node_paths` method to return the paths for the nodes that are for_moving
- fix: a more streamline, and logical algorithm for the the patch variant
- refactor: create a precomputed keys for old and new children so as not to do it all over again
- feat: add 2 new Patch Variant `MoveBeforeNode` and `MoveAfterNode`
    - this is needed for patches that just the matching element from the old to the new in the same tree.
- make `Patch::insert_before_node` and `Patch::insert_after_node` patch more flexible by using `impl IntoIterator` in the arg.
- feat: make TreePath and patches argument more flexible to accept array or vec

## 0.55.2
- remove unused dep

## 0.55.1
- feat: add a huge performance benefits by checking if the nodes, elements and attributes are the same, before proceeding to make a diff

## 0.55.0
- Change multiple method names
   - node.rs Node
       - as_leaf_ref -> leaf
       - as_element_mut -> element_mut
       - as_element_ref -> element_ref
       - add_children -> with_children
       - add_children_ref_mut -> add_children
       - add_attributes -> with_attributes
       - add_attributes_ref_mut -> add_attributes
       - get_attributes -> attributes
       - get_children -> children
       - get_children_count -> children_count
       - set_attributes_ref_mut -> set_attributes
       - get_attribute_value -> attribute_value
   - element.rs Element
       - get_children -> children
       - get_attributes -> attributes
       - get_attribute_value -> attribute_value

## 0.54.1
- Some clippy improvements, and removing unused variables

## 0.54.0
- bump up the version to align with sauron
- use a better algorithm where longest increasing subsequence is considered
- comment out test that don't fit with the new algorithm anymore
- `Patch::replace_node` uses vec of nodes for the replacement nodes.
- convert ALLCAPS generic type to Capitalize
- use edition 2021


## 0.22.0
- Refactor Patch such that, tag and patch_path is the same field in a struct
    while the different types is in an enum variant PatchType, this minimized the use of repeated fields

## 0.21.1
- clippy fixes
- remove feature `with-measure` as it is not used anymore

## 0.21.0
- Add `NodeList` as variant of a node which is used for view functions which needs to return multiple nodes

## 0.20.0
- **Breaking** Remove internal api `Patch::insert_ndoe` in favor of `Patch::insert_before_node`

## 0.19.4
- add `as_leaf_ref` method for `Node`.

## 0.19.3
- documentation correction for `diff_keyed_elements`

## 0.19.2
- add `pluck` method in TreePath, which is a more concise name than `remove_first` when used in traversing the actual dom.

## 0.19.1
- Code cleanup and refactoring.
- Uses TreePath to be passed around in diffing functions.
    - Add convenient methods in TreePath

## 0.19.0
- Change algorithm for diffing keyed elements this time with a more stream-line and shorter code

## 0.18.0
- Remove the Patch::ReplaceLeaf variant as it can be expressed using Patch::ReplaceNode

## 0.17.0
- return TreePath object from Patch::path method

## 0.16.0
- TreePath starts empty vec![] as the path as its corresponding root node (previously it starts at vec![0])

## 0.15.0
- Added InsertBeforeNode and InsertAfterNode variant for path, removed InsertNode as it is superseeded by InsertBeforeNode

## 0.14.0
- Make mt-dom more generic by using a generic variant for the leaf node value.

## 0.13.0
- refactor: collapse the Patches variant into enum struct to make it more compact and coherent in one place

## 0.12.3
- More conversion of IntoIterator

## 0.12.2
- Use impl IntoIterator to allow array or vec of attributes and children.
    This provides a much cleaner syntax when building a view code.

## 0.12.1
- Remove stray `log::trace`

## 0.12.0
- **breaking** Make `Comment` a variant of `Node`, so we can move the hacks to the DOM tree rather than in the apply_patches
- We'll git back to the rearranged keyed-node bug in the future
- rename cur_path to path and cur_child_path to child_path since we remove new_path and new_child_path already
- Make clippy happy

## 0.11.1
- doc-comment updates
- removal of unused-code

## 0.11.0
- **breaking** Remove the use of `node_idx` and use `TreePath` alone
- Simplify the code in `diff_keyed_elements`
- **breaking** Remove unused `new_path`.

## 0.10.0
- Added `safe_html` to `Text` node, this indicated whether to render as text node or as innerHTML of its parent element.
- Remove function `increment_node_idx_to_descendant_count` and use a more elegent method in Node `descendant_node_count`
    - modify count_recursive to not use mutable counter passed around
    - Improve code for node_counting
- Remove the use of NodeIdx, just use usize

# 0.9.0
- Remove `old_path` and `PatchPath` and use only `TreePath` which describes the path traversal for the target DOM element
- Remove `AttValue`, so we can only deal with `VAL` which is the `AttributeValue` from sauron, which contains simple values and Callbacks

## 0.8.0
- refactor `diff` and `diff::keyed_elements` splitting it into small functions.
    - minimize the use of mutable collections
    - rename variables to appropriate names
    - Fix AppendChildren patch to also add to already_inserted when creating the patch
    - Use consistent naming of variables as in diff for keyed_elements
- Change tag line to `mt-dom`
- **breaking** Refactor `Patch` to make use of `PatchPath` and `TreePath` as an alternative method to using `NodeIdx` to traverse the DOM.
- Add `Zipper` implementation to traverse the `Node`.

## 0.7.1
- Remove warnings

## 0.7.0
 - **breaking** Move Callback to sauron and mt-dom will focus solely on vdom diffing.
 - modify the diffing algorithmn to replace a node when the key didn't matched

## 0.6.0
- Add a REP for explicitly replace the node when REP evaluates to true
- Patch variant now contains NodeIdx for the new node inserted, appended, and the replacement node
- Convert Text variant into a struct
- Add skip function as an optimization feature for diffing old and new node

## 0.5.3
- Fix ReplaceNode tag to be optional, for replacing text node

## 0.5.2
- Fix bug on attributes not diffed in keyed_elements

## 0.5.1
- Fix bug on AppendChildren node_idx

## 0.5.0
- Overhaul the algorithmn for the diffing keyed elements.
- Change InsertChildren into InsertNode
- Change RemoveChildren into RemoveNode
- Add apply_patches for mt-dom for verifying the patch produces the target vdom
- Add Debug contraint for NS,ATT and VAL for easier debugging
- Add a self closing flag in Element
- Improve the diffing algorithm for keyed elements to accomodate included elements that are not keyed
- Add with-measure feature to see what's happenning inside the code

## 0.4.1
- Add utility `node_count` function to count the number of nodes from the node tree
- Add a function `add_children_ref_mut` to `Node` for node to add a children using a mutable reference to self
- Add `map_msg` function for `Attribute`

## 0.4.0
- Fix get_attribute_value to return all the values of the attributes which matched the name, as opposed to only returning the first match
- Add test case for multiple calls to the same attribute should be included in the patch even if only 1 changed in the same attribute name
- Add a children_mut method to return a mutable reference to the children of the node
- Group the attributes by name before comparing

## 0.3.0
- Add note limitation on Callback
- Use a reconciliation algorithm to try match keyed elements
- TruncateChildren in Patches since it is replaced with RemoveChildren
- Implement a manual debug for Node, and Element

## 0.2.2
- Add utility function to merge to existing attributes of the same name
- Add a function merge_attributes to specifically find for existing attributes of an element and merge it

## 0.2.1
- revise the implementation of diff, not needing the merge attributes of the same name, since it adds a performance penalty
- constructing the nodes should use the utility to make multiple values of attributes aggregated right from building of the virtual dom

