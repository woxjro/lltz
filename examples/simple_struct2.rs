use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{Function, Instruction, MiniLlvm, Register, Type};
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
    //int main()
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
    //%struct.Fish = type { i32, i32, i32, %struct.Prefecture }
    //%struct.Prefecture = type { i32, i32 }
    //
    //define dso_local i32 @main() #0 {
    //  %1 = alloca i32, align 4
    //  %2 = alloca %struct.Fish, align 4
    //  store i32 0, i32* %1, align 4
    //  %3 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 0
    //  store i32 1, i32* %3, align 4
    //  %4 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 1
    //  store i32 30, i32* %4, align 4
    //  %5 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 2
    //  store i32 800, i32* %5, align 4
    //  %6 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 3
    //  %7 = getelementptr inbounds %struct.Prefecture, %struct.Prefecture* %6, i32 0, i32 0
    //  store i32 2, i32* %7, align 4
    //  %8 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 3
    //  %9 = getelementptr inbounds %struct.Prefecture, %struct.Prefecture* %8, i32 0, i32 1
    //  store i32 8800000, i32* %9, align 4
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
        //  store i32 0, i32* %1, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("0"),
            ptr: Register::new("%1"),
        },
        //  %3 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 0
        Instruction::GetElementPtr {
            result: Register::new("%3"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("0")),
            ],
        },
        //  store i32 1, i32* %3, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("1"),
            ptr: Register::new("%3"),
        },
        //  %4 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 1
        Instruction::GetElementPtr {
            result: Register::new("%4"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("1")),
            ],
        },
        //  store i32 30, i32* %4, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("30"),
            ptr: Register::new("%4"),
        },
        //  %5 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 2
        Instruction::GetElementPtr {
            result: Register::new("%5"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("2")),
            ],
        },
        //  store i32 800, i32* %5, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("800"),
            ptr: Register::new("%5"),
        },
        //  %6 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 3
        Instruction::GetElementPtr {
            result: Register::new("%6"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("3")),
            ],
        },
        //  %7 = getelementptr inbounds %struct.Prefecture, %struct.Prefecture* %6, i32 0, i32 0
        Instruction::GetElementPtr {
            result: Register::new("%7"),
            ty: struct_prefecture.clone(),
            ptrval: Register::new("%6"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("0")),
            ],
        },
        //  store i32 2, i32* %7, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("2"),
            ptr: Register::new("%7"),
        },
        //  %8 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 3
        Instruction::GetElementPtr {
            result: Register::new("%8"),
            ty: struct_fish.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("3")),
            ],
        },
        //  %9 = getelementptr inbounds %struct.Prefecture, %struct.Prefecture* %8, i32 0, i32 1
        Instruction::GetElementPtr {
            result: Register::new("%9"),
            ty: struct_prefecture.clone(),
            ptrval: Register::new("%8"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("1")),
            ],
        },
        //  store i32 8800000, i32* %9, align 4
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("8800000"),
            ptr: Register::new("%9"),
        },
        //  ret i32 0
        Instruction::Ret {
            ty: Type::I32,
            value: Register::new("0"),
        },
    ];

    //}}

    let mini_llvm = MiniLlvm {
        structure_types: vec![],
        functions: vec![Function {
            function_name: String::from("smart_contract"),
            result_type: Type::I32,
            argument_list: vec![],
            instructions,
        }],
    };

    let michelson_code = compile(mini_llvm);

    let file_name = "simple_struct2";
    let command_typecheck =
        format!("#tezos-client --mode mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    //println!("{}", michelson_code);
}
