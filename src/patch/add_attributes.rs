//! patch is adding attributes
use crate::Attribute;
use crate::TreePath;
use std::fmt::Debug;

/// A patch where a new attributes is added to the target element
#[derive(Clone, Debug, PartialEq)]
pub struct AddAttributes<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// node tag
    /// use for verifying that the we are patching the correct node which
    /// should match the same tag
    pub tag: &'a TAG,
    /// the path to traverse to get to the target lement of which we add the attributes.
    pub patch_path: TreePath,
    /// the attributes to be patched into the target node
    pub attrs: Vec<&'a Attribute<NS, ATT, VAL>>,
}

impl<'a, NS, TAG, ATT, VAL> AddAttributes<'a, NS, TAG, ATT, VAL>
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
        AddAttributes {
            tag,
            patch_path,
            attrs,
        }
    }
}
