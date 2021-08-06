use super::NodeIdx;
use crate::Node;
use std::fmt::Debug;

/// Replace a node with another node. This typically happens when a node's tag changes.
/// ex: <div> becomes <span>
#[derive(Clone, Debug, PartialEq)]
pub struct ReplaceNode<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// the tag of the node we are going to replace
    /// can replace text node, and text node doesn't have tags
    pub tag: Option<&'a TAG>,
    /// the index of the node we are going to replace
    pub node_idx: NodeIdx,
    /// the node_idx of the replacement
    pub new_node_idx: NodeIdx,
    /// the node that will replace the target node
    pub replacement: &'a Node<NS, TAG, ATT, VAL, EVENT>,
}

impl<'a, NS, TAG, ATT, VAL, EVENT> ReplaceNode<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// create a new ReplaceNode patch
    pub fn new(
        tag: Option<&'a TAG>,
        node_idx: NodeIdx,
        new_node_idx: NodeIdx,
        replacement: &'a Node<NS, TAG, ATT, VAL, EVENT>,
    ) -> Self {
        ReplaceNode {
            tag,
            node_idx,
            new_node_idx,
            replacement,
        }
    }
}
