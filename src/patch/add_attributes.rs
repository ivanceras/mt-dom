//! patch is adding attributes
use super::NodeIdx;
use crate::Attribute;
use std::fmt;

/// Add attributes
#[derive(PartialEq)]
pub struct AddAttributes<'a, NS, TAG, ATT, VAL, EVENT, MSG> {
    /// node tag
    /// use for verifying that the we are patching the correct node which
    /// should match the same tag
    pub tag: &'a TAG,
    /// index of the node we are going to patch
    pub node_idx: NodeIdx,
    /// new node_idx of the node we are adding an attribute to
    pub new_node_idx: NodeIdx,
    /// the attributes to be patched into the target node
    pub attrs: Vec<&'a Attribute<NS, ATT, VAL, EVENT, MSG>>,
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG>
    AddAttributes<'a, NS, TAG, ATT, VAL, EVENT, MSG>
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
        AddAttributes {
            tag,
            node_idx,
            new_node_idx,
            attrs,
        }
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG> fmt::Debug
    for AddAttributes<'a, NS, TAG, ATT, VAL, EVENT, MSG>
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("AddAttributes")
            .field("tag", &self.tag)
            .field("node_idx", &self.node_idx)
            .field("new_node_idx", &self.new_node_idx)
            .field("attrs", &self.attrs)
            .finish()
    }
}
