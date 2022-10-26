use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{
    Arg, Function, Instruction, MiniLlvm, Register, Type,
};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //%struct.Parameter = type { i32, i32, i32, %struct.Fish }
    let parameter = Type::Struct {
        id: String::from("Parameter"),
        fields: vec![Type::I32, Type::I32],
    };
    let fish = Type::Struct {
        id: String::from("Fish"),
        fields: vec![Type::I32, Type::I32, Type::I32],
    };

    //%struct.Storage = type { i32, i32, i32, i32, %struct.Fish }
    let storage = Type::Struct {
        id: String::from("Storage"),
        //fields: vec![Type::I32, Type::I32, fish.clone()],
        fields: vec![Type::I32, fish.clone(), Type::I32],
    };

    //%struct.Operation = type {}
    let operation = Type::Struct {
        id: String::from("Operation"),
        fields: vec![],
    };

    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    let pair = Type::Struct {
        id: String::from("Pair"),
        // FIXME: [0 x %struct.Operation]にしたい.
        //        配列をサポートしていない
        fields: vec![operation.clone(), storage.clone()],
    };

    let mini_llvm = MiniLlvm {
        structure_types: vec![
            operation.clone(),
            pair.clone(),
            parameter.clone(),
            fish.clone(),
            storage.clone(),
        ],
        functions: vec![
            //define dso_local void @smart_contract(
            //      %struct.Pair* noalias sret %0,
            //      %struct.Parameter* byval(%struct.Parameter) align 8 %1,
            //      %struct.Storage* byval(%struct.Storage) align 8 %2
            //) #0 {
            Function {
                //define dso_local void @smart_contract(
                function_name: String::from("smart_contract"),
                //FIXME: i32ではない. void
                result_type: Type::I32,
                //      %struct.Pair* noalias sret %0,
                //      %struct.Parameter* byval(%struct.Parameter) align 8 %1,
                //      %struct.Storage* byval(%struct.Storage) align 8 %2
                argument_list: vec![
                    Arg {
                        ty: Type::Ptr(Box::new(pair.clone())),
                        reg: Register::new("%0"),
                    },
                    Arg {
                        ty: Type::Ptr(Box::new(parameter.clone())),
                        reg: Register::new("%1"),
                    },
                    Arg {
                        ty: Type::Ptr(Box::new(storage.clone())),
                        reg: Register::new("%2"),
                    },
                ],
                instructions: vec![
                    Instruction::GetElementPtr {
                        result: Register::new("%3"),
                        ty: pair.clone(),
                        ptrval: Register::new("%0"),
                        subsequent: vec![
                            (Type::I32, Register::new("0")),
                            (Type::I32, Register::new("1")),
                        ],
                    },
                    Instruction::LlvmMemcpy {
                        dest: Register::new("%3"),
                        src: Register::new("%2"),
                        ty: storage.clone(),
                    },
                ],
            },
        ],
    };

    let michelson_code = compile(mini_llvm);

    let file_name = "simple_smartcontract";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Pair 1 (Pair 2 3 4) 5' and input 'Pair 6 7' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
