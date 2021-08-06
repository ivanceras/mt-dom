//! patch is adding attributes
use super::NodeIdx;
use crate::Attribute;
use std::fmt;
use std::fmt::Debug;

/// Add attributes
#[derive(Clone, Debug, PartialEq)]
pub struct AddAttributes<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// node tag
    /// use for verifying that the we are patching the correct node which
    /// should match the same tag
    pub tag: &'a TAG,
    /// index of the node we are going to patch
    pub node_idx: NodeIdx,
    /// new node_idx of the node we are adding an attribute to
    pub new_node_idx: NodeIdx,
    /// the attributes to be patched into the target node
    pub attrs: Vec<&'a Attribute<NS, ATT, VAL, EVENT>>,
}

impl<'a, NS, TAG, ATT, VAL, EVENT> AddAttributes<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// Add attributes that the new node has that the old node does not
    /// Note: the attributes is not a reference since attributes of same
    /// name are merged to produce a new unify attribute
    pub fn new(
        tag: &'a TAG,
        node_idx: NodeIdx,
        new_node_idx: NodeIdx,
        attrs: Vec<&'a Attribute<NS, ATT, VAL, EVENT>>,
    ) -> Self {
        AddAttributes {
            tag,
            node_idx,
            new_node_idx,
            attrs,
        }
    }
}
