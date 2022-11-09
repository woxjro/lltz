use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{
    Arg, Function, Instruction, MiniLlvm, Opcode, Register, Type,
};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //typedef long mutez;
    //struct Pair main(struct Parameter p, struct Storage s) {
    //  struct Pair res;
    //  mutez a = 10;
    //  mutez b = 20;
    //  mutez c = a + b;
    //  return res;
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
    //  %1 = alloca mutez, align 4
    //  %2 = alloca mutez, align 4
    //  %3 = alloca mutez, align 4
    //  %4 = alloca mutez, align 4
    //  store mutez 0, mutez* %1, align 4
    //  store mutez 10, mutez* %2, align 4
    //  store mutez 20, mutez* %3, align 4
    //  %5 = load mutez, mutez* %2, align 4
    //  %6 = load mutez, mutez* %3, align 4
    //  %7 = add nsw mutez %5, %6
    //  store mutez %7, mutez* %4, align 4
    //  ret mutez 0
    //}

    let instructions = vec![
        Instruction::Alloca {
            ptr: Register::new("%1"),
            ty: Type::Mutez,
        },
        Instruction::Alloca {
            ptr: Register::new("%2"),
            ty: Type::Mutez,
        },
        Instruction::Alloca {
            ptr: Register::new("%3"),
            ty: Type::Mutez,
        },
        Instruction::Alloca {
            ptr: Register::new("%4"),
            ty: Type::Mutez,
        },
        Instruction::Store {
            ty: Type::Mutez,
            value: Register::new("0"),
            ptr: Register::new("%1"),
        },
        Instruction::Store {
            ty: Type::Mutez,
            value: Register::new("10"),
            ptr: Register::new("%2"),
        },
        Instruction::Store {
            ty: Type::Mutez,
            value: Register::new("20"),
            ptr: Register::new("%3"),
        },
        Instruction::Load {
            ty: Type::Mutez,
            result: Register::new("%5"),
            ptr: Register::new("%2"),
        },
        Instruction::Load {
            ty: Type::Mutez,
            result: Register::new("%6"),
            ptr: Register::new("%3"),
        },
        Instruction::Op {
            ty: Type::Mutez,
            opcode: Opcode::Add,
            result: Register::new("%7"),
            op1: Register::new("%5"),
            op2: Register::new("%6"),
        },
        Instruction::Store {
            ty: Type::Mutez,
            value: Register::new("%7"),
            ptr: Register::new("%4"),
        },
        Instruction::Ret {
            ty: Type::Mutez,
            value: Register::new("0"),
        },
    ];

    let parameter = Type::Struct {
        id: String::from("Parameter"),
        fields: vec![],
    };

    let storage = Type::Struct {
        id: String::from("Storage"),
        fields: vec![],
    };

    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    let pair = Type::Struct {
        id: String::from("Pair"),
        fields: vec![
            Type::Array {
                size: 0,
                elementtype: Box::new(Type::Operation),
            },
            storage.clone(),
        ],
    };

    let mini_llvm = MiniLlvm {
        structure_types: vec![parameter.clone(), storage.clone(), pair.clone()],
        functions: vec![Function {
            function_name: String::from("smart_contract"),
            result_type: Type::Mutez,
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

    let file_name = "simple_add_mutez";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
