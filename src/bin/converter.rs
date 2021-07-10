use std::path::PathBuf;

use ableton_rack_converter::{self, fixers};

fn main() {
    let file_load = PathBuf::from(".\\validation\\Utility11.adg");
    let file_save = PathBuf::from(".\\exported_files\\Utility11_save.adg");
    let mut device = ableton_rack_converter::load_adg(file_load);
    fixers::traverse_children(&mut device);
    ableton_rack_converter::save_adg(&device, file_save);
}
