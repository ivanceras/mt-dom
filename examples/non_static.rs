use mt_dom;
use mt_dom::*;

pub type Node<MSG> = mt_dom::Node<
    'static,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    (),
    MSG,
>;

/// Element type with tag and attribute name type set to &'static str
pub type Element<MSG> = mt_dom::Element<
    'static,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    (),
    MSG,
>;

/// Patch as result of diffing the current_vdom and the new vdom.
/// The tag and attribute name types is set to &'static str
pub type Patch<'a, MSG> = mt_dom::Patch<
    'a,
    &'static str,
    &'static str,
    &'static str,
    &'static str,
    (),
    MSG,
>;

/// Attribute type used in sauron where the type of the Attribute name is &'static str
pub type Attribute<MSG> = mt_dom::Attribute<
    'static,
    &'static str,
    &'static str,
    &'static str,
    (),
    MSG,
>;

/// Callback where Event type is supplied
pub type Callback<MSG> = mt_dom::Callback<'static, (), MSG>;

fn main() {
    let elm1: Node<()> = element(
        "div",
        vec![attr("class", "class1"), attr("id", "elm1")],
        vec![],
    );
    println!("eml1: {:#?}", elm1);

    let elm2: Node<()> = element(
        "div",
        vec![attr("class", "class2"), attr("id", "elm2")],
        vec![],
    );
    println!("eml2: {:#?}", elm2);

    let diff = diff_with_key(&elm1, &elm2, &"key");
    println!("patches: {:#?}", diff);
}
