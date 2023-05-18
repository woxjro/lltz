use lltz::json::mlir::ast::{get_smart_contract_operation, Block};
use lltz::michelify::compile;
use lltz::tools::example::emit_file;
use std::process::Command;

pub fn main() {
    let res = Command::new("michelson-mlir-opt")
        .args([
            "--dump-json",
            "--irdl-file=./mlir/dialect/irdl/michelson.irdl.mlir",
            "./examples/mlir/simplest.mlir",
        ])
        .output()
        .unwrap()
        .stderr;
    let json = String::from_utf8(res).unwrap();

    let deserialized: Block = serde_json::from_str(&json).unwrap();
    let smart_contract = get_smart_contract_operation(deserialized).unwrap();

    emit_file(
        "mlir_simplest",
        "Unit",
        "Unit",
        compile(smart_contract.into()),
    );
}
