use clap::Parser;
use lltz::{
    json::mlir::ast::{get_smart_contract_operation, Block},
    michelify::compile,
};
use std::{fs::File, io::prelude::*, process::Command};

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

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let res = Command::new("./mlir/build/bin/michelson-mlir-opt")
        .args([
            "--dump-json",
            "--irdl-file=./mlir/dialect/irdl/michelson.irdl.mlir",
            &args.input,
        ])
        .output()?
        .stderr;
    let json = String::from_utf8(res)?;

    println!("{json}");

    let deserialized: Block = serde_json::from_str(&json)?;
    let smart_contract = get_smart_contract_operation(deserialized)?;

    let michelson_code = compile(smart_contract.into())?.to_string();

    match args.output {
        Some(output) => {
            let contents = format!(
                "{command_typecheck}{command_mock}{michelson_code}",
                command_typecheck = format!(
                    "#octez-client --mode mockup --base-dir \
                        /tmp/mockup typecheck script {output}\n"
                ),
                command_mock = format!(
                    "#octez-client --mode mockup --base-dir /tmp/mockup \
                        run script {output} on storage '' and input '' --trace-stack\n"
                )
            );
            let mut file = File::create(output)?;
            file.write_all(contents.as_bytes())?;
        }
        None => {
            println!("{michelson_code}");
        }
    }
    Ok(())
}
