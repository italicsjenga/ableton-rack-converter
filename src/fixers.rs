use xml_dom::level2::{Element, Node, RefNode};

pub fn traverse_children(node: &mut RefNode) {
    match node.node_name().to_string().as_str() {
        "OverwriteProtectionNumber" => fix_overwrite_protection(node),
        _ => {
            for mut e in node.child_nodes() {
                traverse_children(&mut e);
            }
        }
    }
}

fn fix_overwrite_protection(node: &mut RefNode) {
    node.set_attribute("Value", "FINDABLE")
        .expect("couldnt set node value");
    println!("{}", node);
}
