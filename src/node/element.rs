use super::attribute::{Att, Ns, Tag, Val};
use super::{Attribute, Node};
use std::fmt::Debug;

/// Represents an element of the virtual node
/// An element has a generic tag, this tag could be a static str tag, such as usage in html dom.
///     Example of which are `div`, `a`, `input`, `img`, etc.
///
/// Tag is a generic type, which can represent a different DOM tree other than the html dom
/// such as widgets in native platform such as gtk, example of which are `Hpane`, `Vbox`, `Image`,
///
/// An element can have an optional namespace, such in the case for html dom where namespace like
/// HTML and SVG, which needs to specified in order to create the DOM element to work on the
/// browser.
///
/// The namespace is also needed in attributes where namespace are necessary such as `xlink:href`
/// where the namespace `xlink` is needed in order for the linked element in an svg image to work.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Element {
    /// namespace of this element,
    /// svg elements requires namespace to render correcly in the browser
    pub namespace: Option<Ns>,
    /// the element tag, such as div, a, button
    pub tag: Tag,
    /// attributes for this element
    pub attrs: Vec<Attribute>,
    /// children elements of this element
    pub children: Vec<Node>,
    /// is the element has a self closing tag
    pub self_closing: bool,
}

impl Element {
    /// create a new instance of an element
    pub fn new(
        namespace: Option<Ns>,
        tag: Tag,
        attrs: impl IntoIterator<Item = Attribute>,
        children: impl IntoIterator<Item = Node>,
        self_closing: bool,
    ) -> Self {
        //unroll the nodelist
        let children = children
            .into_iter()
            .flat_map(|child| match child {
                Node::NodeList(node_list) => node_list,
                _ => vec![child],
            })
            .collect();
        Self {
            namespace,
            tag,
            attrs: attrs.into_iter().collect(),
            children,
            self_closing,
        }
    }

    /// add attributes to this element
    pub fn add_attributes(
        &mut self,
        attrs: impl IntoIterator<Item = Attribute>,
    ) {
        self.attrs.extend(attrs)
    }

    /// add children virtual node to this element
    pub fn add_children(&mut self, children: impl IntoIterator<Item = Node>) {
        self.children.extend(children.into_iter());
    }

    /// returns a refernce to the children of this node
    pub fn children(&self) -> &[Node] {
        &self.children
    }

    /// returns a mutable reference to the children of this node
    pub fn children_mut(&mut self) -> &mut [Node] {
        &mut self.children
    }

    /// Removes an child node  from this element and returns it.
    ///
    /// The removed child is replaced by the last child of the element's children.
    ///
    /// # Panics
    /// Panics if index is out of bounds in children
    ///
    pub fn swap_remove_child(&mut self, index: usize) -> Node {
        self.children.swap_remove(index)
    }

    /// Swaps the 2 child node in this element
    ///
    /// # Arguments
    /// * a - The index of the first child node
    /// * b - The index of the second child node
    ///
    /// # Panics
    /// Panics if both `a` and `b` are out of bounds
    ///
    pub fn swap_children(&mut self, a: usize, b: usize) {
        self.children.swap(a, b)
    }

    /// consume self and return the children
    pub fn take_children(self) -> Vec<Node> {
        self.children
    }

    /// return a reference to the attribute of this element
    pub fn attributes(&self) -> &[Attribute] {
        &self.attrs
    }

    /// consume self and return the attributes
    pub fn take_attributes(self) -> Vec<Attribute> {
        self.attrs
    }

    /// return the namespace of this element
    pub fn namespace(&self) -> Option<&Ns> {
        self.namespace.as_ref()
    }

    /// return the tag of this element
    pub fn tag(&self) -> &Tag {
        &self.tag
    }

    /// consume self and return the tag of this element
    pub fn take_tag(self) -> Tag {
        self.tag
    }

    /// change the tag of this element
    pub fn set_tag(&mut self, tag: Tag) {
        self.tag = tag;
    }

    /// remove the attributes with this key
    pub fn remove_attribute(&mut self, name: &Att) {
        self.attrs.retain(|att| att.name != *name)
    }

    /// remove the existing values of this attribute
    /// and add the new values
    pub fn set_attributes(
        &mut self,
        attrs: impl IntoIterator<Item = Attribute>,
    ) {
        for attr in attrs {
            self.remove_attribute(&attr.name);
            self.attrs.push(attr);
        }
    }

    /// merge to existing attributes if it exist
    pub fn merge_attributes(
        &mut self,
        new_attrs: impl IntoIterator<Item = Attribute>,
    ) {
        for new_att in new_attrs {
            if let Some(existing_attr) =
                self.attrs.iter_mut().find(|att| att.name == new_att.name)
            {
                existing_attr.value.extend(new_att.value);
            } else {
                self.attrs.push(new_att);
            }
        }
    }

    /// return all the attribute values which the name &Att
    pub fn attribute_value(&self, name: &Att) -> Option<Vec<&Val>> {
        let result: Vec<&Val> = self
            .attrs
            .iter()
            .filter(|att| att.name == *name)
            .flat_map(|att| att.value())
            .collect();

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}
