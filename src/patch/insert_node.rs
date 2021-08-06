use super::NodeIdx;
use crate::Node;
use std::fmt;
use std::fmt::Debug;

/// InsertNode patch contains the a node to insert into
#[derive(Clone, Debug, PartialEq)]
pub struct InsertNode<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// the tag of the target node to be inserted
    pub tag: Option<&'a TAG>,
    /// the target node_idx of which our node will be inserted before it.
    pub node_idx: NodeIdx,
    /// the new node_idx of this newly inserted node.
    /// this will be inserted into a fast lookup of NodeIdx to get
    /// the referenced node faster than having to traverse it.
    pub new_node_idx: NodeIdx,
    /// the node to be inserted
    pub node: &'a Node<NS, TAG, ATT, VAL, EVENT>,
}
impl<'a, NS, TAG, ATT, VAL, EVENT> InsertNode<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// create a new InsertNode patch
    pub fn new(
        tag: Option<&'a TAG>,
        node_idx: NodeIdx,
        new_node_idx: NodeIdx,
        node: &'a Node<NS, TAG, ATT, VAL, EVENT>,
    ) -> Self {
        InsertNode {
            tag,
            node_idx,
            new_node_idx,
            node,
        }
    }
}
