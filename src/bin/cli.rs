use clap::Parser;
use lltz::json::mlir::ast::{get_smart_contract_operation, Block};
use lltz::michelify::compile;
use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// input file path
    #[arg(short, long)]
    input: String,

    /// output file path
    #[arg(short, long)]
    output: Option<String>,
}

pub fn main() {
    let args = Args::parse();

    let res = Command::new("michelson-mlir-opt")
        .args([
            "--dump-json",
            "--irdl-file=./mlir/dialect/irdl/michelson.irdl.mlir",
            &args.input,
        ])
        .output()
        .unwrap()
        .stderr;
    let json = String::from_utf8(res).unwrap();

    let deserialized: Block = serde_json::from_str(&json).unwrap();
    let smart_contract = get_smart_contract_operation(deserialized).unwrap();

    let michelson_code = compile(smart_contract.into());

    match args.output {
        Some(output) => {
            let contents = format!(
                "{command_typecheck}{command_mock}{michelson_code}",
                command_typecheck = format!(
                    "#tezos-client --mode mockup --base-dir \
                        /tmp/mockup typecheck script {output}\n"
                ),
                command_mock = format!(
                    "#tezos-client --mode mockup --base-dir /tmp/mockup \
                        run script {output} on storage '0' and input 'Unit' --trace-stack\n"
                )
            );
            let mut file = File::create(output).unwrap();
            file.write_all(contents.as_bytes()).unwrap();
        }
        None => {
            println!("{michelson_code}");
        }
    }
}
