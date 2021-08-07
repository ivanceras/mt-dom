use super::NodeIdx;
use crate::Node;
use crate::PatchPath;
use std::fmt::Debug;

/// Append a vector of child nodes to a parent node id.
#[derive(Clone, Debug, PartialEq)]
pub struct AppendChildren<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// the tag of the node we are appending the children into
    pub tag: &'a TAG,
    /// index of the node we are going to append the children into
    pub patch_path: PatchPath,
    /// children nodes to be appended and their corresponding new_node_idx
    pub children: Vec<(NodeIdx, &'a Node<NS, TAG, ATT, VAL, EVENT>)>,
}

impl<'a, NS, TAG, ATT, VAL, EVENT> AppendChildren<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// create a new AppendChildren patch
    pub fn new(
        tag: &'a TAG,
        patch_path: PatchPath,
        children: Vec<(NodeIdx, &'a Node<NS, TAG, ATT, VAL, EVENT>)>,
    ) -> Self {
        AppendChildren {
            tag,
            patch_path,
            children,
        }
    }
}
