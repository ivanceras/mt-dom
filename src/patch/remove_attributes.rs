use super::NodeIdx;
use crate::Attribute;
use std::fmt;

/// Remove attributes that the old node had that the new node doesn't
#[derive(PartialEq)]
pub struct RemoveAttributes<'a, NS, TAG, ATT, VAL, EVENT, MSG> {
    /// the tag of the node to be remove
    /// this is only used for verifying that we are patching the correct node
    pub tag: &'a TAG,
    /// index of the node we are going to patch
    /// relative to the application root node
    pub node_idx: NodeIdx,
    /// the new node_idx of the node we are removing attributes from
    pub new_node_idx: NodeIdx,
    /// attributes that are to be removed from this target node
    pub attrs: Vec<&'a Attribute<NS, ATT, VAL, EVENT, MSG>>,
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG>
    RemoveAttributes<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    /// Add attributes that the new node has that the old node does not
    /// Note: the attributes is not a reference since attributes of same
    /// name are merged to produce a new unify attribute
    pub fn new(
        tag: &'a TAG,
        node_idx: NodeIdx,
        new_node_idx: NodeIdx,
        attrs: Vec<&'a Attribute<NS, ATT, VAL, EVENT, MSG>>,
    ) -> Self {
        RemoveAttributes {
            tag,
            node_idx,
            new_node_idx,
            attrs,
        }
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG> fmt::Debug
    for RemoveAttributes<'a, NS, TAG, ATT, VAL, EVENT, MSG>
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("RemoveAttributes")
            .field("tag", &self.tag)
            .field("node_idx", &self.node_idx)
            .field("new_node_idx", &self.new_node_idx)
            .field("attrs", &self.attrs)
            .finish()
    }
}
