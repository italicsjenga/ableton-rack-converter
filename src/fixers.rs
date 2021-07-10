use xml_dom::level2::{Attribute, Node, RefNode};

pub fn traverse_children(node: &mut RefNode) {
    if node.node_name().to_string() == "Ableton" {
        for (name, mut attribute) in node.attributes() {
            println!("{}: {}", name, attribute);
            match name.to_string().as_str() {
                "MajorVersion" => attribute.set_value("5").unwrap(),
                "MinorVersion" => attribute.set_value("10.0_370").unwrap(),
                "Revision" => attribute
                    .set_value("5ae7d4938908194888f90ed5411dc3def59687f2")
                    .unwrap(),
                "Creator" => attribute.set_value("Ableton Live 10.0.3").unwrap(),
                _ => (),
            }
        }
    }
    match node.node_name().to_string().as_str() {
        "OverwriteProtectionNumber" => fix_overwrite_protection(node),
        _ => {
            for mut e in node.child_nodes() {
                traverse_children(&mut e);
            }
        }
    }
}

fn fix_overwrite_protection(node: &mut RefNode) {}
