use crate::Attribute;
use crate::TreePath;
use std::fmt::Debug;

/// Remove attributes that the old node had that the new node doesn't
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveAttributes<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// the tag of the node to be remove
    /// this is only used for verifying that we are patching the correct node
    pub tag: &'a TAG,
    /// index of the node we are going to patch
    /// relative to the application root node
    pub patch_path: TreePath,
    /// attributes that are to be removed from this target node
    pub attrs: Vec<&'a Attribute<NS, ATT, VAL>>,
}

impl<'a, NS, TAG, ATT, VAL> RemoveAttributes<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// Add attributes that the new node has that the old node does not
    /// Note: the attributes is not a reference since attributes of same
    /// name are merged to produce a new unify attribute
    pub fn new(
        tag: &'a TAG,
        patch_path: TreePath,
        attrs: Vec<&'a Attribute<NS, ATT, VAL>>,
    ) -> Self {
        RemoveAttributes {
            tag,
            patch_path,
            attrs,
        }
    }
}
