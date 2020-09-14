use super::NodeIdx;

/// remove the children with the indices of this node.
/// The usize if the index of the children of this node to remove from
#[derive(Debug, PartialEq)]
pub struct RemoveChildren<'a, TAG> {
    /// the tag of the node which we are to remove the children from
    pub tag: &'a TAG,
    /// index of the node we are patching, relative to the application root node
    pub node_idx: NodeIdx,
    /// which child index to be removed
    pub target_index: Vec<usize>,
}
impl<'a, TAG> RemoveChildren<'a, TAG> {
    /// create a new RemoveChildren patch
    pub fn new(
        tag: &'a TAG,
        node_idx: NodeIdx,
        target_index: Vec<usize>,
    ) -> Self {
        RemoveChildren {
            tag,
            node_idx,
            target_index,
        }
    }
}
