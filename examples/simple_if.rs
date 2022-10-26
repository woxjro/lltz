use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{
    Arg, Condition, Function, Instruction, MiniLlvm, Register, Type,
};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    // struct Pair main(struct Parameter p, struct Storage s) {
    //     struct Pair res;
    //     int a = 0;
    //     int b;
    //     if (a == 0) {
    //         b = 777;
    //     } else {
    //         b = 444;
    //     }
    //     return res;
    // }

    // LLVM IR
    //
    // %struct.Parameter = type {}
    // %struct.Storage   = type {}
    // %struct.Operation = type {}
    // %struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    //
    //define dso_local void @smart_contract(
    //  %struct.Pair* noalias sret %pair,
    //  %struct.Parameter* byval(%struct.Parameter) align 8 %parameter,
    //  %struct.Storage* byval(%struct.Storage) align 8 %storage
    //) #0 {
    //   %1 = alloca i32, align 4
    //   %2 = alloca i32, align 4
    //   %3 = alloca i32, align 4
    //   store i32 0, i32* %1, align 4
    //   store i32 0, i32* %2, align 4
    //   %4 = load i32, i32* %2, align 4
    //   %5 = icmp eq i32 %4, 0
    //   br i1 %5, label %6, label %7
    //
    // 6:                                                ; preds = %0
    //   store i32 777, i32* %3, align 4
    //   br label %8
    //
    // 7:                                                ; preds = %0
    //   store i32 444, i32* %3, align 4
    //   br label %8
    //
    // 8:                                                ; preds = %7, %6
    //   ret i32 0
    // }

    // mini LLVM IR
    //define dso_local void @smart_contract(
    //  %struct.Pair* noalias sret %pair,
    //  %struct.Parameter* byval(%struct.Parameter) align 8 %parameter,
    //  %struct.Storage* byval(%struct.Storage) align 8 %storage
    //) #0 {
    //   %1 = alloca i32, align 4
    //   %2 = alloca i32, align 4
    //   %3 = alloca i32, align 4
    //   store i32 0, i32* %1, align 4
    //   store i32 0, i32* %2, align 4
    //   %4 = load i32, i32* %2, align 4
    //   %5 = icmp eq i32 %4, 0
    //
    //   if i1 %5, {
    //      store i32 444, i32* %3, align 4
    //   } {
    //      store i32 777, i32* %3, align 4
    //   }
    //
    //  FIXME: return void
    //   ret i32 0
    // }

    let instructions = vec![
        //{{
        //   %1 = alloca i32, align 4
        //   %2 = alloca i32, align 4
        //   %3 = alloca i32, align 4
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
            ty: Type::I32,
        },
        //   store i32 0, i32* %1, align 4
        //   store i32 0, i32* %2, align 4
        //   %4 = load i32, i32* %2, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("0"),
            ptr: Register::new("%1"),
        },
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("0"),
            ptr: Register::new("%2"),
        },
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%4"),
            ptr: Register::new("%2"),
        },
        //   %5 = icmp eq i32 %4, 0
        //
        //   if i1 %5, {
        //      store i32 777, i32* %3, align 4
        //      br label %8
        //   } {
        //      store i32 444, i32* %3, align 4
        //      br label %8
        //   }
        Instruction::Icmp {
            ty: Type::I32,
            cond: Condition::Eq,
            result: Register::new("%5"),
            op1: Register::new("%4"),
            op2: Register::new("0"),
        },
        Instruction::If {
            reg: Register::new("%5"),
            code_block_t: vec![Instruction::Store {
                ty: Type::I32,
                value: Register::new("777"),
                ptr: Register::new("%3"),
            }],
            code_block_f: vec![Instruction::Store {
                ty: Type::I32,
                value: Register::new("444"),
                ptr: Register::new("%3"),
            }],
        },
        //   ret i32 0
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

    let file_name = "simple_if";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
