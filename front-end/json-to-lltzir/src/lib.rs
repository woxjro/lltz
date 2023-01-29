use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

use String as Label;
use String as Register;

#[derive(Serialize, Deserialize, Debug)]
struct Cfg {
    blocks: Vec<Label>,
    exiting: Label,
    header: Label,
    header_cond: Register,
    latch: Label,
    parent: Option<Label>,
    r#loop: Label,
    sub_loops: Vec<Cfg>,
}

impl Cfg {
    pub fn print(&self) {
        println!("while {{ {} }} (%{}) {{", self.header, self.header_cond);
        for lp in &self.sub_loops {
            lp.print();
        }
        println!("  {}", self.latch);
        println!("}} {}", self.exiting);
    }
}

pub fn main() {
    let file_path = "cfg.for.cond.json";

    let mut file = File::open(file_path).unwrap();
    let mut json_string = String::new();
    file.read_to_string(&mut json_string).unwrap();
    //let point = Point { x: 1, y: 2 };

    //let serialized = serde_json::to_string(&point).unwrap();
    //println!("serialized = {}", serialized);

    let deserialized: Cfg = serde_json::from_str(&json_string).unwrap();
    deserialized.print();
    //dbg!(&deserialized);
    //println!("deserialized = {:?}", deserialized);
}
