# TODO
- [ ] Implement applying patches the mt-dom to for the purpose of verifying
    if it produces the same DOM tree as in the browser.
- [ ] Change callback to use &'a lifetime instead of 'static
- [ ] Use associated type rather than just all Generics to simplify the code.
- [X] Modularize Patch
    - [X] Create a struct of each of the variants
- [ ] Add a `self_closing` flag to element to be able to properly render self closing elements such as `<input />`, `<br/>` etc.
    - this is needed for the apply_patch and render trait in sauron to match the exact browser html output.
