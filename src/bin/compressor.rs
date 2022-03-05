#![windows_subsystem = "windows"]

use ableton_rack_converter;
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
    if extension != "xml" {
        return;
    }
    let path_str = file_load.file_stem().unwrap().to_str().unwrap();
    let file_save = PathBuf::from(path_str);
    ableton_rack_converter::compress_file(file_load, file_save);
}
