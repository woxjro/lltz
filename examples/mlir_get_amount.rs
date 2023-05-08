use lltz::json_to_mlir::{get_smart_contract_operation, Block};
use std::process::Command;
pub fn main() {
    let res = Command::new("michelson-mlir-opt")
        .args([
            "--dump-json",
            "--irdl-file=./mlir/dialect/irdl/michelson.irdl.mlir",
            "./examples/mlir/get_amount.mlir",
        ])
        .output()
        .unwrap()
        .stderr;
    let json = String::from_utf8(res).unwrap();

    let deserialized: Block = serde_json::from_str(&json).unwrap();
    let smart_contract = get_smart_contract_operation(deserialized).unwrap();
    dbg!(&smart_contract.regions[0].blocks[0].arguments);
    let ops = &smart_contract.regions[0].blocks[0].operations;

    for op in ops {
        println!("{}", op.name,);
    }
}
