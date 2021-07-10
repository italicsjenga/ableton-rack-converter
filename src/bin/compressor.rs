use ableton_rack_converter;
use std::{fs, path::PathBuf};

fn main() {
    let paths = fs::read_dir(".\\test_conversion\\").unwrap();
    for path in paths {
        let loadpath = path.unwrap().path();
        if loadpath.extension().expect("couldn't get extension") == "xml" {
            let mut path_str = String::from(".\\exported_files\\");
            path_str.push_str(loadpath.file_stem().unwrap().to_str().unwrap());
            path_str.push_str("-compressed.adg");
            let savepath = PathBuf::from(path_str);
            println!("{:?}", savepath);
            ableton_rack_converter::compress_file(loadpath, savepath);
        }
    }
}
