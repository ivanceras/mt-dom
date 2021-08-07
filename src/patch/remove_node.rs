use crate::PatchPath;
use std::fmt::Debug;

/// remove the node at this
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveNode<'a, TAG>
where
    TAG: PartialEq + Clone + Debug,
{
    /// the tag of the node that is to be removed
    pub tag: Option<&'a TAG>,
    /// the node_idx of the node to be removed
    pub patch_path: PatchPath,
}
impl<'a, TAG> RemoveNode<'a, TAG>
where
    TAG: PartialEq + Clone + Debug,
{
    /// create a new RemoveNode patch
    pub fn new(tag: Option<&'a TAG>, patch_path: PatchPath) -> Self {
        RemoveNode { tag, patch_path }
    }
}
