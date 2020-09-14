use super::NodeIdx;
use crate::Node;
use std::fmt;

/// Insert a vector of child nodes to the current node being patch.
/// The usize is the index of of the children of the node to be
/// patch to insert to. The new children will be inserted before this usize
#[derive(PartialEq)]
pub struct InsertChildren<'a, NS, TAG, ATT, VAL, EVENT, MSG> {
    /// the tag of the target node we are going to insert children into
    pub tag: &'a TAG,
    /// index of the target node we are patching
    pub node_idx: NodeIdx,
    /// which child index to insert to
    pub target_index: usize,
    /// the nodes that are to be inserted into the target node
    pub children: Vec<&'a Node<NS, TAG, ATT, VAL, EVENT, MSG>>,
}
impl<'a, NS, TAG, ATT, VAL, EVENT, MSG>
    InsertChildren<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    /// create a new InsertChildren patch
    pub fn new(
        tag: &'a TAG,
        node_idx: NodeIdx,
        target_index: usize,
        children: Vec<&'a Node<NS, TAG, ATT, VAL, EVENT, MSG>>,
    ) -> Self {
        InsertChildren {
            tag,
            node_idx,
            target_index,
            children,
        }
    }
}
impl<'a, NS, TAG, ATT, VAL, EVENT, MSG> fmt::Debug
    for InsertChildren<'a, NS, TAG, ATT, VAL, EVENT, MSG>
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InsertChildren")
            .field("tag", &self.tag)
            .field("node_idx", &self.node_idx)
            .field("target_index", &self.target_index)
            .field("children", &self.children)
            .finish()
    }
}
