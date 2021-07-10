use ableton_rack_converter::{self, fixers};

fn main() {
    let file_load = String::from(".\\validation\\Utility11.adg");
    let xml_save = String::from(".\\exported_files\\Utility11_save.xml");
    let file_save = String::from(".\\exported_files\\Utility11_save.adg");
    let mut device = ableton_rack_converter::load_adg(&file_load);
    fixers::traverse_children(&mut device);
    ableton_rack_converter::save_uncompressed(&device, &xml_save);
    ableton_rack_converter::save_adg(&device, &file_save);
}
