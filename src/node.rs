pub use attribute::Attribute;
pub use element::Element;
use std::fmt::Debug;

pub(crate) mod attribute;
mod element;

/// represents a node in a virtual dom
/// A node could be an element which can contain one or more children of nodes.
/// A node could also be just a text node which contains a string
///
/// Much of the types are Generics
///
/// NS - is the type for the namespace, this will be &'static str when used in html based virtual dom implementation
/// TAG - is the type for the element tag, this will be &'static str when used in html based virtual
/// dom impmenentation
/// ATT - is the type for the attribute name, this will be &'static str when used in html based
/// virtual dom implementation
/// VAL - is the type for the value of the attribute, this will be String, f64, or just another
/// generics that suits the implementing library which used mt-dom for just dom-diffing purposes
#[derive(Clone, Debug, PartialEq)]
pub enum Node<NS, TAG, LEAF, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// Element variant of a virtual node
    Element(Element<NS, TAG, LEAF, ATT, VAL>),
    /// A Leaf node
    Leaf(LEAF),
}

impl<NS, TAG, LEAF, ATT, VAL> Node<NS, TAG, LEAF, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    /// consume self and return the element if it is an element variant
    /// None if it is a text node
    pub fn take_element(self) -> Option<Element<NS, TAG, LEAF, ATT, VAL>> {
        match self {
            Node::Element(element) => Some(element),
            _ => None,
        }
    }

    /// returns a reference to the LEAF if the node is a Leaf variant
    pub fn as_leaf_ref(&self) -> Option<&LEAF> {
        match self {
            Node::Leaf(leaf) => Some(leaf),
            _ => None,
        }
    }

    /// Get a mutable reference to the element, if this node is an element node
    pub fn as_element_mut(
        &mut self,
    ) -> Option<&mut Element<NS, TAG, LEAF, ATT, VAL>> {
        match *self {
            Node::Element(ref mut element) => Some(element),
            _ => None,
        }
    }

    /// returns a reference to the element if this is an element node
    pub fn as_element_ref(&self) -> Option<&Element<NS, TAG, LEAF, ATT, VAL>> {
        match *self {
            Node::Element(ref element) => Some(element),
            _ => None,
        }
    }

    /// Consume a mutable self and add a children to this node it if is an element
    /// will have no effect if it is a text node.
    /// This is used in building the nodes in a builder pattern
    pub fn add_children(
        mut self,
        children: impl IntoIterator<Item = Node<NS, TAG, LEAF, ATT, VAL>>,
    ) -> Self {
        if let Some(element) = self.as_element_mut() {
            element.add_children(children);
        } else {
            panic!("Can not add children to a text node");
        }
        self
    }

    /// add children but not consume self
    pub fn add_children_ref_mut(
        &mut self,
        children: impl IntoIterator<Item = Node<NS, TAG, LEAF, ATT, VAL>>,
    ) {
        if let Some(element) = self.as_element_mut() {
            element.add_children(children);
        } else {
            panic!("Can not add children to a text node");
        }
    }

    /// add attributes to the node and returns itself
    /// this is used in view building
    pub fn add_attributes(
        mut self,
        attributes: impl IntoIterator<Item = Attribute<NS, ATT, VAL>>,
    ) -> Self {
        if let Some(elm) = self.as_element_mut() {
            elm.add_attributes(attributes);
        } else {
            panic!("Can not add attributes to a text node");
        }
        self
    }

    /// add attributes using a mutable reference to self
    pub fn add_attributes_ref_mut(
        &mut self,
        attributes: impl IntoIterator<Item = Attribute<NS, ATT, VAL>>,
    ) {
        if let Some(elm) = self.as_element_mut() {
            elm.add_attributes(attributes);
        } else {
            panic!("Can not add attributes to a text node");
        }
    }

    /// get the attributes of this node
    /// returns None if it is a text node
    pub fn get_attributes(&self) -> Option<&[Attribute<NS, ATT, VAL>]> {
        match *self {
            Node::Element(ref element) => Some(element.get_attributes()),
            _ => None,
        }
    }

    /// returns the tag of this node if it is an element
    /// otherwise None if it is a text node
    pub fn tag(&self) -> Option<&TAG> {
        if let Some(e) = self.as_element_ref() {
            Some(&e.tag)
        } else {
            None
        }
    }

    /// return the children of this node if it is an element
    /// returns None if it is a text node
    pub fn get_children(&self) -> Option<&[Node<NS, TAG, LEAF, ATT, VAL>]> {
        if let Some(element) = self.as_element_ref() {
            Some(element.get_children())
        } else {
            None
        }
    }

    /// Return the count of the children of this node
    pub fn get_children_count(&self) -> usize {
        if let Some(children) = self.get_children() {
            children.len()
        } else {
            0
        }
    }

    /// return the children of this node if it is an element
    /// returns None if it is a text node
    pub fn children_mut(
        &mut self,
    ) -> Option<&mut [Node<NS, TAG, LEAF, ATT, VAL>]> {
        if let Some(element) = self.as_element_mut() {
            Some(element.children_mut())
        } else {
            None
        }
    }
    /// Removes an child node  from this element and returns it.
    ///
    /// The removed child is replaced by the last child of the element's children.
    ///
    /// # Panics
    /// Panics if this is a text node
    ///
    pub fn swap_remove_child(
        &mut self,
        index: usize,
    ) -> Node<NS, TAG, LEAF, ATT, VAL> {
        match self {
            Node::Element(element) => element.swap_remove_child(index),
            _ => panic!("text has no child"),
        }
    }

    /// Swaps the 2 child node in this element
    ///
    /// # Arguments
    /// * a - The index of the first child node
    /// * b - The index of the second child node
    ///
    /// # Panics
    /// Panics if both `a` and `b` are out of bounds
    /// Panics if this is a text node
    pub fn swap_children(&mut self, a: usize, b: usize) {
        match self {
            Node::Element(element) => element.swap_children(a, b),
            _ => panic!("text has no child"),
        }
    }

    /// Returns the total number of nodes on this node tree, that is counting the direct and
    /// indirect child nodes of this node.
    pub fn node_count(&self) -> usize {
        1 + self.descendant_node_count()
    }

    /// only count the descendant node
    pub fn descendant_node_count(&self) -> usize {
        let mut cnt = 0;
        match self {
            Node::Element(element) => {
                for child in element.children.iter() {
                    cnt += child.node_count();
                }
            }
            _ => (),
        }
        cnt
    }

    /// remove the existing attributes and set with the new value
    pub fn set_attributes_ref_mut(
        &mut self,
        attributes: impl IntoIterator<Item = Attribute<NS, ATT, VAL>>,
    ) {
        if let Some(elm) = self.as_element_mut() {
            elm.set_attributes(attributes);
        }
    }

    /// merge to existing attributes if the attribute name already exist
    pub fn merge_attributes(
        mut self,
        attributes: impl IntoIterator<Item = Attribute<NS, ATT, VAL>>,
    ) -> Self {
        if let Some(elm) = self.as_element_mut() {
            elm.merge_attributes(attributes);
        }
        self
    }

    /// returh the attribute values of this node which match the attribute name `name`
    pub fn get_attribute_value(&self, name: &ATT) -> Option<Vec<&VAL>> {
        if let Some(elm) = self.as_element_ref() {
            elm.get_attribute_value(name)
        } else {
            None
        }
    }
}

/// create a virtual node with tag, attrs and children
/// # Example
/// ```rust
/// use mt_dom::{Node,element,attr};
///
/// let div:Node<&'static str, &'static str, &'static str, &'static str, &'static str> =
///     element(
///          "div",
///          vec![attr("class", "container")],
///          vec![],
///      );
/// ```
#[inline]
pub fn element<NS, TAG, LEAF, ATT, VAL>(
    tag: TAG,
    attrs: impl IntoIterator<Item = Attribute<NS, ATT, VAL>>,
    children: impl IntoIterator<Item = Node<NS, TAG, LEAF, ATT, VAL>>,
) -> Node<NS, TAG, LEAF, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    element_ns(None, tag, attrs, children, false)
}

/// create a virtual node with namespace, tag, attrs and children
/// # Example
/// ```rust
/// use mt_dom::{Node,element_ns,attr};
///
/// let svg:Node<&'static str, &'static str, (), &'static str, &'static str> =
///     element_ns(
///         Some("http://www.w3.org/2000/svg"),
///          "svg",
///          vec![attr("width","400"), attr("height","400")],
///          vec![],
///          false
///      );
/// ```
pub fn element_ns<NS, TAG, LEAF, ATT, VAL>(
    namespace: Option<NS>,
    tag: TAG,
    attrs: impl IntoIterator<Item = Attribute<NS, ATT, VAL>>,
    children: impl IntoIterator<Item = Node<NS, TAG, LEAF, ATT, VAL>>,
    self_closing: bool,
) -> Node<NS, TAG, LEAF, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    Node::Element(Element::new(namespace, tag, attrs, children, self_closing))
}

/// create a leaf node
pub fn leaf<NS, TAG, LEAF, ATT, VAL>(
    leaf: LEAF,
) -> Node<NS, TAG, LEAF, ATT, VAL>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Debug,
    LEAF: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
{
    Node::Leaf(leaf)
}
