use ableton_rack_converter::{self, fixers};

fn main() {
    let file_load = String::from(".\\validation\\Utility10.adg");
    let file_save = String::from(".\\exported_files\\Utility10_save.xml");
    let mut device = ableton_rack_converter::load_adg(&file_load);
    fixers::traverse_children(&mut device);
    ableton_rack_converter::save_uncompressed(&device, &file_save);
}
