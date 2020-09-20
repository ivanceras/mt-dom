# Changelog

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

