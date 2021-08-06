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
    pub fn child(mut self, index: usize) -> Zipper<NS, TAG, ATT, VAL, EVENT> {
        // Remove the specified child from the node's children.
        // A Zipper shouldn't let its users inspect its parent,
        // since we mutate the parents
        // to move the focused nodes out of their list of children.
        // We use swap_remove() for efficiency.
        let child = self.node.swap_remove_child(index);

        // Return a new Zipper focused on the specified child.
        Zipper {
            node: child,
            parent: Some(Box::new(self)),
            index_in_parent: index,
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
