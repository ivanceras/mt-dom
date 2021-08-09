use crate::node::Text;
use crate::TreePath;

/// A patch where the text content of a text node is changed
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeText<'a> {
    /// the target element to be patch can be traverse using this patch path
    pub patch_path: TreePath,
    /// the old text is not really needed for applying the patch.
    /// but it is useful for debugging purposed, that we are changing the intended target text by
    /// visual inspection
    pub old: &'a Text,
    /// the neew text patch
    pub new: &'a Text,
}

impl<'a> ChangeText<'a> {
    /// create a new change text patch
    pub fn new(old: &'a Text, patch_path: TreePath, new: &'a Text) -> Self {
        ChangeText {
            patch_path,
            old,
            new,
        }
    }
}
