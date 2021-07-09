use ableton_rack_converter::load_adg;

fn main() {
    let filename = String::from("C:\\Users\\janka\\Documents\\Projects\\Programming\\ableton-rack-converter\\validation\\Utility10.adg");
    let device = load_adg(&filename);
}
