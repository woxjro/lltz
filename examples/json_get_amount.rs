use lltz::json_to_mlir::{get_smart_contract_operation, Block};
use std::fs::File;
use std::io::prelude::*;
pub fn main() {
    let file_path = "examples/json/get_amount.json";

    let mut file = File::open(file_path).unwrap();
    let mut json_string = String::new();
    file.read_to_string(&mut json_string).unwrap();

    let deserialized: Block = serde_json::from_str(&json_string).unwrap();
    let smart_contract = get_smart_contract_operation(deserialized).unwrap();
    dbg!(&smart_contract.regions[0].blocks[0].arguments);
    let ops = &smart_contract.regions[0].blocks[0].operations;

    for op in ops {
        println!("{}", op.name,);
    }
}
