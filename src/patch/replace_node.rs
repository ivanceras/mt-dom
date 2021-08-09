use crate::Node;
use crate::TreePath;
use std::fmt::Debug;

/// Replace a node with another node. This typically happens when a node's tag changes.
/// ex: <div> becomes <span>
#[derive(Clone, Debug, PartialEq)]
pub struct ReplaceNode<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// the tag of the node we are going to replace
    /// can replace text node, and text node doesn't have tags
    pub tag: Option<&'a TAG>,
    /// the index of the node we are going to replace
    pub patch_path: TreePath,
    /// the node that will replace the target node
    pub replacement: &'a Node<NS, TAG, ATT, VAL>,
}

impl<'a, NS, TAG, ATT, VAL> ReplaceNode<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// create a new ReplaceNode patch
    pub fn new(
        tag: Option<&'a TAG>,
        patch_path: TreePath,
        replacement: &'a Node<NS, TAG, ATT, VAL>,
    ) -> Self {
        ReplaceNode {
            tag,
            patch_path,
            replacement,
        }
    }
}
