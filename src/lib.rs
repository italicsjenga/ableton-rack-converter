use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    str,
};
use xml_dom::{level2::RefNode, parser};

pub mod fixers;

pub fn load_adg(filename: PathBuf) -> RefNode {
    let contents = File::open(filename).expect("failed to load file");
    let xml = decompress(contents);
    decode(&xml)
}

fn decompress(loaded_file: File) -> String {
    let mut decoder = GzDecoder::new(loaded_file);
    let mut decompressed = String::new();
    decoder
        .read_to_string(&mut decompressed)
        .expect("could not decompress file");
    decompressed
}

fn decode(xml: &str) -> RefNode {
    parser::read_xml(xml).expect("failed to parse xml")
}

pub fn save_adg(dom: &RefNode, filename: PathBuf) {
    let xml = encode(dom);
    let compressed = compress(&xml);
    fs::write(filename, compressed).expect("could not write file");
}

pub fn save_uncompressed(dom: &RefNode, filename: PathBuf) {
    let xml = encode(dom);
    fs::write(filename, xml.as_bytes()).expect("could not write file");
}

pub fn compress_file(loadpath: PathBuf, savepath: PathBuf) {
    let mut contents = File::open(loadpath).expect("failed to load file");
    let mut loaded = String::new();
    contents
        .read_to_string(&mut loaded)
        .expect("failed to read file to string");
    let compressed = compress(&loaded);
    fs::write(savepath, compressed).expect("failed to write compressed file");
}

fn compress(xml: &str) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(xml.as_bytes())
        .expect("could not compress");
    encoder.finish().unwrap()
}

fn encode(dom: &RefNode) -> String {
    dom.to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
