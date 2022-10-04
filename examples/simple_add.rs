use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{Instruction, Opcode, Register, Type};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //int main() {
    //  int a = 10;
    //  int b = 20;
    //  int c = a + b;
    //  return 0;
    //}
    //define dso_local i32 @main() #0 {
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32, align 4
    //  %4 = alloca i32, align 4
    //  store i32 0, i32* %1, align 4
    //  store i32 10, i32* %2, align 4
    //  store i32 20, i32* %3, align 4
    //  %5 = load i32, i32* %2, align 4
    //  %6 = load i32, i32* %3, align 4
    //  %7 = add nsw i32 %5, %6
    //  store i32 %7, i32* %4, align 4
    //  ret i32 0
    //}

    //{{
    let instr1 = Instruction::Alloca {
        ptr: Register {
            id: "%1".to_string(),
        },
        ty: Type::I32,
    };

    let instr2 = Instruction::Alloca {
        ptr: Register {
            id: "%2".to_string(),
        },
        ty: Type::I32,
    };
    let instr3 = Instruction::Alloca {
        ptr: Register {
            id: "%3".to_string(),
        },
        ty: Type::I32,
    };
    let instr4 = Instruction::Alloca {
        ptr: Register {
            id: "%4".to_string(),
        },
        ty: Type::I32,
    };

    let instr5 = Instruction::Store {
        ty: Type::I32,
        value: Register {
            id: "0".to_string(),
        },
        ptr: Register {
            id: "%1".to_string(),
        },
    };

    let instr6 = Instruction::Store {
        ty: Type::I32,
        value: Register {
            id: "10".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };
    let instr7 = Instruction::Store {
        ty: Type::I32,
        value: Register {
            id: "20".to_string(),
        },
        ptr: Register {
            id: "%3".to_string(),
        },
    };

    let instr8 = Instruction::Load {
        ty: Type::I32,
        result: Register {
            id: "%5".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };
    let instr9 = Instruction::Load {
        ty: Type::I32,
        result: Register {
            id: "%6".to_string(),
        },
        ptr: Register {
            id: "%3".to_string(),
        },
    };
    let instr10 = Instruction::Op {
        ty: Type::I32,
        opcode: Opcode::Add,
        result: Register {
            id: "%7".to_string(),
        },
        op1: Register {
            id: "%5".to_string(),
        },
        op2: Register {
            id: "%6".to_string(),
        },
    };
    let instr11 = Instruction::Store {
        ty: Type::I32,
        value: Register {
            id: "%7".to_string(),
        },
        ptr: Register {
            id: "%4".to_string(),
        },
    };

    let instr12 = Instruction::Ret {
        ty: Type::I32,
        value: Register {
            id: "0".to_string(),
        },
    };
    //}}

    let instructions = vec![
        instr1, instr2, instr3, instr4, instr5, instr6, instr7, instr8, instr9, instr10, instr11,
        instr12,
    ];

    let michelson_code = compile(instructions);

    let file_name = "simple_add";
    let command_typecheck =
        format!("#tezos-client --mode mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    println!("{}", michelson_code);
}
