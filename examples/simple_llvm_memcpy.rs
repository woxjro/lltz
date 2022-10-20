use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{
    Arg, Function, Instruction, MiniLlvm, Register, Type,
};
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
    //    struct Fish fish;
    //    struct Fish fish2;
    //    fish.kind = Ika;
    //    fish.size = 30;
    //    fish.weight = 800;
    //    fish.p.kind = Osaka;
    //    fish.p.population = 8800000;
    //    fish2 = fish;
    //    return 0;
    //}
    //

    //%struct.Parameter = type {}
    //%struct.Storage   = type {}
    //%struct.Operation = type {}
    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    //%struct.Fish = type { i32, i32, i32, %struct.Prefecture }
    //%struct.Prefecture = type { i32, i32 }
    //
    //define dso_local void @smart_contract(
    //  %struct.Pair* noalias sret %pair,
    //  %struct.Parameter* byval(%struct.Parameter) align 8 %parameter,
    //  %struct.Storage* byval(%struct.Storage) align 8 %storage
    //) #0 {
    //  %1 = alloca i32, align 4
    //  %2 = alloca %struct.Fish, align 4
    //  %3 = alloca %struct.Fish, align 4
    //  store i32 0, i32* %1, align 4
    //  %4 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 0
    //  store i32 1, i32* %4, align 4
    //  %5 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 1
    //  store i32 30, i32* %5, align 4
    //  %6 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 2
    //  store i32 800, i32* %6, align 4
    //  %7 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 3
    //  %8 = getelementptr inbounds %struct.Prefecture, %struct.Prefecture* %7, i32 0, i32 0
    //  store i32 2, i32* %8, align 4
    //  %9 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 3
    //  %10 = getelementptr inbounds %struct.Prefecture, %struct.Prefecture* %9, i32 0, i32 1
    //  store i32 8800000, i32* %10, align 4
    //  %11 = bitcast %struct.Fish* %3 to i8*
    //  %12 = bitcast %struct.Fish* %2 to i8*
    //  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %11, i8* align 4 %12, i64 20, i1 false)
    //  FIXME: return void
    //  ret i32 0
    //}

    //{{
    //%struct.Prefecture = type { i32, i32 }
    let struct_prefecture = Type::Struct {
        id: String::from("Prefecture"),
        fields: vec![Type::I32, Type::I32],
    };

    //%struct.Fish = type { i32, i32, i32, %struct.Prefecture }
    let struct_fish = Type::Struct {
        id: String::from("Fish"),
        fields: vec![Type::I32, Type::I32, Type::I32, struct_prefecture.clone()],
    };

    let instructions = vec![
        //  %1 = alloca i32, align 4
        Instruction::Alloca {
            ptr: Register::new("%1"),
            ty: Type::I32,
        },
        //  %2 = alloca %struct.Fish, align 4
        Instruction::Alloca {
            ptr: Register::new("%2"),
            ty: struct_fish.clone(),
        },
        //  %3 = alloca %struct.Fish, align 4
        Instruction::Alloca {
            ptr: Register::new("%3"),
            ty: struct_fish.clone(),
        },
        //  store i32 0, i32* %1, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("0"),
            ptr: Register::new("%1"),
        },
        //  %4 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 0
        Instruction::GetElementPtr {
            result: Register::new("%4"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("0")),
            ],
        },
        //  store i32 1, i32* %4, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("1"),
            ptr: Register::new("%4"),
        },
        //  %5 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 1
        Instruction::GetElementPtr {
            result: Register::new("%5"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("1")),
            ],
        },
        //  store i32 30, i32* %5, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("30"),
            ptr: Register::new("%5"),
        },
        //  %6 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 2
        Instruction::GetElementPtr {
            result: Register::new("%6"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("2")),
            ],
        },
        //  store i32 800, i32* %6, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("800"),
            ptr: Register::new("%6"),
        },
        //  %7 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 3
        Instruction::GetElementPtr {
            result: Register::new("%7"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("3")),
            ],
        },
        //  %8 = getelementptr inbounds %struct.Prefecture, %struct.Prefecture* %7, i32 0, i32 0
        Instruction::GetElementPtr {
            result: Register::new("%8"),
            ty: struct_prefecture.clone(),
            ptrval: Register::new("%7"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("0")),
            ],
        },
        //  store i32 2, i32* %8, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("2"),
            ptr: Register::new("%8"),
        },
        //  %9 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 3
        Instruction::GetElementPtr {
            result: Register::new("%9"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("3")),
            ],
        },
        //  %10 = getelementptr inbounds %struct.Prefecture, %struct.Prefecture* %9, i32 0, i32 1
        Instruction::GetElementPtr {
            result: Register::new("%10"),
            ty: struct_prefecture.clone(),
            ptrval: Register::new("%9"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("1")),
            ],
        },
        //  store i32 8800000, i32* %10, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("8800000"),
            ptr: Register::new("%10"),
        },
        ////%11 = bitcast %struct.Fish* %3 to i8*
        ////%12 = bitcast %struct.Fish* %2 to i8*
        ////call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %11, i8* align 4 %12, i64 20, i1 false)
        //  @llvm.memcpy(%struct.Fish* %3, %struct.Fish* %2)
        Instruction::LlvmMemcpy {
            dest: Register::new("%3"),
            src: Register::new("%2"),
            ty: struct_fish.clone(),
        },
        //  ret i32 0
        Instruction::Ret {
            ty: Type::I32,
            value: Register::new("0"),
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
            storage.clone(),
            parameter.clone(),
            operation.clone(),
            pair.clone(),
            struct_fish.clone(),
            struct_prefecture.clone(),
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

    let file_name = "simple_llvm_memcpy";
    let command_typecheck =
        format!("#tezos-client --mode mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    //println!("{}", michelson_code);
}
