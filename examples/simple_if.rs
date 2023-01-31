use lltz::compiler::compile;
use lltz::lltz_ir::{
    Arg, Condition, Function, Instruction, Program, Register, Type,
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
    //   %1 = alloca int, align 4
    //   %2 = alloca int, align 4
    //   %3 = alloca int, align 4
    //   store int 0, int* %1, align 4
    //   store int 0, int* %2, align 4
    //   %4 = load int, int* %2, align 4
    //   %5 = icmp eq int %4, 0
    //   br bool %5, label %6, label %7
    //
    // 6:                                                ; preds = %0
    //   store int 777, int* %3, align 4
    //   br label %8
    //
    // 7:                                                ; preds = %0
    //   store int 444, int* %3, align 4
    //   br label %8
    //
    // 8:                                                ; preds = %7, %6
    //   ret int 0
    // }

    // LLTZ IR
    //define dso_local void @smart_contract(
    //  %struct.Pair* noalias sret %pair,
    //  %struct.Parameter* byval(%struct.Parameter) align 8 %parameter,
    //  %struct.Storage* byval(%struct.Storage) align 8 %storage
    //) #0 {
    //   %1 = alloca int, align 4
    //   %2 = alloca int, align 4
    //   %3 = alloca int, align 4
    //   store int 0, int* %1, align 4
    //   store int 0, int* %2, align 4
    //   %4 = load int, int* %2, align 4
    //   %5 = icmp eq int %4, 0
    //
    //   if bool %5, {
    //      store int 444, int* %3, align 4
    //   } {
    //      store int 777, int* %3, align 4
    //   }
    //
    //  FIXME: return void
    //   ret int 0
    // }

    let instructions = vec![
        //{{
        //   %1 = alloca int, align 4
        //   %2 = alloca int, align 4
        //   %3 = alloca int, align 4
        Instruction::Alloca {
            ptr: Register::new("%1"),
            ty: Type::Int,
        },
        Instruction::Alloca {
            ptr: Register::new("%2"),
            ty: Type::Int,
        },
        Instruction::Alloca {
            ptr: Register::new("%3"),
            ty: Type::Int,
        },
        //   store int 0, int* %1, align 4
        //   store int 0, int* %2, align 4
        //   %4 = load int, int* %2, align 4
        Instruction::Store {
            ty: Type::Int,
            value: Register::new("0"),
            ptr: Register::new("%1"),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Register::new("0"),
            ptr: Register::new("%2"),
        },
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%4"),
            ptr: Register::new("%2"),
        },
        //   %5 = icmp eq int %4, 0
        //
        //   if bool %5, {
        //      store int 777, int* %3, align 4
        //      br label %8
        //   } {
        //      store int 444, int* %3, align 4
        //      br label %8
        //   }
        Instruction::Icmp {
            ty: Type::Int,
            cond: Condition::Eq,
            result: Register::new("%5"),
            op1: Register::new("%4"),
            op2: Register::new("0"),
        },
        Instruction::If {
            reg: Register::new("%5"),
            code_block_t: vec![Instruction::Store {
                ty: Type::Int,
                value: Register::new("777"),
                ptr: Register::new("%3"),
            }],
            code_block_f: vec![Instruction::Store {
                ty: Type::Int,
                value: Register::new("444"),
                ptr: Register::new("%3"),
            }],
        },
        //   ret int 0
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

    let lltz_ir = Program {
        structure_types: vec![parameter.clone(), storage.clone(), pair.clone()],
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

    let michelson_code = compile(lltz_ir);

    let file_name = "simple_if";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
