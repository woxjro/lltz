use lltz::compiler::compile;
use lltz::lltz_ir::{Arg, Const, Function, Instruction, Program, Register, Type, Value};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //
    //struct Pair main(struct Parameter p, struct Storage s) {
    //  struct Pair res;
    //  Mutez arr[5];
    //  arr[0] = 31;
    //  arr[4] = 777;
    //  return 0;
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
    //  %2 = alloca [5 x Mutez], align 16
    //  %3 = getelementptr inbounds [5 x Mutez], [5 x Mutez]* %2, i64 0, i64 0, !dbg !19
    //  store Mutez 31, Mutez* %3, align 16, !dbg !20
    //  %4 = getelementptr inbounds [5 x Mutez], [5 x Mutez]* %2, i64 0, i64 4, !dbg !21
    //  store Mutez 777, Mutez* %4, align 16, !dbg !22
    //  ret i32 0, !dbg !23
    //}

    let instructions = vec![
        //  %2 = alloca [5 x Mutez], align 16
        Instruction::Alloca {
            ptr: Register::new("%2"),
            ty: Type::Array {
                size: 5,
                elementtype: Box::new(Type::Mutez),
            },
        },
        //  %3 = getelementptr inbounds [5 x Mutez], [5 x Mutez]* %2, i64 0, i64 0, !dbg !19
        //  store Mutez 31, Mutez* %3, align 16, !dbg !20
        Instruction::GetElementPtr {
            result: Register::new("%3"),
            ty: Type::Array {
                size: 5,
                elementtype: Box::new(Type::Mutez),
            },
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(0))),
            ],
        },
        Instruction::Store {
            ty: Type::Mutez,
            value: Value::Const(Const::Mutez(31)),
            ptr: Register::new("%3"),
        },
        //  %4 = getelementptr inbounds [5 x Mutez], [5 x Mutez]* %2, i64 0, i64 4, !dbg !21
        //  store Mutez 777, Mutez* %4, align 16, !dbg !22
        Instruction::GetElementPtr {
            result: Register::new("%4"),
            ty: Type::Array {
                size: 5,
                elementtype: Box::new(Type::Mutez),
            },
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(4))),
            ],
        },
        Instruction::Store {
            ty: Type::Mutez,
            value: Value::Const(Const::Mutez(777)),
            ptr: Register::new("%4"),
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

    let file_name = "simple_array";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
