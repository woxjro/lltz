use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{Instruction, Register, Type};
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

    //%struct.Fish = type { i32, i32, i32 }
    //
    //define dso_local i32 @main() #0 {
    //  %1 = alloca i32, align 4
    //  %2 = alloca %struct.Fish, align 4
    //  store i32 0, i32* %1, align 4
    //  %3 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 0
    //  store i32 1, i32* %3, align 4
    //  %4 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 1
    //  store i32 0, i32* %4, align 4
    //  %5 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 2
    //  store i32 700, i32* %5, align 4
    //  ret i32 0
    //}

    //{{
    //  %1 = alloca i32, align 4
    //  %2 = alloca %struct.Fish, align 4
    //  store i32 0, i32* %1, align 4
    let instr1 = Instruction::Alloca {
        ptr: Register::new("%1"),
        ty: Type::I32,
    };

    let instr2 = Instruction::Alloca {
        ptr: Register::new("%2"),
        ty: Type::Struct {
            id: String::from("Fish"),
            fields: vec![Type::I32, Type::I32, Type::I32],
        },
    };

    let instr3 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("0"),
        ptr: Register::new("%1"),
    };

    //  %3 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 0
    //  store i32 1, i32* %3, align 4
    let instr4 = Instruction::GetElementPtr {
        result: Register::new("%3"),
        ty: Type::Struct {
            id: String::from("Fish"),
            fields: vec![Type::I32, Type::I32, Type::I32],
        },
        ptrval: Register::new("%2"),
        subsequent: vec![
            (Type::I32, Register::new("0")),
            (Type::I32, Register::new("0")),
        ],
    };

    let instr5 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("1"),
        ptr: Register::new("%3"),
    };

    //  %4 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 1
    //  store i32 0, i32* %4, align 4
    let instr6 = Instruction::GetElementPtr {
        result: Register::new("%4"),
        ty: Type::Struct {
            id: String::from("Fish"),
            fields: vec![Type::I32, Type::I32, Type::I32],
        },
        ptrval: Register::new("%2"),
        subsequent: vec![
            (Type::I32, Register::new("0")),
            (Type::I32, Register::new("1")),
        ],
    };
    let instr7 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("0"),
        ptr: Register::new("%4"),
    };
    //  %5 = getelementptr inbounds %struct.Fish, %struct.Fish* %2, i32 0, i32 2
    //  store i32 700, i32* %5, align 4
    let instr8 = Instruction::GetElementPtr {
        result: Register::new("%5"),
        ty: Type::Struct {
            id: String::from("Fish"),
            fields: vec![Type::I32, Type::I32, Type::I32],
        },
        ptrval: Register::new("%2"),
        subsequent: vec![
            (Type::I32, Register::new("0")),
            (Type::I32, Register::new("2")),
        ],
    };

    let instr9 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("700"),
        ptr: Register::new("%5"),
    };

    let instr10 = Instruction::Ret {
        ty: Type::I32,
        value: Register::new("0"),
    };
    //}}

    let instructions = vec![
        instr1, instr2, instr3, instr4, instr5, instr6, instr7, instr8, instr9, instr10,
    ];

    let michelson_code = compile(instructions);

    let file_name = "simple_struct";
    let command_typecheck =
        format!("#tezos-client --mode mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    //println!("{}", michelson_code);
}
