//! patch module

//use crate::node::Text;
use crate::{Attribute, Node};
use std::fmt::Debug;

pub use tree_path::TreePath;

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
pub enum Patch<'a, NS, TAG, LEAF, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// insert the nodes before the node at patch_path
    InsertBeforeNode {
        /// the tag of the node at patch_path
        tag: Option<&'a TAG>,
        /// the path to traverse to get to the target element
        /// of which our nodes will be inserted before it.
        patch_path: TreePath,
        /// the nodes to be inserted before patch_path
        nodes: Vec<&'a Node<NS, TAG, LEAF, ATT, VAL>>,
    },

    /// insert the nodes after the node at patch_path
    InsertAfterNode {
        /// the tag of the node at patch_path
        tag: Option<&'a TAG>,
        /// the path to traverse to get to the target element
        /// of which our nodes will be inserted after it.
        patch_path: TreePath,
        /// the nodes to be inserted after the patch_path
        nodes: Vec<&'a Node<NS, TAG, LEAF, ATT, VAL>>,
    },

    /// Append a vector of child nodes to a parent node id at patch_path
    AppendChildren {
        /// the tag of the node we are appending the children into
        tag: &'a TAG,
        /// index of the node we are going to append the children into
        patch_path: TreePath,
        /// children nodes to be appended and their corresponding new_node_idx
        children: Vec<&'a Node<NS, TAG, LEAF, ATT, VAL>>,
    },
    /// remove node
    RemoveNode {
        /// The tag of the node that is to be removed.
        /// This is only used for additional check where are removing the correct node.
        tag: Option<&'a TAG>,
        /// the node_idx of the node to be removed
        patch_path: TreePath,
    },
    /// ReplaceNode a node with another node. This typically happens when a node's tag changes.
    /// ex: <div> becomes <span>
    ReplaceNode {
        /// The tag of the node we are going to replace.
        /// This is only used for additional checking that we are removing the correct node.
        tag: Option<&'a TAG>,
        /// the traversal path of the node we are going to replace
        patch_path: TreePath,
        /// the node that will replace the target node
        replacement: &'a Node<NS, TAG, LEAF, ATT, VAL>,
    },
    /// Add attributes that the new node has that the old node does not
    /// Note: the attributes is not a reference since attributes of same
    /// name are merged to produce a new unify attribute
    AddAttributes {
        /// node tag
        /// use for verifying that the we are patching the correct node which
        /// should match the same tag
        tag: &'a TAG,
        /// the path to traverse to get to the target lement of which we add the attributes.
        patch_path: TreePath,
        /// the attributes to be patched into the target node
        attrs: Vec<&'a Attribute<NS, ATT, VAL>>,
    },
    /// Remove attributes that the old node had that the new node doesn't
    RemoveAttributes {
        /// The tag of the node we are removing the attributes from.
        /// This is only used for additional check that we are patching the correct node
        tag: &'a TAG,
        /// the path to traverse to get to the target lement of which we remove the attributes
        patch_path: TreePath,
        /// attributes that are to be removed from this target node
        attrs: Vec<&'a Attribute<NS, ATT, VAL>>,
    },
    /// Replace the old leaf with a new leaf
    ReplaceLeaf {
        /// the path to be traverse in order to replace this leaf
        patch_path: TreePath,
        /// the old leaf that will be replace,
        /// this is for debugging and assertion purposes that we are
        /// changing the matching old LEAF content in the vdom and the real dom
        old: &'a LEAF,
        /// the new leaf for replacement
        new: &'a LEAF,
    },
}

impl<'a, NS, TAG, LEAF, ATT, VAL> Patch<'a, NS, TAG, LEAF, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// return the path to traverse for this patch to get to the target Node
    pub fn path(&self) -> &TreePath {
        match self {
            Patch::InsertBeforeNode { patch_path, .. } => &patch_path,
            Patch::InsertAfterNode { patch_path, .. } => &patch_path,
            Patch::AppendChildren { patch_path, .. } => &patch_path,
            Patch::RemoveNode { patch_path, .. } => &patch_path,
            Patch::ReplaceNode { patch_path, .. } => &patch_path,
            Patch::AddAttributes { patch_path, .. } => &patch_path,
            Patch::RemoveAttributes { patch_path, .. } => &patch_path,
            Patch::ReplaceLeaf { patch_path, .. } => &patch_path,
        }
    }

    /// return the tag of this patch
    pub fn tag(&self) -> Option<&TAG> {
        match self {
            Patch::InsertBeforeNode { tag, .. } => *tag,
            Patch::InsertAfterNode { tag, .. } => *tag,
            Patch::AppendChildren { tag, .. } => Some(tag),
            Patch::RemoveNode { tag, .. } => *tag,
            Patch::ReplaceNode { tag, .. } => *tag,
            Patch::AddAttributes { tag, .. } => Some(tag),
            Patch::RemoveAttributes { tag, .. } => Some(tag),
            Patch::ReplaceLeaf { .. } => None,
        }
    }

    /// create an InsertNode patch
    pub fn insert_node(
        tag: Option<&'a TAG>,
        patch_path: TreePath,
        node: &'a Node<NS, TAG, LEAF, ATT, VAL>,
    ) -> Patch<'a, NS, TAG, LEAF, ATT, VAL> {
        Patch::InsertBeforeNode {
            tag,
            patch_path,
            nodes: vec![node],
        }
    }

    /// create an InsertBeforeNode patch
    pub fn insert_before_node(
        tag: Option<&'a TAG>,
        patch_path: TreePath,
        nodes: Vec<&'a Node<NS, TAG, LEAF, ATT, VAL>>,
    ) -> Patch<'a, NS, TAG, LEAF, ATT, VAL> {
        Patch::InsertBeforeNode {
            tag,
            patch_path,
            nodes,
        }
    }

    /// create an InsertAfterNode patch
    pub fn insert_after_node(
        tag: Option<&'a TAG>,
        patch_path: TreePath,
        nodes: Vec<&'a Node<NS, TAG, LEAF, ATT, VAL>>,
    ) -> Patch<'a, NS, TAG, LEAF, ATT, VAL> {
        Patch::InsertAfterNode {
            tag,
            patch_path,
            nodes,
        }
    }

    /// create a patch where we add children to the target node
    pub fn append_children(
        tag: &'a TAG,
        patch_path: TreePath,
        children: Vec<&'a Node<NS, TAG, LEAF, ATT, VAL>>,
    ) -> Patch<'a, NS, TAG, LEAF, ATT, VAL> {
        Patch::AppendChildren {
            tag,
            patch_path,
            children,
        }
    }

    /// create a patch where the target element that can be traverse
    /// using the patch path will be remove
    pub fn remove_node(
        tag: Option<&'a TAG>,
        patch_path: TreePath,
    ) -> Patch<'a, NS, TAG, LEAF, ATT, VAL> {
        Patch::RemoveNode { tag, patch_path }
    }

    /// create a patch where a node is replaced by the `replacement` node.
    /// The target node to be replace is traverse using the `patch_path`
    pub fn replace_node(
        tag: Option<&'a TAG>,
        patch_path: TreePath,
        replacement: &'a Node<NS, TAG, LEAF, ATT, VAL>,
    ) -> Patch<'a, NS, TAG, LEAF, ATT, VAL> {
        Patch::ReplaceNode {
            tag,
            patch_path,
            replacement,
        }
    }

    /// create a patch where a new attribute is added to the target element
    pub fn add_attributes(
        tag: &'a TAG,
        patch_path: TreePath,
        attrs: Vec<&'a Attribute<NS, ATT, VAL>>,
    ) -> Patch<'a, NS, TAG, LEAF, ATT, VAL> {
        Patch::AddAttributes {
            tag,
            patch_path,
            attrs,
        }
    }

    /// create patch where it remove attributes of the target element that can be traversed by the
    /// patch_path.
    pub fn remove_attributes(
        tag: &'a TAG,
        patch_path: TreePath,
        attrs: Vec<&'a Attribute<NS, ATT, VAL>>,
    ) -> Patch<'a, NS, TAG, LEAF, ATT, VAL> {
        Patch::RemoveAttributes {
            tag,
            patch_path,
            attrs,
        }
    }

    /// create a patch where the old leaf is replaced with a new one
    pub fn replace_leaf(
        patch_path: TreePath,
        old: &'a LEAF,
        new: &'a LEAF,
    ) -> Patch<'a, NS, TAG, LEAF, ATT, VAL> {
        Patch::ReplaceLeaf {
            patch_path,
            old,
            new,
        }
    }
}
