use super::NodeIdx;
use crate::Node;
use std::fmt;

/// Append a vector of child nodes to a parent node id.
#[derive(PartialEq)]
pub struct AppendChildren<'a, NS, TAG, ATT, VAL, EVENT, MSG> {
    pub tag: &'a TAG,
    pub node_idx: NodeIdx,
    pub children: Vec<&'a Node<NS, TAG, ATT, VAL, EVENT, MSG>>,
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG>
    AppendChildren<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    pub fn new(
        tag: &'a TAG,
        node_idx: NodeIdx,
        children: Vec<&'a Node<NS, TAG, ATT, VAL, EVENT, MSG>>,
    ) -> Self {
        AppendChildren {
            tag,
            node_idx,
            children,
        }
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG> fmt::Debug
    for AppendChildren<'a, NS, TAG, ATT, VAL, EVENT, MSG>
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AppendChildren")
            .field("tag", &self.tag)
            .field("node_idx", &self.node_idx)
            .field("children", &self.children)
            .finish()
    }
}
