use crate::TreePath;

/// A patch where the comment node is changed
#[derive(Clone, Debug, PartialEq)]
pub struct ChangeComment<'a> {
    /// the target element to be patch can be traverse using this patch path
    pub patch_path: TreePath,
    /// old comment
    pub old: &'a String,
    /// new comment
    pub new: &'a String,
}

impl<'a> ChangeComment<'a> {
    /// create a new change text patch
    pub fn new(old: &'a String, patch_path: TreePath, new: &'a String) -> Self {
        ChangeComment {
            patch_path,
            old,
            new,
        }
    }
}
