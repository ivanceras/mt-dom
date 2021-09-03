//! patch module
pub use add_attributes::AddAttributes;
pub use append_children::AppendChildren;
pub use change_text::ChangeText;
pub use insert_node::InsertNode;
pub use remove_attributes::RemoveAttributes;
pub use remove_node::RemoveNode;
pub use replace_node::ReplaceNode;
use std::fmt::Debug;
pub use tree_path::TreePath;

mod add_attributes;
mod append_children;
mod change_text;
mod insert_node;
mod remove_attributes;
mod remove_node;
mod replace_node;
mod tree_path;

/// A Patch encodes an operation that modifies a real DOM element or native UI element
///
/// To update the real DOM that a user sees you'll want to first diff your
/// old virtual dom and new virtual dom.
///
/// This diff operation will generate `Vec<Patch>` with zero or more patches that, when
/// applied to your real DOM, will make your real DOM look like your new virtual dom.
///
/// Each of the Patch contains `TreePath` which contains an array of indexes for each node
/// that we need to traverse to get the target element.
///
/// Consider the following html:
///
/// ```html
/// <body>
///     <main>
///         <input type="text"/>
///         <img src="pic.jpg"/>
///     </main>
///     <footer>
///         <a>Link</a>
///         <nav/>
///     </footer>
/// </body>
/// ```
/// The corresponding DOM tree would be
/// ```bob
///              .─.
///             ( 0 )  <body>
///              `-'
///             /   \
///            /     \
///           /       \
///          ▼         ▼
///  <main> .─.         .─. <footer>
///        ( 0 )       ( 1 )
///         `-'         `-'
///        /  \          | \ '.
///       /    \         |  \  '.
///      ▼      ▼        |   \   '.
///    .─.      .─.      ▼    ▼     ▼
///   ( 0 )    ( 1 )    .─.   .─.   .─.
///    `─'      `─'    ( 0 ) ( 1 ) ( 2 )
///  <input> <img>      `─'   `─'   `─'
///                    <a>  <Text>   <nav>
/// ```
/// To traverse to the `<nav>` element we follow the TreePath([0,1,2]).
/// 0 - is the root element which is always zero.
/// 1 - is the `footer` element since it is the 2nd element of the body.
/// 2 - is the `nav` element since it is the 3rd node in the `footer` element.
#[derive(Clone, Debug, PartialEq)]
pub enum Patch<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// Insert a vector of child nodes to the current node being patch.
    /// The usize is the index of of the children of the node to be
    /// patch to insert to. The new children will be inserted before this usize
    InsertNode(InsertNode<'a, NS, TAG, ATT, VAL>),
    /// Append a vector of child nodes to a parent node id.
    AppendChildren(AppendChildren<'a, NS, TAG, ATT, VAL>),
    /// remove node
    RemoveNode(RemoveNode<'a, TAG>),
    /// ReplaceNode a node with another node. This typically happens when a node's tag changes.
    /// ex: <div> becomes <span>
    ReplaceNode(ReplaceNode<'a, NS, TAG, ATT, VAL>),
    /// Add attributes that the new node has that the old node does not
    /// Note: the attributes is not a reference since attributes of same
    /// name are merged to produce a new unify attribute
    AddAttributes(AddAttributes<'a, NS, TAG, ATT, VAL>),
    /// Remove attributes that the old node had that the new node doesn't
    RemoveAttributes(RemoveAttributes<'a, NS, TAG, ATT, VAL>),
    /// Change the text of a Text node.
    ChangeText(ChangeText<'a>),
}

impl<'a, NS, TAG, ATT, VAL> Patch<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// return the path to traverse for this patch to get to the target Node
    pub fn path(&self) -> &[usize] {
        match self {
            Patch::InsertNode(ic) => &ic.patch_path.path,
            Patch::AppendChildren(ac) => &ac.patch_path.path,
            Patch::RemoveNode(rn) => &rn.patch_path.path,
            Patch::ReplaceNode(rn) => &rn.patch_path.path,
            Patch::AddAttributes(at) => &at.patch_path.path,
            Patch::RemoveAttributes(rt) => &rt.patch_path.path,
            Patch::ChangeText(ct) => &ct.patch_path.path,
        }
    }

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
}

impl<'a, NS, TAG, ATT, VAL> From<ChangeText<'a>>
    for Patch<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    fn from(ct: ChangeText<'a>) -> Self {
        Patch::ChangeText(ct)
    }
}

impl<'a, NS, TAG, ATT, VAL> From<InsertNode<'a, NS, TAG, ATT, VAL>>
    for Patch<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    fn from(ic: InsertNode<'a, NS, TAG, ATT, VAL>) -> Self {
        Patch::InsertNode(ic)
    }
}

impl<'a, NS, TAG, ATT, VAL> From<AppendChildren<'a, NS, TAG, ATT, VAL>>
    for Patch<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    fn from(ac: AppendChildren<'a, NS, TAG, ATT, VAL>) -> Self {
        Patch::AppendChildren(ac)
    }
}

impl<'a, NS, TAG, ATT, VAL> From<RemoveNode<'a, TAG>>
    for Patch<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    fn from(rc: RemoveNode<'a, TAG>) -> Self {
        Patch::RemoveNode(rc)
    }
}

impl<'a, NS, TAG, ATT, VAL> From<ReplaceNode<'a, NS, TAG, ATT, VAL>>
    for Patch<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    fn from(rn: ReplaceNode<'a, NS, TAG, ATT, VAL>) -> Self {
        Patch::ReplaceNode(rn)
    }
}

impl<'a, NS, TAG, ATT, VAL> From<AddAttributes<'a, NS, TAG, ATT, VAL>>
    for Patch<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    fn from(at: AddAttributes<'a, NS, TAG, ATT, VAL>) -> Self {
        Patch::AddAttributes(at)
    }
}

impl<'a, NS, TAG, ATT, VAL> From<RemoveAttributes<'a, NS, TAG, ATT, VAL>>
    for Patch<'a, NS, TAG, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    fn from(rt: RemoveAttributes<'a, NS, TAG, ATT, VAL>) -> Self {
        Patch::RemoveAttributes(rt)
    }
}
