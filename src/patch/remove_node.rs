use crate::TreePath;
use std::fmt::Debug;

/// A patch where the target element that can be traverse using the patch_path will be remove.
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveNode<'a, TAG>
where
    TAG: PartialEq + Clone + Debug,
{
    /// The tag of the node that is to be removed.
    /// This is only used for additional check where are removing the correct node.
    pub tag: Option<&'a TAG>,
    /// the node_idx of the node to be removed
    pub patch_path: TreePath,
}
impl<'a, TAG> RemoveNode<'a, TAG>
where
    TAG: PartialEq + Clone + Debug,
{
    /// create a new RemoveNode patch
    pub fn new(tag: Option<&'a TAG>, patch_path: TreePath) -> Self {
        RemoveNode { tag, patch_path }
    }
}
