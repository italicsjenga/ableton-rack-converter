#![windows_subsystem = "windows"]

use ableton_rack_converter;
use std::{env, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        return;
    }
    let file_load = PathBuf::from(args[1].as_str());
    if file_load.extension().expect("wrong/no extension") != "xml" {
        return;
    }
    let mut path_str = String::from("");
    path_str.push_str(file_load.file_stem().unwrap().to_str().unwrap());
    path_str.push_str("-compressed.adg");
    let file_save = PathBuf::from(path_str);
    ableton_rack_converter::compress_file(file_load, file_save);
}
