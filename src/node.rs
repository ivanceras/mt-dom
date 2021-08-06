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
pub enum Node<NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// Element variant of a virtual node
    Element(Element<NS, TAG, ATT, VAL, EVENT>),
    /// Text variant of a virtual node
    Text(Text),
}

/// Text node
#[derive(PartialEq, Default, Debug, Clone)]
pub struct Text {
    /// text node value
    pub text: String,
}

impl Text {
    /// create a new text node
    pub fn new(txt: impl ToString) -> Self {
        Text {
            text: txt.to_string(),
        }
    }

    /// set the text node value
    pub fn set_text(&mut self, txt: impl ToString) {
        self.text = txt.to_string();
    }
}

impl<NS, TAG, ATT, VAL, EVENT> Node<NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// returns true if this a text node
    pub fn is_text(&self) -> bool {
        match self {
            Node::Element(_) => false,
            Node::Text(_) => true,
        }
    }

    /// consume self and return the element if it is an element variant
    /// None if it is a text node
    pub fn take_element(self) -> Option<Element<NS, TAG, ATT, VAL, EVENT>> {
        match self {
            Node::Element(element) => Some(element),
            Node::Text(_) => None,
        }
    }

    /// Get a mutable reference to the element, if this node is an element node
    pub fn as_element_mut(
        &mut self,
    ) -> Option<&mut Element<NS, TAG, ATT, VAL, EVENT>> {
        match *self {
            Node::Element(ref mut element) => Some(element),
            Node::Text(_) => None,
        }
    }

    /// returns a reference to the element if this is an element node
    pub fn as_element_ref(&self) -> Option<&Element<NS, TAG, ATT, VAL, EVENT>> {
        match *self {
            Node::Element(ref element) => Some(element),
            Node::Text(_) => None,
        }
    }

    /// Consume a mutable self and add a children to this node it if is an element
    /// will have no effect if it is a text node.
    /// This is used in building the nodes in a builder pattern
    pub fn add_children(
        mut self,
        children: Vec<Node<NS, TAG, ATT, VAL, EVENT>>,
    ) -> Self {
        if let Some(element) = self.as_element_mut() {
            element.add_children(children);
        }
        self
    }

    /// add children but not consume self
    pub fn add_children_ref_mut(
        &mut self,
        children: Vec<Node<NS, TAG, ATT, VAL, EVENT>>,
    ) {
        if let Some(element) = self.as_element_mut() {
            element.add_children(children);
        }
    }

    /// add attributes to the node and returns itself
    /// this is used in view building
    pub fn add_attributes(
        mut self,
        attributes: Vec<Attribute<NS, ATT, VAL, EVENT>>,
    ) -> Self {
        if let Some(elm) = self.as_element_mut() {
            elm.add_attributes(attributes);
        }
        self
    }

    /// add attributes using a mutable reference to self
    pub fn add_attributes_ref_mut(
        &mut self,
        attributes: Vec<Attribute<NS, ATT, VAL, EVENT>>,
    ) {
        if let Some(elm) = self.as_element_mut() {
            elm.add_attributes(attributes);
        }
    }

    /// get the attributes of this node
    /// returns None if it is a text node
    pub fn get_attributes(&self) -> Option<&[Attribute<NS, ATT, VAL, EVENT>]> {
        match *self {
            Node::Element(ref element) => Some(element.get_attributes()),
            Node::Text(_) => None,
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

    /// returns the text content if it is a text node
    pub fn text(&self) -> Option<&str> {
        match self {
            Node::Text(t) => Some(&t.text),
            Node::Element(_) => None,
        }
    }

    /// return the children of this node if it is an element
    /// returns None if it is a text node
    pub fn get_children(&self) -> Option<&[Node<NS, TAG, ATT, VAL, EVENT>]> {
        if let Some(element) = self.as_element_ref() {
            Some(element.get_children())
        } else {
            None
        }
    }

    /// return the children of this node if it is an element
    /// returns None if it is a text node
    pub fn children_mut(
        &mut self,
    ) -> Option<&mut [Node<NS, TAG, ATT, VAL, EVENT>]> {
        if let Some(element) = self.as_element_mut() {
            Some(element.children_mut())
        } else {
            None
        }
    }

    /// recursive count the number of nodes under this tree
    pub fn node_count(&self) -> usize {
        let mut current = 0;
        self.node_count_recursive(&mut current);
        current
    }

    fn node_count_recursive(&self, current: &mut usize) {
        match self {
            Node::Text(_) => *current += 1,
            Node::Element(element) => {
                *current += 1;
                for child in element.children.iter() {
                    child.node_count_recursive(current);
                }
            }
        }
    }

    /// remove the existing attributes and set with the new value
    pub fn set_attributes_ref_mut(
        &mut self,
        attributes: Vec<Attribute<NS, ATT, VAL, EVENT>>,
    ) {
        if let Some(elm) = self.as_element_mut() {
            elm.set_attributes(attributes);
        }
    }

    /// merge to existing attributes if the attribute name already exist
    pub fn merge_attributes(
        mut self,
        attributes: Vec<Attribute<NS, ATT, VAL, EVENT>>,
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
#[inline]
pub fn element<NS, TAG, ATT, VAL, EVENT>(
    tag: TAG,
    attrs: Vec<Attribute<NS, ATT, VAL, EVENT>>,
    children: Vec<Node<NS, TAG, ATT, VAL, EVENT>>,
) -> Node<NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    element_ns(None, tag, attrs, children, false)
}

/// create a virtual node with namespace, tag, attrs and children
pub fn element_ns<NS, TAG, ATT, VAL, EVENT>(
    namespace: Option<NS>,
    tag: TAG,
    attrs: Vec<Attribute<NS, ATT, VAL, EVENT>>,
    children: Vec<Node<NS, TAG, ATT, VAL, EVENT>>,
    self_closing: bool,
) -> Node<NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    Node::Element(Element::new(namespace, tag, attrs, children, self_closing))
}

/// Create a textnode element
pub fn text<S, NS, TAG, ATT, VAL, EVENT>(s: S) -> Node<NS, TAG, ATT, VAL, EVENT>
where
    S: ToString,
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    Node::Text(Text::new(s))
}
