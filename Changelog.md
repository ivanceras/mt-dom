# Changelog

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

