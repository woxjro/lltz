use lltz::json::mlir::ast::{get_smart_contract_operation, Block};
use lltz::michelify::compile;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

pub fn main() {
    let res = Command::new("michelson-mlir-opt")
        .args([
            "--dump-json",
            "--irdl-file=./mlir/dialect/irdl/michelson.irdl.mlir",
            "./examples/mlir/boomerang.mlir",
        ])
        .output()
        .unwrap()
        .stderr;
    let json = String::from_utf8(res).unwrap();

    let deserialized: Block = serde_json::from_str(&json).unwrap();
    let smart_contract = get_smart_contract_operation(deserialized).unwrap();

    let michelson_code = compile(smart_contract.into());
    println!("{michelson_code}");

    let file_name = "mlir_boomerang";
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}",
            command_typecheck = format!("#tezos-client --mode mockup --base-dir \
                /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n"),
            command_mock = format!("#tezos-client --mode mockup --base-dir /tmp/mockup \
                run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n")
        );
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
