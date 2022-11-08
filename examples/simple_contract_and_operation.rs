use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{
    Arg, Function, Instruction, MiniLlvm, Opcode, Register, Type,
};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //
    //struct Pair main(struct Parameter p, struct Storage s) {
    //}
    //
    //%struct.Parameter = type {}
    //%struct.Storage   = type {}
    //%struct.Operation = type {}
    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    //
    //define dso_local void @smart_contract(
    //  %struct.Pair* noalias sret %pair,
    //  %struct.Parameter* byval(%struct.Parameter) align 8 %parameter,
    //  %struct.Storage* byval(%struct.Storage) align 8 %storage
    //) #0 {
    //  %1 = alloca operation
    //  %2 = alloca contract unit // for michelson::SELF
    //  %3 = get_self();
    //  store contract unit %3, %2;
    //  ret int 0
    //}
    let parameter = Type::Struct {
        id: String::from("Parameter"),
        fields: vec![],
    };

    let storage = Type::Struct {
        id: String::from("Storage"),
        fields: vec![],
    };

    let operation = Type::Struct {
        id: String::from("Operation"),
        fields: vec![],
    };

    let instructions = vec![
        Instruction::Alloca {
            ptr: Register::new("%1"),
            ty: Type::Operation,
        },
        Instruction::Alloca {
            ptr: Register::new("%2"),
            ty: Type::Contract(Box::new(parameter.clone())),
        },
        Instruction::MichelsonGetSelf {
            result: Register::new("%3"),
        },
        Instruction::Store {
            ty: Type::Contract(Box::new(parameter.clone())),
            value: Register::new("%3"),
            ptr: Register::new("%2"),
        },
        //%4 = alloca contract unit
        Instruction::Alloca {
            ptr: Register::new("%4"),
            ty: Type::Contract(Box::new(Type::Struct {
                id: String::from("unit"),
                fields: vec![],
            })),
        },
        Instruction::Alloca {
            ptr: Register::new("%5"),
            ty: Type::Address,
        },
        Instruction::Store {
            ty: Type::Address,
            value: Register::new("\"tz1ddb9NMYHZi5UzPdzTZMYQQZoMub195zgv\""),
            ptr: Register::new("%5"),
        },
        Instruction::Load {
            result: Register::new("%6"),
            ty: Type::Address,
            ptr: Register::new("%5"),
        },
        Instruction::MichelsonContract {
            result: Register::new("%7"),
            ty: Type::Struct {
                id: String::from("unit"),
                fields: vec![],
            },
            address: Register::new("%6"),
        },
        Instruction::Alloca {
            ptr: Register::new("%8"),
            ty: Type::Option(Box::new(Type::Contract(Box::new(Type::Struct {
                id: String::from("unit"),
                fields: vec![],
            })))),
        },
        Instruction::Store {
            ty: Type::Option(Box::new(Type::Contract(Box::new(Type::Struct {
                id: String::from("unit"),
                fields: vec![],
            })))),
            value: Register::new("%7"),
            ptr: Register::new("%8"),
        },
        Instruction::Load {
            result: Register::new("%9"),
            ty: Type::Option(Box::new(Type::Contract(Box::new(Type::Struct {
                id: String::from("unit"),
                fields: vec![],
            })))),
            ptr: Register::new("%8"),
        },
        //store option contract unit %7
        //%8 = load option contract unit
        Instruction::MichelsonAssertSome {
            result: Register::new("%10"),
            ty: Type::Option(Box::new(Type::Contract(Box::new(Type::Struct {
                id: String::from("unit"),
                fields: vec![],
            })))),
            value: Register::new("%9"),
        },
    ];

    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    let pair = Type::Struct {
        id: String::from("Pair"),
        // FIXME: [0 x %struct.Operation]にしたい.
        //        配列をサポートしていない
        fields: vec![operation.clone(), storage.clone()],
    };

    let mini_llvm = MiniLlvm {
        structure_types: vec![
            parameter.clone(),
            storage.clone(),
            operation.clone(),
            pair.clone(),
        ],
        functions: vec![Function {
            function_name: String::from("smart_contract"),
            result_type: Type::Int,
            argument_list: vec![
                Arg {
                    ty: Type::Ptr(Box::new(pair.clone())),
                    reg: Register::new("%pair"),
                },
                Arg {
                    ty: Type::Ptr(Box::new(parameter.clone())),
                    reg: Register::new("%parameter"),
                },
                Arg {
                    ty: Type::Ptr(Box::new(storage.clone())),
                    reg: Register::new("%storage"),
                },
            ],
            instructions,
        }],
    };

    let michelson_code = compile(mini_llvm);

    let file_name = "simple_contract_and_operation";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
