/// Describe the path traversal of a Node starting from the root node
///
/// The figure below shows `node_idx` in a depth first traversal.
///
/// ```ignore
///            .─.
///           ( 0 )
///            `-'
///           /   \
///          /     \
///         /       \
///        ▼         ▼
///       .─.         .─.
///      ( 1 )       ( 4 )
///       `-'         `-'
///      /  \          | \ '.
///     /    \         |  \  '.
///    ▼      ▼        |   \   '.
///  .─.      .─.      ▼    ▼     ▼
/// ( 2 )    ( 3 )    .─.   .─.   .─.
///  `─'      `─'    ( 5 ) ( 6 ) ( 7 )
///                   `─'   `─'   `─'
/// ```
///
/// The figure below shows the index of each child node relative to their parent node
///
/// ```ignore
///             .─.
///            ( 0 )
///             `-'
///            /   \
///           /     \
///          /       \
///         ▼         ▼
///        .─.         .─.
///       ( 0 )       ( 1 )
///        `-'         `-'
///       /  \          | \ '.
///      /    \         |  \  '.
///     ▼      ▼        |   \   '.
///   .─.      .─.      ▼    ▼     ▼
///  ( 0 )    ( 1 )    .─.   .─.   .─.
///   `─'      `─'    ( 0 ) ( 1 ) ( 2 )
///                    `─'   `─'   `─'
/// ```
/// The equivalent idx and path are as follows:
/// ```ignore
///    0 = [0]
///    1 = [0,0]
///    2 = [0,0,0]
///    3 = [0,0,1]
///    4 = [0,1]
///    5 = [0,1,0]
///    6 = [0,1,1]
///    7 = [0,1,2]
/// ```
pub struct TreePath {
    /// The `node_idx` is the index when traversing the DOM tree depth first.
    /// 0 is the root node, 1 is the first child of the root node, 2 is the first child of first
    ///  child
    pub node_idx: usize,
    /// an alternative path vector, where it specifies
    /// the first element is the index of the root node which is always 0
    /// the second element is the index of the child to traverse to and so on.
    /// Given a DOM tree where `node_idx` and `path` was derived from, we can
    /// verify that node_idx and path point to the same node.
    /// The advantage of using this is that this doesn't need to traverse nodes that are not
    /// relevant. Traversal operation complexity is O(log n)
    pub path: Vec<usize>,
}

/// path of this patch
pub struct PatchPath {
    pub old_path: TreePath,
    pub new_path: TreePath,
}

impl TreePath {
    pub fn new(node_idx: usize, path: Vec<usize>) -> Self {
        Self { node_idx, path }
    }
}

impl PatchPath {
    pub fn new(old_path: TreePath, new_path: TreePath) -> Self {
        Self { old_path, new_path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sauron::prelude::*;

    // index is the index of this code with respect to it's sibling
    fn assert_traverse_match(
        node: &Node<()>,
        node_idx: &mut usize,
        index: usize,
        path: Vec<usize>,
    ) {
        let id = node.get_attribute_value(&"id").unwrap()[0]
            .get_simple()
            .unwrap();
        let class = node.get_attribute_value(&"class").unwrap()[0]
            .get_simple()
            .unwrap();
        println!("\tid: {:?} class: {:?}", id, class);
        println!("\tnode_idx: {} = {}", node_idx, format_vec(&path));
        assert_eq!(id.to_string(), node_idx.to_string());
        assert_eq!(class.to_string(), format_vec(&path));
        if let Some(children) = node.get_children() {
            let children_len = children.len();
            for (i, child) in children.iter().enumerate() {
                *node_idx += 1;
                let mut child_path = path.clone();
                child_path.push(i);
                assert_traverse_match(child, node_idx, i, child_path);
            }
        }
    }

    fn format_vec(v: &[usize]) -> String {
        format!(
            "[{}]",
            v.iter()
                .map(|v| v.to_string())
                .collect::<Vec<_>>()
                .join(",")
        )
    }

    #[test]
    fn should_match_paths() {
        let node: Node<()> = node! {
            <div id="0" class="[0]">
                <div id="1" class="[0,0]">
                    <div id="2" class="[0,0,0]"/>
                    <div id="3" class="[0,0,1]"/>
                </div>
                <div id="4" class="[0,1]">
                    <div id="5" class="[0,1,0]"/>
                    <div id="6" class="[0,1,1]"/>
                    <div id="7" class="[0,1,2]"/>
                </div>
            </div>
        };
        assert_traverse_match(&node, &mut 0, 0, vec![0]);
    }
}
