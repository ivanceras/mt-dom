# Changelog

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

