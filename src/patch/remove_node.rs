use super::NodeIdx;
use std::fmt::Debug;

/// remove the node at this NodeIdx
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveNode<'a, TAG>
where
    TAG: PartialEq + Clone + Debug,
{
    /// the tag of the node that is to be removed
    pub tag: Option<&'a TAG>,
    /// the node_idx of the node to be removed
    pub node_idx: NodeIdx,
}
impl<'a, TAG> RemoveNode<'a, TAG>
where
    TAG: PartialEq + Clone + Debug,
{
    /// create a new RemoveNode patch
    pub fn new(tag: Option<&'a TAG>, node_idx: NodeIdx) -> Self {
        RemoveNode { tag, node_idx }
    }
}
