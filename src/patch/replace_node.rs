use crate::Node;
use crate::TreePath;
use std::fmt::Debug;

/// A patch where a node is replaced by the `replacement` node. The target node to be replaced
/// can be traverse using the `patch_path`
#[derive(Clone, Debug, PartialEq)]
pub struct ReplaceNode<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// The tag of the node we are going to replace.
    /// This is only used for additional checking that we are removing the correct node.
    pub tag: Option<&'a TAG>,
    /// the traversal path of the node we are going to replace
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
