use super::NodeIdx;

/// remove the children with the indices of this node.
/// The usize if the index of the children of this node to remove from
#[derive(Debug, PartialEq)]
pub struct RemoveNode<'a, TAG> {
    /// the tag of the node which we are to remove the children from
    pub tag: Option<&'a TAG>,
    /// index of the node we are patching, relative to the application root node
    pub node_idx: NodeIdx,
}
impl<'a, TAG> RemoveNode<'a, TAG> {
    /// create a new RemoveChildren patch
    pub fn new(tag: Option<&'a TAG>, node_idx: NodeIdx) -> Self {
        RemoveNode { tag, node_idx }
    }
}
