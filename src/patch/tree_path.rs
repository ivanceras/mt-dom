use crate::Node;
use std::fmt::Debug;

/// Describe the path traversal of a Node starting from the root node
///
/// The figure below shows `node_idx` in a depth first traversal.
///
/// ```text
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
/// ```text
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
/// ```text
///    0 = [0]
///    1 = [0,0]
///    2 = [0,0,0]
///    3 = [0,0,1]
///    4 = [0,1]
///    5 = [0,1,0]
///    6 = [0,1,1]
///    7 = [0,1,2]
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct TreePath {
    /// an alternative path vector, where it specifies
    /// the first element is the index of the root node which is always 0
    /// the second element is the index of the child to traverse to and so on.
    /// Given a DOM tree where `node_idx` and `path` was derived from, we can
    /// verify that node_idx and path point to the same node.
    /// The advantage of using this is that this doesn't need to traverse nodes that are not
    /// relevant. Traversal operation complexity is O(log n)
    pub path: Vec<usize>,
}

impl TreePath {
    /// create a TreePath with node index `node_idx` and traversal path `path`
    pub fn new(path: Vec<usize>) -> Self {
        Self { path }
    }

    /// add a path node idx
    pub fn push_path(&mut self, node_idx: usize) {
        self.path.push(node_idx)
    }

    /// find the node using the path of this tree path
    pub fn find_node_by_path<'a, NS, TAG, ATT, VAL>(
        &self,
        node: &'a Node<NS, TAG, ATT, VAL>,
    ) -> Option<&'a Node<NS, TAG, ATT, VAL>>
    where
        NS: PartialEq + Clone + Debug,
        TAG: PartialEq + Clone + Debug,
        ATT: PartialEq + Clone + Debug,
        VAL: PartialEq + Clone + Debug,
    {
        find_node_by_path(node, &self)
    }
}

fn traverse_node_by_path<'a, NS, TAG, ATT, VAL>(
    node: &'a Node<NS, TAG, ATT, VAL>,
    path: &mut TreePath,
) -> Option<&'a Node<NS, TAG, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    println!("\n Traversing path: {:?}", path);
    if path.path.is_empty() {
        Some(node)
    } else if let Some(children) = node.get_children() {
        let idx = path.path.remove(0);
        println!("\t idx to see: {}", idx);
        if let Some(child) = &children.get(idx) {
            traverse_node_by_path(child, path)
        } else {
            None
        }
    } else {
        None
    }
}

fn find_node_by_path<'a, NS, TAG, ATT, VAL>(
    node: &'a Node<NS, TAG, ATT, VAL>,
    path: &TreePath,
) -> Option<&'a Node<NS, TAG, ATT, VAL>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    let mut path = path.clone();
    let root_idx = path.path.remove(0); // remove the first 0
    assert_eq!(0, root_idx, "path must start with 0");
    traverse_node_by_path(node, &mut path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    type MyNode = Node<&'static str, &'static str, &'static str, &'static str>;

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
        path: Vec<usize>,
    ) {
        let id = node.get_attribute_value(&"id").unwrap()[0];
        let class = node.get_attribute_value(&"class").unwrap()[0];
        println!("\tid: {:?} class: {:?}", id, class);
        println!("\tnode_idx: {} = {}", node_idx, format_vec(&path));
        assert_eq!(id.to_string(), node_idx.to_string());
        assert_eq!(class.to_string(), format_vec(&path));
        if let Some(children) = node.get_children() {
            for (i, child) in children.iter().enumerate() {
                *node_idx += 1;
                let mut child_path = path.clone();
                child_path.push(i);
                assert_traverse_match(child, node_idx, child_path);
            }
        }
    }

    fn traverse_tree_path(
        node: &MyNode,
        path: &TreePath,
        node_idx: &mut usize,
    ) {
        let id = node.get_attribute_value(&"id").unwrap()[0];
        let class = node.get_attribute_value(&"class").unwrap()[0];
        println!("\tid: {:?} class: {:?}", id, class);
        println!("\tnode_idx: {} = {}", node_idx, format_vec(&path.path));
        assert_eq!(id.to_string(), node_idx.to_string());
        assert_eq!(class.to_string(), format_vec(&path.path));
        if let Some(children) = node.get_children() {
            for (i, child) in children.iter().enumerate() {
                *node_idx += 1;
                let mut child_path = path.clone();
                child_path.path.push(i);
                traverse_tree_path(child, &child_path, node_idx);
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
        assert_traverse_match(&node, &mut 0, vec![0]);
        traverse_tree_path(&node, &TreePath::new(vec![0]), &mut 0);
    }

    #[test]
    fn should_find_root_node() {
        let node = sample_node();
        let path = TreePath::new(vec![0]);
        let root = find_node_by_path(&node, &path);
        dbg!(&root);
        assert_eq!(Some(&node), root);
    }

    #[test]
    fn should_find_node1() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 0]);
        let found = find_node_by_path(&node, &path);
        dbg!(&found);
        let expected = element(
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
        );
        assert_eq!(Some(&expected), found);
    }

    #[test]
    fn should_find_node2() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 0, 0]);
        let found = find_node_by_path(&node, &path);
        dbg!(&found);
        let expected = element(
            "div",
            vec![attr("class", "[0,0,0]"), attr("id", "2")],
            vec![],
        );
        assert_eq!(Some(&expected), found);
    }

    #[test]
    fn should_find_node3() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 0, 1]);
        let found = find_node_by_path(&node, &path);
        dbg!(&found);
        let expected = element(
            "div",
            vec![attr("class", "[0,0,1]"), attr("id", "3")],
            vec![],
        );
        assert_eq!(Some(&expected), found);
    }

    #[test]
    fn should_find_node4() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 1]);
        let found = find_node_by_path(&node, &path);
        dbg!(&found);
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
        assert_eq!(Some(&expected), found);
    }

    #[test]
    fn should_find_node5() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 1, 0]);
        let found = find_node_by_path(&node, &path);
        dbg!(&found);
        let expected = element(
            "div",
            vec![attr("class", "[0,1,0]"), attr("id", "5")],
            vec![],
        );
        assert_eq!(Some(&expected), found);
    }

    #[test]
    fn should_find_node6() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 1, 1]);
        let found = find_node_by_path(&node, &path);
        dbg!(&found);
        let expected = element(
            "div",
            vec![attr("class", "[0,1,1]"), attr("id", "6")],
            vec![],
        );
        assert_eq!(Some(&expected), found);
    }

    #[test]
    fn should_find_none_in_013() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 1, 3]);
        let found = find_node_by_path(&node, &path);
        dbg!(&found);
        assert_eq!(None, found);
    }

    #[test]
    fn should_find_none_in_00000() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 0, 0, 0]);
        let found = find_node_by_path(&node, &path);
        dbg!(&found);
        assert_eq!(None, found);
    }

    #[test]
    fn should_find_none_in_007() {
        let node = sample_node();
        let path = TreePath::new(vec![0, 0, 7]);
        let bond = find_node_by_path(&node, &path);
        dbg!(&bond);
        assert_eq!(None, bond);
    }
}
