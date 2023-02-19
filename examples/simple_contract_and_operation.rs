use lltz::compiler::compile;
use lltz::lltz_ir::{Arg, Const, Function, Instruction, Program, Register, Type, Value};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //
    //typedef char* Address;
    //struct Parameter {}
    //struct Storage {}
    //struct Pair {
    //  Operation ops[3]
    //}

    //Operation transfer_tokens(struct Parameter param, Mutez tokens, Contract contract) {
    //  return DUMMY_OPERATION;
    //}
    //
    //struct Pair smart_contract(struct Parameter param, struct Storage storage) {
    //    struct Pair p;
    //    Address addr = "tz1ddb9NMYHZi5UzPdzTZMYQQZoMub195zgv";
    //    Contract contract = get_contract(addr);
    //    struct Parameter param2;
    //    struct Operation op = transfer_tokens(param2, 100, contract);
    //    p.ops[1] = op;
    //    return p;
    //};
    //
    //
    //define dso_local void @smart_contract(
    //      %struct.Pair* noalias sret %0,
    //      %struct.Storage* byval(%struct.Storage) %1
    //      %struct.Parameter* byval(%struct.Parameter) %2
    //) #0 {
    //  %4 = alloca Address
    //  %5 = alloca Contract
    //  %7 = alloca Operation
    //  store Address "tz1ddb9NMYHZi5UzPdzTZMYQQZoMub195zgv", Address* %4
    //  %8 = load Address, Address* %4
    //  %9 = call Contract @get_contract(i8* %8)
    //  store Contract %9, Contract* %5
    //  %10 = load Contract, Contract* %5
    //  %11 = call Operation @transfer_tokens(Mutez 100, Contract %10)
    //  store Operation %11, Operation* %7
    //  %12 = load Operation, Operation* %7
    //  %13 = getelementptr inbounds %struct.Pair, %struct.Pair* %0, i32 0, i32 0
    //  %14 = getelementptr inbounds [3 x Operation], [3 x Operation]* %13, i64 0, i64 1
    //  store Operation %12, Operation* %14
    //  ret void
    //}
    //

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
                size: 3,
                elementtype: Box::new(Type::Operation),
            },
            storage.clone(),
        ],
    };

    let instructions = vec![
        //  %4 = alloca Address, align 8
        //  %5 = alloca Contract, align 4
        //  %7 = alloca Operation, align 4
        Instruction::Alloca {
            ptr: Register::new("%4"),
            ty: Type::Address,
        },
        Instruction::Alloca {
            ptr: Register::new("%5"),
            ty: Type::Contract(Box::new(Type::Struct {
                id: String::from("unit"),
                fields: vec![],
            })),
        },
        Instruction::Alloca {
            ptr: Register::new("%7"),
            ty: Type::Operation,
        },
        //  store Address "tz1ddb9NMYHZi5UzPdzTZMYQQZoMub195zgv", Address* %4, align 8, !dbg !77
        //  %8 = load Address, Address* %4, align 8, !dbg !80
        Instruction::Store {
            ty: Type::Address,
            //value: Register::new("\"KT1Vh2yUNseYabMc1c9EKiBbtQxbyoRWAFDv\""),
            value: Value::Const(Const::Address(
                "\"tz1ddb9NMYHZi5UzPdzTZMYQQZoMub195zgv\"".to_string(),
            )),
            ptr: Register::new("%4"),
        },
        Instruction::Load {
            result: Register::new("%8"),
            ty: Type::Address,
            ptr: Register::new("%4"),
        },
        //  %9 = call Contract @get_contract(i8* %8), !dbg !81
        //  store Contract %9, Contract* %5, align 4, !dbg !79
        Instruction::MichelsonContract {
            result: Register::new("%9"),
            ty: Type::Struct {
                id: String::from("unit"),
                fields: vec![],
            },
            address: Register::new("%8"),
        },
        Instruction::MichelsonAssertSome {
            result: Register::new("%200"),
            ty: Type::Option(Box::new(Type::Contract(Box::new(Type::Struct {
                id: String::from("unit"),
                fields: vec![],
            })))),
            value: Register::new("%9"),
        },
        Instruction::Store {
            ty: Type::Contract(Box::new(Type::Struct {
                id: String::from("unit"),
                fields: vec![],
            })),
            value: Value::Register(Register::new("%200")),
            ptr: Register::new("%5"),
        },
        //  %10 = load Contract, Contract* %5, align 4, !dbg !86
        //  %11 = call Operation @transfer_tokens(Mutez 100, Contract %10), !dbg !87
        //  store Operation %11, Operation* %7, align 4, !dbg !85
        Instruction::Load {
            result: Register::new("%10"),
            ty: Type::Contract(Box::new(Type::Struct {
                id: String::from("unit"),
                fields: vec![],
            })),
            ptr: Register::new("%5"),
        },
        Instruction::MichelsonTransferTokens {
            result: Register::new("%11"),
            init: Register::new("%100"), //FIXME, TODO とりあえずいまは動く
            tokens: Register::new("100"),
            contract: Register::new("%10"),
        },
        Instruction::Store {
            ty: Type::Operation,
            value: Value::Register(Register::new("%11")),
            ptr: Register::new("%7"),
        },
        //  %12 = load Operation, Operation* %7, align 4, !dbg !88
        Instruction::Load {
            result: Register::new("%12"),
            ty: Type::Operation,
            ptr: Register::new("%7"),
        },
        //  %13 = getelementptr inbounds %struct.Pair, %struct.Pair* %0, i32 0, i32 0, !dbg !89
        Instruction::GetElementPtr {
            result: Register::new("%13"),
            ty: pair.clone(),
            ptrval: Register::new("%0"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(0))),
            ],
        },
        //  %14 = getelementptr inbounds [3 x Operation], [3 x Operation]* %13, i64 0, i64 1, !dbg !90
        Instruction::GetElementPtr {
            result: Register::new("%14"),
            ty: Type::Array {
                size: 3,
                elementtype: Box::new(Type::Operation),
            },
            ptrval: Register::new("%13"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(1))),
            ],
        },
        //  store Operation %12, Operation* %14, align 4, !dbg !91
        Instruction::Store {
            ty: Type::Operation,
            value: Value::Register(Register::new("%12")),
            ptr: Register::new("%14"),
        },
    ];

    let lltz_ir = Program {
        structure_types: vec![parameter.clone(), storage.clone(), pair.clone()],
        functions: vec![Function {
            function_name: String::from("smart_contract"),
            result_type: Type::Int,
            argument_list: vec![
                Arg {
                    ty: Type::Ptr(Box::new(pair.clone())),
                    reg: Register::new("%0"),
                },
                Arg {
                    ty: Type::Ptr(Box::new(storage.clone())),
                    reg: Register::new("%1"),
                },
                Arg {
                    ty: Type::Ptr(Box::new(parameter.clone())),
                    reg: Register::new("%2"),
                },
            ],
            instructions,
        }],
    };

    let michelson_code = compile(lltz_ir);

    let file_name = "simple_contract_and_operation";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
