use super::NodeIdx;
use crate::Node;
use std::fmt;

/// InsertNode patch contains the a node to insert into
#[derive(PartialEq)]
pub struct InsertNode<'a, NS, TAG, ATT, VAL, EVENT, MSG> {
    /// the tag of the target node to be inserted
    pub tag: Option<&'a TAG>,
    /// the target node_idx of which our node will be inserted before it.
    pub node_idx: NodeIdx,
    /// the node to be inserted
    pub node: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
}
impl<'a, NS, TAG, ATT, VAL, EVENT, MSG>
    InsertNode<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    /// create a new InsertNode patch
    pub fn new(
        tag: Option<&'a TAG>,
        node_idx: NodeIdx,
        node: &'a Node<NS, TAG, ATT, VAL, EVENT, MSG>,
    ) -> Self {
        InsertNode {
            tag,
            node_idx,
            node,
        }
    }
}
impl<'a, NS, TAG, ATT, VAL, EVENT, MSG> fmt::Debug
    for InsertNode<'a, NS, TAG, ATT, VAL, EVENT, MSG>
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("InsertNode")
            .field("tag", &self.tag)
            .field("node_idx", &self.node_idx)
            .field("node", &self.node)
            .finish()
    }
}
