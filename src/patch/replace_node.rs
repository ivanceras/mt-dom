use super::NodeIdx;
use crate::Node;
use std::fmt;

/// Replace a node with another node. This typically happens when a node's tag changes.
/// ex: <div> becomes <span>
#[derive(PartialEq)]
pub struct ReplaceNode<'a, NS, TAG, ATT, VAL, EVENT, MSG> {
    /// the tag of the node we are going to replace
    /// can replace text node, and text node doesn't have tags
    pub tag: Option<&'a TAG>,
    /// the index of the node we are going to replace
    pub node_idx: NodeIdx,
    /// the node that will replace the target node
    pub replacement: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG>
    ReplaceNode<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    /// create a new ReplaceNode patch
    pub fn new(
        tag: Option<&'a TAG>,
        node_idx: NodeIdx,
        replacement: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    ) -> Self {
        ReplaceNode {
            tag,
            node_idx,
            replacement,
        }
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG> fmt::Debug
    for ReplaceNode<'a, NS, TAG, ATT, VAL, EVENT, MSG>
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("ReplaceNode")
            .field("tag", &self.tag)
            .field("node_idx", &self.node_idx)
            .field("replacement", &self.replacement)
            .finish()
    }
}
