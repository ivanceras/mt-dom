//! patch module
pub use add_attributes::AddAttributes;
pub use append_children::AppendChildren;
pub use change_text::ChangeText;
pub use insert_children::InsertChildren;
pub use remove_attributes::RemoveAttributes;
pub use remove_node::RemoveNode;
pub use replace_node::ReplaceNode;
use std::fmt;

mod add_attributes;
mod append_children;
mod change_text;
mod insert_children;
mod remove_attributes;
mod remove_node;
mod replace_node;

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
// TODO: create a struct for the contents of each variants
// since they are getting larger
#[derive(PartialEq)]
pub enum Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG> {
    /// Insert a vector of child nodes to the current node being patch.
    /// The usize is the index of of the children of the node to be
    /// patch to insert to. The new children will be inserted before this usize
    InsertChildren(InsertChildren<'a, NS, TAG, ATT, VAL, EVENT, MSG>),
    /// Append a vector of child nodes to a parent node id.
    AppendChildren(AppendChildren<'a, NS, TAG, ATT, VAL, EVENT, MSG>),
    /// remove node
    RemoveNode(RemoveNode<'a, TAG>),
    /// ReplaceNode a node with another node. This typically happens when a node's tag changes.
    /// ex: <div> becomes <span>
    ReplaceNode(ReplaceNode<'a, NS, TAG, ATT, VAL, EVENT, MSG>),
    /// Add attributes that the new node has that the old node does not
    /// Note: the attributes is not a reference since attributes of same
    /// name are merged to produce a new unify attribute
    AddAttributes(AddAttributes<'a, NS, TAG, ATT, VAL, EVENT, MSG>),
    /// Remove attributes that the old node had that the new node doesn't
    RemoveAttributes(RemoveAttributes<'a, NS, TAG, ATT, VAL, EVENT, MSG>),
    /// Change the text of a Text node.
    ChangeText(ChangeText<'a>),
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG>
    Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    /// Every Patch is meant to be applied to a specific node within the DOM. Get the
    /// index of the DOM node that this patch should apply to. DOM nodes are indexed
    /// depth first with the root node in the tree having index 0.
    pub fn node_idx(&self) -> NodeIdx {
        match self {
            Patch::InsertChildren(ic) => ic.node_idx,
            Patch::AppendChildren(ac) => ac.node_idx,
            Patch::RemoveNode(rn) => rn.node_idx,
            Patch::ReplaceNode(rn) => rn.node_idx,
            Patch::AddAttributes(at) => at.node_idx,
            Patch::RemoveAttributes(rt) => rt.node_idx,
            Patch::ChangeText(ct) => ct.node_idx,
        }
    }

    /// return the tag of this patch
    pub fn tag(&self) -> Option<&TAG> {
        match self {
            Patch::InsertChildren(ic) => Some(ic.tag),
            Patch::AppendChildren(ac) => Some(ac.tag),
            Patch::RemoveNode(rn) => rn.tag,
            Patch::ReplaceNode(rn) => Some(rn.tag),
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
            Patch::InsertChildren(..) => 6,
            Patch::RemoveNode(..) => 7,
        }
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG> fmt::Debug
    for Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>
where
    NS: fmt::Debug,
    TAG: fmt::Debug,
    ATT: fmt::Debug,
    VAL: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Patch::InsertChildren(ic) => ic.fmt(f),
            Patch::AppendChildren(ac) => ac.fmt(f),
            Patch::RemoveNode(rn) => rn.fmt(f),
            Patch::ReplaceNode(rn) => rn.fmt(f),
            Patch::AddAttributes(at) => at.fmt(f),
            Patch::RemoveAttributes(rt) => rt.fmt(f),
            Patch::ChangeText(ct) => ct.fmt(f),
        }
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG> From<ChangeText<'a>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    fn from(ct: ChangeText<'a>) -> Self {
        Patch::ChangeText(ct)
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG>
    From<InsertChildren<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    fn from(ic: InsertChildren<'a, NS, TAG, ATT, VAL, EVENT, MSG>) -> Self {
        Patch::InsertChildren(ic)
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG>
    From<AppendChildren<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    fn from(ac: AppendChildren<'a, NS, TAG, ATT, VAL, EVENT, MSG>) -> Self {
        Patch::AppendChildren(ac)
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG> From<RemoveNode<'a, TAG>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    fn from(rc: RemoveNode<'a, TAG>) -> Self {
        Patch::RemoveNode(rc)
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG>
    From<ReplaceNode<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    fn from(rn: ReplaceNode<'a, NS, TAG, ATT, VAL, EVENT, MSG>) -> Self {
        Patch::ReplaceNode(rn)
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG>
    From<AddAttributes<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    fn from(at: AddAttributes<'a, NS, TAG, ATT, VAL, EVENT, MSG>) -> Self {
        Patch::AddAttributes(at)
    }
}

impl<'a, NS, TAG, ATT, VAL, EVENT, MSG>
    From<RemoveAttributes<'a, NS, TAG, ATT, VAL, EVENT, MSG>>
    for Patch<'a, NS, TAG, ATT, VAL, EVENT, MSG>
{
    fn from(rt: RemoveAttributes<'a, NS, TAG, ATT, VAL, EVENT, MSG>) -> Self {
        Patch::RemoveAttributes(rt)
    }
}
