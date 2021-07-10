use ableton_rack_converter;
use xml_dom::level2::RefNode;

fn main() {
    let file_load = String::from("C:\\Users\\janka\\Documents\\Projects\\Programming\\ableton-rack-converter\\validation\\Utility10.adg");
    let file_save = String::from("C:\\Users\\janka\\Documents\\Projects\\Programming\\ableton-rack-converter\\validation\\Utility10_save.adg");
    let device: RefNode = ableton_rack_converter::load_adg(&file_load);
    ableton_rack_converter::save_adg(&device, &file_save);
}
