use crate::Node;
use std::fmt::Debug;

/// A zipper is a technique of representing an aggregate data structure so that it is convenient for writing programs
/// that traverse the structure arbitrarily and update its contents
#[derive(Clone, Debug, PartialEq)]
pub struct Zipper<NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    node: Node<NS, TAG, ATT, VAL, EVENT>,
    parent: Option<Box<Zipper<NS, TAG, ATT, VAL, EVENT>>>,
    index_in_parent: usize,
}

impl<NS, TAG, ATT, VAL, EVENT> Zipper<NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    pub fn child(
        mut self,
        index: usize,
    ) -> Option<Zipper<NS, TAG, ATT, VAL, EVENT>> {
        // Remove the specified child from the node's children.
        // A Zipper shouldn't let its users inspect its parent,
        // since we mutate the parents
        // to move the focused nodes out of their list of children.
        // We use swap_remove() for efficiency.
        if let Some(children) = self.node.get_children() {
            if children.get(index).is_some() {
                let child = self.node.swap_remove_child(index);

                // Return a new Zipper focused on the specified child.
                Some(Zipper {
                    node: child,
                    parent: Some(Box::new(self)),
                    index_in_parent: index,
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn parent(self) -> Zipper<NS, TAG, ATT, VAL, EVENT> {
        // Destructure this Zipper
        let Zipper {
            node,
            parent,
            index_in_parent,
        } = self;

        // Destructure the parent Zipper
        let Zipper {
            node: mut parent_node,
            parent: parent_parent,
            index_in_parent: parent_index_in_parent,
        } = *parent.unwrap();

        // Insert the node of this Zipper back in its parent.
        // Since we used swap_remove() to remove the child,
        // we need to do the opposite of that.
        parent_node.add_children_ref_mut(vec![node]);
        let len = parent_node.get_children_count();
        parent_node.swap_children(index_in_parent, len - 1);

        // Return a new Zipper focused on the parent.
        Zipper {
            node: parent_node,
            parent: parent_parent,
            index_in_parent: parent_index_in_parent,
        }
    }

    pub fn finish(mut self) -> Node<NS, TAG, ATT, VAL, EVENT> {
        while let Some(_) = self.parent {
            self = self.parent();
        }

        self.node
    }
}

impl<NS, TAG, ATT, VAL, EVENT> Node<NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// create a Zipper for a Node
    pub fn zipper(self) -> Zipper<NS, TAG, ATT, VAL, EVENT> {
        Zipper {
            node: self,
            parent: None,
            index_in_parent: 0,
        }
    }
}

fn zipper_traverse_node<NS, TAG, ATT, VAL, EVENT>(
    node: Node<NS, TAG, ATT, VAL, EVENT>,
    path: &mut Vec<usize>,
) -> Option<Node<NS, TAG, ATT, VAL, EVENT>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    if path.is_empty() {
        Some(node)
    } else {
        let zipper = node.zipper();
        let idx = path.remove(0);
        if let Some(child_zipper) = zipper.child(idx) {
            zipper_traverse_node(child_zipper.node, path)
        } else {
            None
        }
    }
}

fn find_node_by_zipper<NS, TAG, ATT, VAL, EVENT>(
    node: Node<NS, TAG, ATT, VAL, EVENT>,
    path: &[usize],
) -> Option<Node<NS, TAG, ATT, VAL, EVENT>>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    let mut path = path.to_vec();
    let root_idx = path.remove(0); // remove the first 0
    assert_eq!(0, root_idx, "path must start with 0");
    let node = node.clone();
    zipper_traverse_node(node, &mut path)
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

    #[test]
    fn should_traverse_correctly() {
        let node = sample_node();
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
        assert_eq!(Some(expected), find_node_by_zipper(node, &[0, 0]));
    }

    #[test]
    fn should_find_root_node() {
        let node = sample_node();

        let root = find_node_by_zipper(node.clone(), &[0]);
        dbg!(&root);
        assert_eq!(Some(node), root);
    }

    #[test]
    fn should_find_node1() {
        let node = sample_node();
        let found = find_node_by_zipper(node, &[0, 0]);
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
        assert_eq!(Some(expected), found);
    }

    #[test]
    fn should_find_node2() {
        let node = sample_node();
        let found = find_node_by_zipper(node, &[0, 0, 0]);
        dbg!(&found);
        let expected = element(
            "div",
            vec![attr("class", "[0,0,0]"), attr("id", "2")],
            vec![],
        );
        assert_eq!(Some(expected), found);
    }

    #[test]
    fn should_find_node3() {
        let node = sample_node();
        let found = find_node_by_zipper(node, &[0, 0, 1]);
        dbg!(&found);
        let expected = element(
            "div",
            vec![attr("class", "[0,0,1]"), attr("id", "3")],
            vec![],
        );
        assert_eq!(Some(expected), found);
    }

    #[test]
    fn should_find_node4() {
        let node = sample_node();
        let node4 = find_node_by_zipper(node, &[0, 1]);
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
        assert_eq!(Some(expected), node4);
    }

    #[test]
    fn should_find_node5() {
        let node = sample_node();
        let node5 = find_node_by_zipper(node, &[0, 1, 0]);
        dbg!(&node5);
        let expected = element(
            "div",
            vec![attr("class", "[0,1,0]"), attr("id", "5")],
            vec![],
        );
        assert_eq!(Some(expected), node5);
    }

    #[test]
    fn should_find_node6() {
        let node = sample_node();
        let node6 = find_node_by_zipper(node, &[0, 1, 1]);
        dbg!(&node6);
        let expected = element(
            "div",
            vec![attr("class", "[0,1,1]"), attr("id", "6")],
            vec![],
        );
        assert_eq!(Some(expected), node6);
    }

    #[test]
    fn should_find_node7() {
        let node = sample_node();
        let node7 = find_node_by_zipper(node, &[0, 1, 2]);
        dbg!(&node7);
        let expected = element(
            "div",
            vec![attr("class", "[0,1,2]"), attr("id", "7")],
            vec![],
        );
        assert_eq!(Some(expected), node7);
    }

    #[test]
    fn should_find_none_in_013() {
        let node = sample_node();
        let found = find_node_by_zipper(node, &[0, 1, 3]);
        dbg!(&found);
        assert_eq!(None, found);
    }

    #[test]
    fn should_find_none_in_00000() {
        let node = sample_node();
        let found = find_node_by_zipper(node, &[0, 0, 0, 0]);
        dbg!(&found);
        assert_eq!(None, found);
    }

    #[test]
    fn should_find_none_007() {
        let node = sample_node();
        assert_eq!(None, find_node_by_zipper(node, &[0, 0, 7]));
    }
}
