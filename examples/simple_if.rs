use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{Condition, Instruction, Opcode, Register, Type};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    // int main() {
    //     int a = 0;
    //     int b;
    //     if (a == 0) {
    //         b = 777;
    //     } else {
    //         b = 444;
    //     }
    //     return 0;
    // }

    // LLVM IR
    // define dso_local i32 @main() #0 {
    //   %1 = alloca i32, align 4
    //   %2 = alloca i32, align 4
    //   %3 = alloca i32, align 4
    //   store i32 0, i32* %1, align 4
    //   store i32 0, i32* %2, align 4
    //   %4 = load i32, i32* %2, align 4
    //   %5 = icmp eq i32 %4, 0
    //   br i1 %5, label %6, label %7
    //
    // 6:                                                ; preds = %0
    //   store i32 777, i32* %3, align 4
    //   br label %8
    //
    // 7:                                                ; preds = %0
    //   store i32 444, i32* %3, align 4
    //   br label %8
    //
    // 8:                                                ; preds = %7, %6
    //   ret i32 0
    // }

    // mini LLVM IR
    // define dso_local i32 @main() #0 {
    //   %1 = alloca i32, align 4
    //   %2 = alloca i32, align 4
    //   %3 = alloca i32, align 4
    //   store i32 0, i32* %1, align 4
    //   store i32 0, i32* %2, align 4
    //   %4 = load i32, i32* %2, align 4
    //   %5 = icmp eq i32 %4, 0
    //
    //   if i1 %5, {
    //      store i32 444, i32* %3, align 4
    //      br label %8
    //   } {
    //      store i32 777, i32* %3, align 4
    //      br label %8
    //   }
    //
    //   ret i32 0
    // }

    //{{
    //   %1 = alloca i32, align 4
    //   %2 = alloca i32, align 4
    //   %3 = alloca i32, align 4
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

    //   store i32 0, i32* %1, align 4
    //   store i32 0, i32* %2, align 4
    //   %4 = load i32, i32* %2, align 4
    let instr4 = Instruction::Store {
        ty: Type::I32,
        value: Register {
            id: "0".to_string(),
        },
        ptr: Register {
            id: "%1".to_string(),
        },
    };

    let instr5 = Instruction::Store {
        ty: Type::I32,
        value: Register {
            id: "0".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };

    let instr6 = Instruction::Load {
        ty: Type::I32,
        result: Register {
            id: "%4".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };

    //   %5 = icmp eq i32 %4, 0
    //
    //   if i1 %5, {
    //      store i32 777, i32* %3, align 4
    //      br label %8
    //   } {
    //      store i32 444, i32* %3, align 4
    //      br label %8
    //   }

    let instr7 = Instruction::Icmp {
        ty: Type::I32,
        cond: Condition::Eq,
        result: Register {
            id: "%5".to_string(),
        },
        op1: Register {
            id: "%4".to_string(),
        },
        op2: Register {
            id: "0".to_string(),
        },
    };

    let instr_t = Instruction::Store {
        ty: Type::I32,
        value: Register {
            id: "777".to_string(),
        },
        ptr: Register {
            id: "%3".to_string(),
        },
    };

    let instr_f = Instruction::Store {
        ty: Type::I32,
        value: Register {
            id: "444".to_string(),
        },
        ptr: Register {
            id: "%3".to_string(),
        },
    };

    let instr8 = Instruction::If {
        reg: Register {
            id: "%5".to_string(),
        },
        code_block_t: vec![instr_t],
        code_block_f: vec![instr_f],
    };

    //   ret i32 0
    //}}

    let instructions = vec![
        instr1, instr2, instr3, instr4, instr5, instr6, instr7, instr8,
    ];

    let michelson_code = compile(instructions);

    let file_name = "simple_if";
    let command_typecheck =
        format!("#tezos-client --mode mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    println!("{}", michelson_code);
}
