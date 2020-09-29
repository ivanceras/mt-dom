use super::NodeIdx;
use crate::node::Text;

/// The patch is changing the text content of a text node
#[derive(Debug, PartialEq)]
pub struct ChangeText<'a> {
    /// node index of the text node to be patch
    /// relative to the root node of the application
    pub node_idx: NodeIdx,
    /// the new node_idx of this text
    pub new_node_idx: NodeIdx,
    /// the old text is not really needed for applying the patch.
    /// but it is useful for debugging purposed, that we are changing the intended target text by
    /// visual inspection
    pub old: &'a Text,
    /// the neew text patch
    pub new: &'a Text,
}

impl<'a> ChangeText<'a> {
    /// create a new change text patch
    pub fn new(
        node_idx: NodeIdx,
        new_node_idx: NodeIdx,
        old: &'a Text,
        new: &'a Text,
    ) -> Self {
        ChangeText {
            node_idx,
            new_node_idx,
            old,
            new,
        }
    }
}
