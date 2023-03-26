use lltz::compiler::compile;
use lltz::lltz_ir::{Arg, Const, Function, Instruction, Opcode, Program, Register, Type, Value};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //
    //struct Pair main(struct Parameter p, struct Storage s) {
    //  struct Pair res;
    //  int a = 10;
    //  int b = 20;
    //  int c = a + b;
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
    //  %1 = alloca int, align 4
    //  %2 = alloca int, align 4
    //  %3 = alloca int, align 4
    //  %4 = alloca int, align 4
    //  store int 0, int* %1, align 4
    //  store int 10, int* %2, align 4
    //  store int 20, int* %3, align 4
    //  %5 = load int, int* %2, align 4
    //  %6 = load int, int* %3, align 4
    //  %7 = add nsw int %5, %6
    //  store int %7, int* %4, align 4
    //  ret int 0
    //}

    let instructions = vec![
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
        Instruction::Alloca {
            ptr: Register::new("%4"),
            ty: Type::Int,
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Const(Const::Int(0)),
            ptr: Register::new("%1"),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Const(Const::Int(10)),
            ptr: Register::new("%2"),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Const(Const::Int(20)),
            ptr: Register::new("%3"),
        },
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%5"),
            ptr: Register::new("%2"),
        },
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%6"),
            ptr: Register::new("%3"),
        },
        Instruction::Op {
            ty: Type::Int,
            opcode: Opcode::Add,
            result: Register::new("%7"),
            op1: Value::Register(Register::new("%5")),
            op2: Value::Register(Register::new("%6")),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Register(Register::new("%7")),
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

    let file_name = "simple_add";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
