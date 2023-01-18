use lltz::compiler::compile;
use lltz::mini_llvm::{
    Arg, Function, Instruction, MiniLlvm, Register, Type,
};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //enum FKind {
    //    Ayu,
    //    Ika,
    //    Kisu
    //};
    //
    //enum Place {
    //    Fukui,
    //    Kyoto,
    //    Osaka,
    //    Shiga,
    //};
    //
    //struct Fish {
    //    enum FKind kind;
    //    enum Place place;
    //    int weight;
    //};
    //
    //int main() {
    //    struct Fish fish;
    //    fish.kind = Ika;
    //    fish.place = Fukui;
    //    fish.weight = 700;
    //    return 0;
    //}

    //%struct.Fish = type { Int, Int, Int }
    //
    //define dso_local Int @main() #0 {
    //  %1 = alloca Int, align 4
    //  %2 = alloca %struct.Fish, align 4
    //  store Int 0, Int* %1, align 4
    //  %3 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 0
    //  store Int 1, Int* %3, align 4
    //  %4 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 1
    //  store Int 0, Int* %4, align 4
    //  %5 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 2
    //  store Int 700, Int* %5, align 4
    //  ret Int 0
    //}

    let instructions = vec![
        //{{
        //  %1 = alloca Int, align 4
        //  %2 = alloca %struct.Fish, align 4
        //  store Int 0, Int* %1, align 4
        Instruction::Alloca {
            ptr: Register::new("%1"),
            ty: Type::Int,
        },
        Instruction::Alloca {
            ptr: Register::new("%2"),
            ty: Type::Struct {
                id: String::from("Fish"),
                fields: vec![Type::Int, Type::Int, Type::Int],
            },
        },
        Instruction::Store {
            ty: Type::Int,
            value: Register::new("0"),
            ptr: Register::new("%1"),
        },
        //  %3 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 0
        //  store Int 1, Int* %3, align 4
        Instruction::GetElementPtr {
            result: Register::new("%3"),
            ty: Type::Struct {
                id: String::from("Fish"),
                fields: vec![Type::Int, Type::Int, Type::Int],
            },
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("0")),
            ],
        },
        Instruction::Store {
            ty: Type::Int,
            value: Register::new("1"),
            ptr: Register::new("%3"),
        },
        //  %4 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 1
        //  store Int 0, Int* %4, align 4
        Instruction::GetElementPtr {
            result: Register::new("%4"),
            ty: Type::Struct {
                id: String::from("Fish"),
                fields: vec![Type::Int, Type::Int, Type::Int],
            },
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("1")),
            ],
        },
        Instruction::Store {
            ty: Type::Int,
            value: Register::new("0"),
            ptr: Register::new("%4"),
        },
        //  %5 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, Int 0, Int 2
        //  store Int 700, Int* %5, align 4
        Instruction::GetElementPtr {
            result: Register::new("%5"),
            ty: Type::Struct {
                id: String::from("Fish"),
                fields: vec![Type::Int, Type::Int, Type::Int],
            },
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("2")),
            ],
        },
        Instruction::Store {
            ty: Type::Int,
            value: Register::new("700"),
            ptr: Register::new("%5"),
        },
        Instruction::Ret {
            ty: Type::Int,
            value: Register::new("0"),
        }, //}}
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
        structure_types: vec![
            parameter.clone(),
            storage.clone(),
            pair.clone(),
            Type::Struct {
                id: String::from("Fish"),
                fields: vec![Type::Int, Type::Int, Type::Int],
            },
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

    let file_name = "simple_struct";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    //println!("{}", michelson_code);
}
