use flate2::read::GzDecoder;
use minidom::Element;
use std::{fs::File, io::Read, str};

pub fn load_adg(filename: &str) -> AbletonDeviceGroup {
    let contents = File::open(filename).unwrap();
    let xml = decompress(contents);
    let stripped = strip_first_line(&xml);
    // println!("{}", stripped);
    decode(&stripped);
    AbletonDeviceGroup::new(xml)
}

fn decompress(loaded_file: File) -> String {
    let mut decoder = GzDecoder::new(loaded_file);
    let mut decompressed = String::new();
    decoder.read_to_string(&mut decompressed).unwrap();
    decompressed
}

fn decode(xml: &str) {
    let root: Result<Element, _> = xml.parse();
    println!("{:?}", root);
    // let root2 = root.unwrap();
}

fn strip_first_line(text: &str) -> &str {
    let bytes = text.trim().as_bytes();
    for (i, &val) in bytes.iter().enumerate() {
        if val == b'\n' {
            return &text[i + 1..bytes.len()];
        }
    }
    &text
}

pub struct AbletonDeviceGroup {
    body: String,
}

impl AbletonDeviceGroup {
    pub fn new(body: String) -> AbletonDeviceGroup {
        AbletonDeviceGroup { body }
    }
    pub fn get_body(&self) -> &str {
        &self.body
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
