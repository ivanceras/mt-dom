use crate::Attribute;
use crate::TreePath;
use std::fmt::Debug;

/// A patch where a remove attributes of the target element that can be traversed by using the
/// patch_path.
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveAttributes<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// The tag of the node we are removing the attributes from.
    /// This is only used for additional check that we are patching the correct node
    pub tag: &'a TAG,
    /// the path to traverse to get to the target lement of which we remove the attributes
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
    /// Create a RemoveAttribute Patch
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
