use lltz::compiler::compile;
use lltz::lltz_ir::{Arg, Const, Function, Instruction, Program, Register, Type, Value};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //enum PKind {
    //    Fukui,
    //    Kyoto,
    //    Osaka,
    //    Shiga,
    //};
    //
    //struct Prefecture {
    //    enum PKind kind;
    //    int population;
    //};
    //
    ////Fish Kind
    //enum FKind {
    //    Ayu,
    //    Ika,
    //    Kisu,
    //    Sanma
    //};
    //
    //struct Fish {
    //    enum FKind kind;
    //    int size;
    //    int weight;
    //    struct Prefecture p;
    //};
    //
    //struct Pair main(struct Parameter p, struct Storage s) {
    //{
    //    struct Fish fish;
    //    fish.kind = Ika;
    //    fish.size = 30;
    //    fish.weight = 800;
    //    fish.p.kind = Osaka;
    //    fish.p.population = 8800000;
    //    return 0;
    //}
    //

    //
    //%struct.Fish = type { Int, Int, Int, %struct.Prefecture }
    //%struct.Prefecture = type { Int, Int }
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
    //  %1 = alloca Int, align 4
    //  %2 = alloca %struct.Fish, align 4
    //  store Int 0, Int* %1, align 4
    //  %3 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 0
    //  store Int 1, Int* %3, align 4
    //  %4 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 1
    //  store Int 30, Int* %4, align 4
    //  %5 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 2
    //  store Int 800, Int* %5, align 4
    //  %6 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 3
    //  %7 = getelementptr inbounds %struct.Prefecture, %struct.Prefecture* %6, Int 0, Int 0
    //  store Int 2, Int* %7, align 4
    //  %8 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 3
    //  %9 = getelementptr inbounds %struct.Prefecture, %struct.Prefecture* %8, Int 0, Int 1
    //  store Int 8800000, Int* %9, align 4
    //  FIXME: return void
    //  ret Int 0
    //}

    //{{
    //%struct.Prefecture = type { Int, Int }
    let struct_prefecture = Type::Struct {
        id: String::from("Prefecture"),
        fields: vec![Type::Int, Type::Int],
    };

    //%struct.Fish = type { Int, Int, Int, %struct.Prefecture }
    let struct_fish = Type::Struct {
        id: String::from("Fish"),
        fields: vec![Type::Int, Type::Int, Type::Int, struct_prefecture.clone()],
    };

    let instructions = vec![
        //  %1 = alloca Int, align 4
        Instruction::Alloca {
            ptr: Register::new("%1"),
            ty: Type::Int,
        },
        //  %2 = alloca %struct.Fish, align 4
        Instruction::Alloca {
            ptr: Register::new("%2"),
            ty: struct_fish.clone(),
        },
        //  store Int 0, Int* %1, align 4
        Instruction::Store {
            ty: Type::Int,
            value: Value::Const(Const::Int(0)),
            ptr: Register::new("%1"),
        },
        //  %3 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 0
        Instruction::GetElementPtr {
            result: Register::new("%3"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(0))),
            ],
        },
        //  store Int 1, Int* %3, align 4
        Instruction::Store {
            ty: Type::Int,
            value: Value::Const(Const::Int(1)),
            ptr: Register::new("%3"),
        },
        //  %4 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 1
        Instruction::GetElementPtr {
            result: Register::new("%4"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(1))),
            ],
        },
        //  store Int 30, Int* %4, align 4
        Instruction::Store {
            ty: Type::Int,
            value: Value::Const(Const::Int(30)),
            ptr: Register::new("%4"),
        },
        //  %5 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 2
        Instruction::GetElementPtr {
            result: Register::new("%5"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(2))),
            ],
        },
        //  store Int 800, Int* %5, align 4
        Instruction::Store {
            ty: Type::Int,
            value: Value::Const(Const::Int(800)),
            ptr: Register::new("%5"),
        },
        //  %6 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 3
        Instruction::GetElementPtr {
            result: Register::new("%6"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(3))),
            ],
        },
        //  %7 = getelementptr inbounds %struct.Prefecture, %struct.Prefecture* %6, Int 0, Int 0
        Instruction::GetElementPtr {
            result: Register::new("%7"),
            ty: struct_prefecture.clone(),
            ptrval: Register::new("%6"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(0))),
            ],
        },
        //  store Int 2, Int* %7, align 4
        Instruction::Store {
            ty: Type::Int,
            value: Value::Const(Const::Int(2)),
            ptr: Register::new("%7"),
        },
        //  %8 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 3
        Instruction::GetElementPtr {
            result: Register::new("%8"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(3))),
            ],
        },
        //  %9 = getelementptr inbounds %struct.Prefecture, %struct.Prefecture* %8, Int 0, Int 1
        Instruction::GetElementPtr {
            result: Register::new("%9"),
            ty: struct_prefecture.clone(),
            ptrval: Register::new("%8"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(1))),
            ],
        },
        //  store Int 8800000, Int* %9, align 4
        Instruction::Store {
            ty: Type::Int,
            value: Value::Const(Const::Int(8800000)),
            ptr: Register::new("%9"),
        },
    ];

    //}}

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
        structure_types: vec![
            parameter.clone(),
            storage.clone(),
            pair.clone(),
            struct_prefecture.clone(),
            struct_fish.clone(),
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

    let michelson_code = compile(lltz_ir);

    let file_name = "simple_struct2";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    //println!("{}", michelson_code);
}
