use crate::Node;
use crate::PatchPath;
use std::fmt::Debug;

/// InsertNode patch contains the a node to insert into
#[derive(Clone, Debug, PartialEq)]
pub struct InsertNode<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// the tag of the target node to be inserted
    pub tag: Option<&'a TAG>,
    /// the target node_idx of which our node will be inserted before it.
    pub patch_path: PatchPath,
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
        patch_path: PatchPath,
        node: &'a Node<NS, TAG, ATT, VAL>,
    ) -> Self {
        InsertNode {
            tag,
            patch_path,
            node,
        }
    }
}
