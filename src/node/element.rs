use crate::node::{Attribute, Node};
use std::fmt;
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
pub struct Element<NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// namespace of this element,
    /// svg elements requires namespace to render correcly in the browser
    pub namespace: Option<NS>,
    /// the element tag, such as div, a, button
    pub tag: TAG,
    /// attributes for this element
    pub attrs: Vec<Attribute<NS, ATT, VAL, EVENT>>,
    /// children elements of this element
    pub children: Vec<Node<NS, TAG, ATT, VAL, EVENT>>,
    /// is the element has a self closing tag
    pub self_closing: bool,
}

impl<NS, TAG, ATT, VAL, EVENT> Element<NS, TAG, ATT, VAL, EVENT>
where
    NS: PartialEq + Clone + Debug,
    TAG: PartialEq + Clone + Debug,
    ATT: PartialEq + Clone + Debug,
    VAL: PartialEq + Clone + Debug,
    EVENT: PartialEq + Clone + Debug,
{
    /// create a new instance of an element
    pub fn new(
        namespace: Option<NS>,
        tag: TAG,
        attrs: Vec<Attribute<NS, ATT, VAL, EVENT>>,
        children: Vec<Node<NS, TAG, ATT, VAL, EVENT>>,
        self_closing: bool,
    ) -> Self {
        Element {
            namespace,
            tag,
            attrs,
            children,
            self_closing,
        }
    }

    /// add attributes to this element
    pub fn add_attributes(
        &mut self,
        attrs: Vec<Attribute<NS, ATT, VAL, EVENT>>,
    ) {
        self.attrs.extend(attrs)
    }

    /// add children virtual node to this element
    pub fn add_children(
        &mut self,
        children: Vec<Node<NS, TAG, ATT, VAL, EVENT>>,
    ) {
        self.children.extend(children);
    }

    /// returns a refernce to the children of this node
    pub fn get_children(&self) -> &[Node<NS, TAG, ATT, VAL, EVENT>] {
        &self.children
    }

    /// returns a mutable reference to the children of this node
    pub fn children_mut(&mut self) -> &mut [Node<NS, TAG, ATT, VAL, EVENT>] {
        &mut self.children
    }

    /// consume self and return the children
    pub fn take_children(self) -> Vec<Node<NS, TAG, ATT, VAL, EVENT>> {
        self.children
    }

    /// return a reference to the attribute of this element
    pub fn get_attributes(&self) -> &[Attribute<NS, ATT, VAL, EVENT>] {
        &self.attrs
    }

    /// consume self and return the attributes
    pub fn take_attributes(self) -> Vec<Attribute<NS, ATT, VAL, EVENT>> {
        self.attrs
    }

    /// return the namespace of this element
    pub fn namespace(&self) -> Option<&NS> {
        self.namespace.as_ref()
    }

    /// return the tag of this element
    pub fn tag(&self) -> &TAG {
        &self.tag
    }

    /// consume self and return the tag of this element
    pub fn take_tag(self) -> TAG {
        self.tag
    }

    /// change the tag of this element
    pub fn set_tag(&mut self, tag: TAG) {
        self.tag = tag;
    }

    /// remove the attributes with this key
    pub fn remove_attribute(&mut self, key: &ATT) {
        self.attrs.retain(|att| att.name != *key)
    }

    /// remove the existing values of this attribute
    /// and add the new values
    pub fn set_attributes(
        &mut self,
        attrs: Vec<Attribute<NS, ATT, VAL, EVENT>>,
    ) {
        attrs
            .iter()
            .for_each(|att| self.remove_attribute(&att.name));
        self.add_attributes(attrs);
    }

    /// merge to existing attributes if it exist
    pub fn merge_attributes(
        &mut self,
        new_attrs: Vec<Attribute<NS, ATT, VAL, EVENT>>,
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

    /// return all the attribute values which the name &ATT
    pub fn get_attribute_value(&self, name: &ATT) -> Option<Vec<&VAL>> {
        let result: Vec<&VAL> = self
            .attrs
            .iter()
            .filter(|att| att.name == *name)
            .flat_map(|att| att.get_plain())
            .collect();

        if result.is_empty() {
            None
        } else {
            Some(result)
        }
    }
}
