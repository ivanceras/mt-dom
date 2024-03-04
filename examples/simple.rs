use core::fmt;
use mt_dom::*;


fn main() {
    println!("simple..");
    let elm1: Node =
        element(
            "div",
            vec![
                attr("class", "container".into()),
                attr("id", "elm1".into()),
            ],
            vec![],
        );
    println!("elm1: {:#?}", elm1);

    let elm2: Node =
        element(
            "div",
            vec![
                attr("class", "container".into()),
                attr("id", "elm2".into()),
            ],
            vec![],
        );

    let diff = diff_with_key(&elm1, &elm2);
    println!("patches: {:#?}", diff);
}
