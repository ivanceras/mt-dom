use mt_dom::*;
fn main() {
    let div: Node = element("div", [attr("key", "1")], [leaf("hello")]);
    println!("{:#?}", div);
}
