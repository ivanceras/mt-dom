//! patch module
pub use add_attributes::AddAttributes;
pub use append_children::AppendChildren;
pub use change_text::ChangeText;
pub use insert_node::InsertNode;
pub use remove_attributes::RemoveAttributes;
pub use remove_node::RemoveNode;
pub use replace_node::ReplaceNode;
use std::fmt::Debug;
pub use tree_path::PatchPath;
pub use tree_path::TreePath;

mod add_attributes;
mod append_children;
mod change_text;
mod insert_node;
mod remove_attributes;
mod remove_node;
mod replace_node;
mod tree_path;

/// NodeIdx alias type
pub type NodeIdx = usize;

/// A Patch encodes an operation that modifies a real DOM element or native UI element
///
/// To update the real DOM that a user sees you'll want to first diff your
/// old virtual dom and new virtual dom.
///
/// This diff operation will generate `Vec<Patch>` with zero or more patches that, when
/// applied to your real DOM, will make your real DOM look like your new virtual dom.
///
/// Each Patch has a usize node index that helps us identify the real DOM node that it applies to.
///
/// Our old virtual dom's nodes are indexed depth first, as shown in this illustration
/// (0 being the root node, 1 being it's first child, 2 being it's first child's first child).
///
/// ```ignore
///             .─.
///            ( 0 )
///             `-'
///            /   \
///           /     \
///          /       \
///         ▼         ▼
///        .─.         .─.
///       ( 1 )       ( 4 )
///        `-'         `-'
///       /  \          | \ '.
///      /    \         |  \  '.
///     ▼      ▼        |   \   '.
///   .─.      .─.      ▼    ▼     ▼
///  ( 2 )    ( 3 )    .─.   .─.   .─.
///   `─'      `─'    ( 5 ) ( 6 ) ( 7 )
///                    `─'   `─'   `─'
/// ```
///
///
#[derive(Clone, Debug, PartialEq)]
pub enum Patch<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// Insert a vector of child nodes to the current node being patch.
    /// The usize is the index of of the children of the node to be
    /// patch to insert to. The new children will be inserted before this usize
    InsertNode(InsertNode<'a, NS, TAG, ATT, VAL, EVENT>),
    /// Append a vector of child nodes to a parent node id.
    AppendChildren(AppendChildren<'a, NS, TAG, ATT, VAL, EVENT>),
    /// remove node
    RemoveNode(RemoveNode<'a, TAG>),
    /// ReplaceNode a node with another node. This typically happens when a node's tag changes.
    /// ex: <div> becomes <span>
    ReplaceNode(ReplaceNode<'a, NS, TAG, ATT, VAL, EVENT>),
    /// Add attributes that the new node has that the old node does not
    /// Note: the attributes is not a reference since attributes of same
    /// name are merged to produce a new unify attribute
    AddAttributes(AddAttributes<'a, NS, TAG, ATT, VAL, EVENT>),
    /// Remove attributes that the old node had that the new node doesn't
    RemoveAttributes(RemoveAttributes<'a, NS, TAG, ATT, VAL, EVENT>),
    /// Change the text of a Text node.
    ChangeText(ChangeText<'a>),
}

impl<'a, NS, TAG, ATT, VAL, EVENT> Patch<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /*
    /// Every Patch is meant to be applied to a specific node within the DOM. Get the
    /// index of the DOM node that this patch should apply to. DOM nodes are indexed
    /// depth first with the root node in the tree having index 0.
    pub fn node_idx(&self) -> NodeIdx {
        match self {
            Patch::InsertNode(ic) => ic.node_idx,
            Patch::AppendChildren(ac) => ac.node_idx,
            Patch::RemoveNode(rn) => rn.node_idx,
            Patch::ReplaceNode(rn) => rn.node_idx,
            Patch::AddAttributes(at) => at.node_idx,
            Patch::RemoveAttributes(rt) => rt.node_idx,
            Patch::ChangeText(ct) => ct.node_idx,
        }
    }
    */

    /// return the tag of this patch
    pub fn tag(&self) -> Option<&TAG> {
        match self {
            Patch::InsertNode(ic) => ic.tag,
            Patch::AppendChildren(ac) => Some(ac.tag),
            Patch::RemoveNode(rn) => rn.tag,
            Patch::ReplaceNode(rn) => rn.tag,
            Patch::AddAttributes(at) => Some(at.tag),
            Patch::RemoveAttributes(rt) => Some(rt.tag),
            Patch::ChangeText(_) => None,
        }
    }

    /// prioritize patches,
    /// patches that doesn't change the NodeIdx in the actual DOM tree will be executed first.
    pub fn priority(&self) -> usize {
        match self {
            Patch::AddAttributes(..) => 1,
            Patch::RemoveAttributes(..) => 2,
            Patch::ChangeText(..) => 3,
            Patch::ReplaceNode(..) => 4,
            Patch::AppendChildren(..) => 5,
            Patch::InsertNode(..) => 6,
            Patch::RemoveNode(..) => 7,
        }
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT> From<ChangeText<'a>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    fn from(ct: ChangeText<'a>) -> Self {
        Patch::ChangeText(ct)
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT>
    From<InsertNode<'a, NS, TAG, ATT, VAL, EVENT>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    fn from(ic: InsertNode<'a, NS, TAG, ATT, VAL, EVENT>) -> Self {
        Patch::InsertNode(ic)
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT>
    From<AppendChildren<'a, NS, TAG, ATT, VAL, EVENT>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    fn from(ac: AppendChildren<'a, NS, TAG, ATT, VAL, EVENT>) -> Self {
        Patch::AppendChildren(ac)
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT> From<RemoveNode<'a, TAG>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    fn from(rc: RemoveNode<'a, TAG>) -> Self {
        Patch::RemoveNode(rc)
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT>
    From<ReplaceNode<'a, NS, TAG, ATT, VAL, EVENT>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    fn from(rn: ReplaceNode<'a, NS, TAG, ATT, VAL, EVENT>) -> Self {
        Patch::ReplaceNode(rn)
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT>
    From<AddAttributes<'a, NS, TAG, ATT, VAL, EVENT>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    fn from(at: AddAttributes<'a, NS, TAG, ATT, VAL, EVENT>) -> Self {
        Patch::AddAttributes(at)
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT>
    From<RemoveAttributes<'a, NS, TAG, ATT, VAL, EVENT>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    fn from(rt: RemoveAttributes<'a, NS, TAG, ATT, VAL, EVENT>) -> Self {
        Patch::RemoveAttributes(rt)
    }
}
