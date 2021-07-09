use std::{fs::File, io::Read};

use flate2::read::GzDecoder;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn load_adg(filename: &str) {
    let contents = File::open(filename).unwrap();
    decompress(contents);
}

fn decompress(loaded_file: File) {
    let mut decoder = GzDecoder::new(loaded_file);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).unwrap();
    println!("{}", decompressed);
}
