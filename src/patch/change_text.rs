use super::NodeIdx;

/// The patch is changing the text content of a text node
#[derive(Debug, PartialEq)]
pub struct ChangeText<'a> {
    /// node index of the text node to be patch
    /// relative to the root node of the application
    pub node_idx: NodeIdx,
    // the old text is not really needed for applying the patch.
    // but it is useful for debugging purposed, that we are changing the intended target text by
    // visual inspection
    pub old: &'a str,
    pub new: &'a str,
}

impl<'a> ChangeText<'a> {
    /// create a new change text patch
    pub fn new(node_idx: NodeIdx, old: &'a str, new: &'a str) -> Self {
        ChangeText { node_idx, old, new }
    }

    /// return the replacement text for the patch
    pub fn get_new(&self) -> &'a str {
        self.new
    }
}
