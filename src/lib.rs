use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::PathBuf,
    str,
};
use xml_dom::{level2::RefNode, parser};

pub mod fixers;

pub fn validate_filetype(ft: &str) -> bool {
    // als - ableton live set
    // adg - ableton device group
    // adv - ableton device preset
    let types = ["adg", "als", "adv"];
    if types.contains(&ft) {
        return true;
    }
    return false;
}

pub fn load_ableton_file(filename: PathBuf) -> RefNode {
    decode(&load_raw_decompressed_file(filename))
}

pub fn load_raw_decompressed_file(filename: PathBuf) -> String {
    let contents = File::open(filename).expect("failed to load file");
    return decompress(contents);
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

pub fn save_ableton_file(dom: &RefNode, filename: PathBuf) {
    let xml = encode(dom);
    let compressed = compress(&xml);
    fs::write(filename, compressed).expect("could not write file");
}

pub fn save_uncompressed_xmldom(dom: &RefNode, filename: PathBuf) {
    let xml = encode(dom);
    fs::write(filename, xml.as_bytes()).expect("could not write file");
}
pub fn save_uncompressed_raw(raw: String, filename: PathBuf) {
    fs::write(filename, raw.as_bytes()).expect("could not write file");
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
