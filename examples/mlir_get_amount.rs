use lltz::json::mlir::ast::{get_smart_contract_operation, Block};
use lltz::michelify::compile;
use lltz::tools;
use std::process::Command;
pub fn main() {
    let res = Command::new("./mlir/build/bin/michelson-mlir-opt")
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
    let program = compile(smart_contract.into());

    #[cfg(debug_assertions)]
    eprintln!(
        "number of instructions: {}",
        program
            .code
            .iter()
            .map(|instr| instr.count())
            .sum::<usize>()
    );

    tools::example::emit_file("mlir_get_amount", "0", "Unit", program);

    #[cfg(debug_assertions)]
    eprintln!(
        "gas consumption: {}",
        tools::measure::get_gas_consumption("./examples/out/mlir_get_amount.tz", "0", "Unit")
    );
}
