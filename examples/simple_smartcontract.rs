use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{
    Function, Instruction, MiniLlvm, Opcode, Register, Type,
};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    let mini_llvm = MiniLlvm {
        structure_types: vec![],
        functions: vec![Function {
            function_name: String::from("main"),
            result_type: Type::I32,
            argument_list: vec![],
            instructions: vec![],
        }],
    };

    let michelson_code = compile(mini_llvm);

    let file_name = "simple_smartcontract";
    let command_typecheck =
        format!("#tezos-client --mode mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    println!("{}", michelson_code);
}
