use xml_dom::level2::{Attribute, Document, Node, RefNode};

pub fn traverse_children(node: &mut RefNode, parent: Option<&mut RefNode>) {
    let node_name = node.node_name().to_string();
    if node_name == "Ableton" {
        // -- LIVE 10.0.3 --
        // let major = "5";
        // let minor = "10.0_370";
        // let revision = "5ae7d4938908194888f90ed5411dc3def59687f2";
        // let creator = "Ableton Live 10.0.3";
        // -- LIVE 10.1.14 --
        let major = "5";
        let minor = "10.0_377";
        let revision = "2398148aa88b1dd45414868a29166dc901f88d67";
        let creator = "Ableton Live 10.1.41";

        println!("converting ableton version");
        for (name, mut attribute) in node.attributes() {
            match name.to_string().as_str() {
                "MajorVersion" => attribute.set_value(major).unwrap(),
                "MinorVersion" => attribute.set_value(minor).unwrap(),
                "Revision" => attribute.set_value(revision).unwrap(),
                "Creator" => attribute.set_value(creator).unwrap(),
                _ => (),
            }
        }
    }
    match node.node_name().to_string().as_str() {
        "OverwriteProtectionNumber" => fix_overwrite_protection(node),
        "Scenes" => match parent {
            None => {}
            Some(p) => {
                fix_scenes(node, p);
            }
        },
        _ => {
            for mut e in node.child_nodes() {
                traverse_children(&mut e, Some(node));
            }
        }
    }
}

fn fix_scenes(node: &mut RefNode, parent: &mut RefNode) {
    let mut newnode = node
        .owner_document()
        .unwrap()
        .create_element("SceneNames")
        .unwrap();
    for c in node.child_nodes() {
        newnode.append_child(c).unwrap();
    }

    parent
        .replace_child(newnode, node.to_owned())
        .expect("Couldn't append new child node");
}

fn fix_overwrite_protection(_node: &mut RefNode) {}
