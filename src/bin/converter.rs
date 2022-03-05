#![windows_subsystem = "windows"]

use ableton_rack_converter::{self, fixers};
use std::{env, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return;
    }
    let file_load = PathBuf::from(args[1].as_str());
    let extension = file_load
        .extension()
        .expect("wrong/no extension")
        .to_str()
        .unwrap();
    if !ableton_rack_converter::validate_filetype(extension) {
        return;
    }
    let path_str = format!(
        "{}{}{}",
        file_load.file_stem().unwrap().to_str().unwrap(),
        "-live10.",
        extension
    );
    let file_save = PathBuf::from(path_str);
    let mut device = ableton_rack_converter::load_ableton_file(file_load);
    fixers::traverse_children(&mut device);
    ableton_rack_converter::save_ableton_file(&device, file_save);
}
