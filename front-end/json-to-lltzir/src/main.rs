use json_to_lltzir::Cfg;
use std::fs::File;
use std::io::prelude::*;
pub fn main() {
    let file_path = "cfg.for.cond.json";

    let mut file = File::open(file_path).unwrap();
    let mut json_string = String::new();
    file.read_to_string(&mut json_string).unwrap();

    let deserialized: Cfg = serde_json::from_str(&json_string).unwrap();
    deserialized.print();
}
