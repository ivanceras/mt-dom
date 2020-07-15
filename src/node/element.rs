use crate::node::Attribute;
use crate::node::Node;

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
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Element<NS, TAG, ATT, VAL> {
    /// namespace of this element,
    /// svg elements requires namespace to render correcly in the browser
    pub(crate) namespace: Option<NS>,
    /// the element tag, such as div, a, button
    pub(crate) tag: TAG,
    /// attributes for this element
    pub(crate) attrs: Vec<Attribute<NS, ATT, VAL>>,
    /// children elements of this element
    pub(crate) children: Vec<Node<NS, TAG, ATT, VAL>>,
}

impl<NS, TAG, ATT, VAL> Element<NS, TAG, ATT, VAL> {
    /// create a new instance of an element
    pub fn new(
        namespace: Option<NS>,
        tag: TAG,
        attrs: Vec<Attribute<NS, ATT, VAL>>,
        children: Vec<Node<NS, TAG, ATT, VAL>>,
    ) -> Self {
        Element {
            namespace,
            tag,
            attrs,
            children,
        }
    }
    /// add attributes to this element
    pub fn add_attributes(&mut self, attrs: Vec<Attribute<NS, ATT, VAL>>) {
        self.attrs.extend(attrs)
    }

    /// add children virtual node to this element
    pub fn add_children(&mut self, children: Vec<Node<NS, TAG, ATT, VAL>>) {
        self.children.extend(children);
    }

    /// returns a refernce to the children of this node
    pub fn get_children(&self) -> &[Node<NS, TAG, ATT, VAL>] {
        &self.children
    }

    /// returns a mutable refernce to the children of this node
    pub fn children_mut(&mut self) -> &mut [Node<NS, TAG, ATT, VAL>] {
        &mut self.children
    }

    /// consume self and return the children
    pub fn take_children(self) -> Vec<Node<NS, TAG, ATT, VAL>> {
        self.children
    }

    /// return a reference to the attribute of this element
    pub fn get_attributes(&self) -> &[Attribute<NS, ATT, VAL>] {
        &self.attrs
    }

    /// return the namespace of this element
    pub fn namespace(&self) -> Option<&NS> {
        self.namespace.as_ref()
    }

    /// return the tag of this element
    pub fn tag(&self) -> &TAG {
        &self.tag
    }

    /// change the tag of this element
    pub fn set_tag(&mut self, tag: TAG) {
        self.tag = tag;
    }

    /// consume and transform this element such that the type of
    /// Attribute will be change from VAL to VAL2
    pub fn map<F, VAL2>(self, f: &F) -> Element<NS, TAG, ATT, VAL2>
    where
        F: Fn(VAL) -> VAL2,
    {
        Element {
            namespace: self.namespace,
            tag: self.tag,
            attrs: self.attrs.into_iter().map(|attr| attr.map(f)).collect(),
            children: self
                .children
                .into_iter()
                .map(|child| child.map(f))
                .collect(),
        }
    }
}

impl<NS, TAG, ATT, VAL> Element<NS, TAG, ATT, VAL>
where
    ATT: PartialEq,
{
    /// remove the attributes with this key
    pub fn remove_attribute(&mut self, key: &ATT) {
        self.attrs.retain(|att| att.name != *key)
    }

    /// remove the existing values of this attribute
    /// and add the new values
    pub fn set_attributes(&mut self, attrs: Vec<Attribute<NS, ATT, VAL>>) {
        attrs
            .iter()
            .for_each(|att| self.remove_attribute(&att.name));
        self.add_attributes(attrs);
    }

    /// return the values of the attributes that matches the given attribute name
    pub fn get_attribute_values(&self, key: &ATT) -> Vec<&VAL> {
        self.attrs
            .iter()
            .filter(|att| att.name == *key)
            .map(|att| &att.value)
            .collect()
    }

    /// return the list of unique attribute names
    /// return according to the order they are seen
    fn get_attribute_names(&self) -> Vec<&ATT> {
        let mut names: Vec<&ATT> = vec![];
        for att in self.attrs.iter() {
            if !names.contains(&&att.name) {
                names.push(&att.name);
            }
        }
        names
    }

    /// return the aggregated values of attributes that has the same
    /// name in this element
    pub fn get_attribute_key_values(&self) -> Vec<(&ATT, Vec<&VAL>)> {
        let mut key_values = vec![];
        let names = self.get_attribute_names();
        for name in names {
            key_values.push((name, self.get_attribute_values(name)));
        }
        key_values
    }
}
