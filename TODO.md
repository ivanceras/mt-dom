# TODO
- [X] Implement applying patches the mt-dom to for the purpose of verifying
    if it produces the same DOM tree as in the browser.
- [ ] Change callback to use &'a lifetime instead of 'static
- [ ] Use associated type rather than just all Generics to simplify the code.
- [X] Modularize Patch
    - [X] Create a struct of each of the variants
- [X] Add a `self_closing` flag to element to be able to properly render self closing elements such as `<input />`, `<br/>` etc.
    - this is needed for the apply_patch and render trait in sauron to match the exact browser html output.
- [X] Remove the use of target_index usize index for InsertChildren and RemoveChildren
    - For InsertChildren, the NodeIdx will be the node after the insertion point
    - For RemoveChildren, the NodeIdx will be the actual NodeIdx of the node to be removed
- [X] Deprecate RemoveChildren with RemoveNode
- [X] Deprecate InsertChildren with InsertNode
- [X] Add skip mechanism to skip diffing nodes marked with this.
- [X] Make the Node::Text variant to be a struct.
    - This is pre-requisite for adding additional fields such as real dom link
- [ ] Add a field `link` for Element and TextNode which points
    to the actual dom when it is created. This will be used directly for patching
    instead of using the `NodeIdx` traversal in patches which has a 0(n) complexity
    and take 40ms to update in a dom tree with 2k nodes.
     - [ ] Patch will now contain the real dom Node, so applying will not have to search for it.
        - Issue: can not link the real dom, since it requires a mutable reference to the patches
        which will have numerous mutable references which is impossible to do.
- [ ] Make the `key` a closure like `skip`.
- [ ] Move `Callback` into sauron.
- [ ] Move algorithmns to sauron such as `map_msg` since it handles the Callback
    - which is supposed to be in sauron

## Optimization
- Create a data structure which has old_element and its node_idx and the new_element with its node_idx
 that way, referencing to a node with the node_idx is very straigh forward way to diff.
