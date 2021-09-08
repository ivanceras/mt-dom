use mt_dom::*;
pub type MyNode = Node<&'static str, &'static str, &'static str, &'static str>;
fn main() {
    let div: MyNode = element("div", [attr("key", "1")], [text("hello")]);
    println!("{:#?}", div);
}
