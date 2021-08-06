use crate::Node;
use std::fmt::Debug;

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

fn traverse_node<'a, NS, TAG, ATT, VAL, EVENT>(
    node: &'a Node<NS, TAG, ATT, VAL, EVENT>,
    path: &mut Vec<usize>,
) -> Option<&'a Node<NS, TAG, ATT, VAL, EVENT>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    println!("\n Traversing path: {:?}", path);
    if path.is_empty() {
        Some(node)
    } else if let Some(children) = node.get_children() {
        let idx = path.remove(0);
        println!("\t idx to see: {}", idx);
        let child = &children[idx];
        traverse_node(&children[idx], path)
    } else {
        None
    }
}

fn find_node_by_path<'a, NS, TAG, ATT, VAL, EVENT>(
    node: &'a Node<NS, TAG, ATT, VAL, EVENT>,
    path: &[usize],
) -> Option<&'a Node<NS, TAG, ATT, VAL, EVENT>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    let mut path = path.to_vec();
    path.remove(0); // remove the first 0
    traverse_node(node, &mut path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    type MyNode =
        Node<&'static str, &'static str, &'static str, &'static str, ()>;

    fn sample_node() -> MyNode {
        let node: MyNode = element(
            "div",
            vec![attr("class", "[0]"), attr("id", "0")],
            vec![
                element(
                    "div",
                    vec![attr("class", "[0,0]"), attr("id", "1")],
                    vec![
                        element(
                            "div",
                            vec![attr("class", "[0,0,0]"), attr("id", "2")],
                            vec![],
                        ),
                        element(
                            "div",
                            vec![attr("class", "[0,0,1]"), attr("id", "3")],
                            vec![],
                        ),
                    ],
                ),
                element(
                    "div",
                    vec![attr("class", "[0,1]"), attr("id", "4")],
                    vec![
                        element(
                            "div",
                            vec![attr("class", "[0,1,0]"), attr("id", "5")],
                            vec![],
                        ),
                        element(
                            "div",
                            vec![attr("class", "[0,1,1]"), attr("id", "6")],
                            vec![],
                        ),
                        element(
                            "div",
                            vec![attr("class", "[0,1,2]"), attr("id", "7")],
                            vec![],
                        ),
                    ],
                ),
            ],
        );
        node
    }

    // index is the index of this code with respect to it's sibling
    fn assert_traverse_match(
        node: &MyNode,
        node_idx: &mut usize,
        index: usize,
        path: Vec<usize>,
    ) {
        let id = node.get_attribute_value(&"id").unwrap()[0];
        let class = node.get_attribute_value(&"class").unwrap()[0];
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
        let node = sample_node();
        assert_traverse_match(&node, &mut 0, 0, vec![0]);
    }

    #[test]
    fn should_find_root_node() {
        let node = sample_node();

        let root = super::find_node_by_path(&node, &[0]);
        dbg!(&root);
        assert_eq!(Some(&node), root);
    }

    #[test]
    fn should_find_node4() {
        let node = sample_node();
        let node4 = super::find_node_by_path(&node, &[0, 1]);
        dbg!(&node4);
        let expected = element(
            "div",
            vec![attr("class", "[0,1]"), attr("id", "4")],
            vec![
                element(
                    "div",
                    vec![attr("class", "[0,1,0]"), attr("id", "5")],
                    vec![],
                ),
                element(
                    "div",
                    vec![attr("class", "[0,1,1]"), attr("id", "6")],
                    vec![],
                ),
                element(
                    "div",
                    vec![attr("class", "[0,1,2]"), attr("id", "7")],
                    vec![],
                ),
            ],
        );
        assert_eq!(Some(&expected), node4);
    }

    #[test]
    fn should_find_node7() {
        let node = sample_node();
        let node7 = super::find_node_by_path(&node, &[0, 1, 2]);
        dbg!(&node7);
        let expected = element(
            "div",
            vec![attr("class", "[0,1,2]"), attr("id", "7")],
            vec![],
        );
        assert_eq!(Some(&expected), node7);
    }
}
