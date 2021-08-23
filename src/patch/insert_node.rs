use crate::Node;
use crate::TreePath;
use std::fmt::Debug;

/// A patch where we insert a new Node before the target element defined by the patch_path
/// traversal
#[derive(Clone, Debug, PartialEq)]
pub struct InsertNode<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// the tag of the target node we insert this node into
    pub tag: Option<&'a TAG>,
    /// the path to traverse to get to the target element of which our node will be inserted before it.
    pub patch_path: TreePath,
    /// the node to be inserted
    pub node: &'a Node<NS, TAG, ATT, VAL>,
}
impl<'a, NS, TAG, ATT, VAL> InsertNode<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// create a new InsertNode patch
    pub fn new(
        tag: Option<&'a TAG>,
        patch_path: TreePath,
        node: &'a Node<NS, TAG, ATT, VAL>,
    ) -> Self {
        InsertNode {
            tag,
            patch_path,
            node,
        }
    }
}
