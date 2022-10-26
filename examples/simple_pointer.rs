use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{
    Arg, Function, Instruction, MiniLlvm, Opcode, Register, Type,
};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //struct Pair main(struct Parameter p, struct Storage s) {
    //    struct Pair res;
    //    int a = 99;
    //    int* pa = &a;
    //    int** ppa = &pa;
    //    **ppa += 33;
    //    return 0;
    //}

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
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32*, align 8
    //  %4 = alloca i32**, align 8
    //  store i32 0, i32* %1, align 4
    //  store i32 99, i32* %2, align 4
    //  store i32* %2, i32** %3, align 8
    //  store i32** %3, i32*** %4, align 8
    //  %5 = load i32**, i32*** %4, align 8
    //  %6 = load i32*, i32** %5, align 8
    //  %7 = load i32, i32* %6, align 4
    //  %8 = add nsw i32 %7, 33
    //  store i32 %8, i32* %6, align 4
    //  ret i32 0
    //}

    //
    //

    let instructions = vec![
        //{{
        //
        //  %1 = alloca i32, align 4
        //  %2 = alloca i32, align 4
        //  %3 = alloca i32*, align 8
        //  %4 = alloca i32**, align 8
        Instruction::Alloca {
            ptr: Register::new("%1"),
            ty: Type::I32,
        },
        Instruction::Alloca {
            ptr: Register::new("%2"),
            ty: Type::I32,
        },
        Instruction::Alloca {
            ptr: Register::new("%3"),
            ty: Type::Ptr(Box::new(Type::I32)),
        },
        Instruction::Alloca {
            ptr: Register::new("%4"),
            ty: Type::Ptr(Box::new(Type::Ptr(Box::new(Type::I32)))),
        },
        //  store i32 0, i32* %1, align 4
        //  store i32 99, i32* %2, align 4
        //  store i32* %2, i32** %3, align 8
        //  store i32** %3, i32*** %4, align 8
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("0"),
            ptr: Register::new("%1"),
        },
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("99"),
            ptr: Register::new("%2"),
        },
        Instruction::Store {
            ty: Type::Ptr(Box::new(Type::I32)),
            value: Register::new("%2"),
            ptr: Register::new("%3"),
        },
        Instruction::Store {
            ty: Type::Ptr(Box::new(Type::Ptr(Box::new(Type::I32)))),
            value: Register::new("%3"),
            ptr: Register::new("%4"),
        },
        //  %5 = load i32**, i32*** %4, align 8
        //  %6 = load i32*, i32** %5, align 8
        //  %7 = load i32, i32* %6, align 4
        Instruction::Load {
            ty: Type::Ptr(Box::new(Type::Ptr(Box::new(Type::I32)))),
            result: Register::new("%5"),
            ptr: Register::new("%4"),
        },
        Instruction::Load {
            ty: Type::Ptr(Box::new(Type::I32)),
            result: Register::new("%6"),
            ptr: Register::new("%5"),
        },
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%7"),
            ptr: Register::new("%6"),
        },
        //  %8 = add nsw i32 %7, 33
        //  store i32 %8, i32* %6, align 4
        Instruction::Op {
            ty: Type::I32,
            opcode: Opcode::Add,
            result: Register::new("%8"),
            op1: Register::new("%7"),
            op2: Register::new("33"),
        },
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("%8"),
            ptr: Register::new("%6"),
        },
        //  ret i32 0
        Instruction::Ret {
            ty: Type::I32,
            value: Register::new("0"),
        },
        //}}
    ];

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
            result_type: Type::I32,
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

    let file_name = "simple_pointer";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
