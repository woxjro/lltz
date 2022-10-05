use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{Condition, Instruction, Opcode, Register, Type};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //int main() {
    //    int n = 10;
    //    int fact = 1;
    //    while (0 < n) {
    //        fact *= n;
    //        n = n - 1;
    //    }
    //    return fact; //3628800
    //}

    // LLVM IR
    //define dso_local i32 @main() #0 {
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32, align 4
    //  store i32 0, i32* %1, align 4
    //  store i32 10, i32* %2, align 4
    //  store i32 1, i32* %3, align 4
    //  br label %4
    //
    //4:                                                ; preds = %7, %0
    //  %5 = load i32, i32* %2, align 4
    //  %6 = icmp slt i32 0, %5
    //  br i1 %6, label %7, label %13
    //
    //7:                                                ; preds = %4
    //  %8 = load i32, i32* %2, align 4
    //  %9 = load i32, i32* %3, align 4
    //  %10 = mul nsw i32 %9, %8
    //  store i32 %10, i32* %3, align 4
    //  %11 = load i32, i32* %2, align 4
    //  %12 = sub nsw i32 %11, 1
    //  store i32 %12, i32* %2, align 4
    //  br label %4
    //
    //13:                                               ; preds = %4
    //  %14 = load i32, i32* %3, align 4
    //  ret i32 %14
    //}

    // mini LLVM IR
    //define dso_local i32 @main() #0 {
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32, align 4
    //  store i32 0, i32* %1, align 4
    //  store i32 10, i32* %2, align 4
    //  store i32 1, i32* %3, align 4
    //
    // while {
    //      %5 = load i32, i32* %2, align 4
    //      %6 = icmp slt i32 0, %5
    //      %6
    // } {
    //      %8 = load i32, i32* %2, align 4
    //      %9 = load i32, i32* %3, align 4
    //      %10 = mul nsw i32 %9, %8
    //      store i32 %10, i32* %3, align 4
    //      %11 = load i32, i32* %2, align 4
    //      %12 = sub nsw i32 %11, 1
    //      store i32 %12, i32* %2, align 4
    //  }
    //
    //  %14 = load i32, i32* %3, align 4
    //  ret i32 %14
    //}

    //{{
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32, align 4
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

    //  store i32 0, i32* %1, align 4
    //  store i32 10, i32* %2, align 4
    //  store i32 1, i32* %3, align 4
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
            id: "10".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };

    let instr6 = Instruction::Store {
        ty: Type::I32,
        value: Register {
            id: "1".to_string(),
        },
        ptr: Register {
            id: "%3".to_string(),
        },
    };

    // while {
    //      %5 = load i32, i32* %2, align 4
    //      %6 = icmp slt i32 0, %5
    //      %6
    // }
    let instr7 = Instruction::Load {
        ty: Type::I32,
        result: Register {
            id: "%5".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };

    let instr8 = Instruction::Icmp {
        ty: Type::I1,
        cond: Condition::Slt,
        result: Register {
            id: "%6".to_string(),
        },
        op1: Register {
            id: "0".to_string(),
        },
        op2: Register {
            id: "%5".to_string(),
        },
    };

    let cond_block = vec![instr7, instr8];
    // {
    //      %8 = load i32, i32* %2, align 4
    //      %9 = load i32, i32* %3, align 4
    //      %10 = mul nsw i32 %9, %8
    //      store i32 %10, i32* %3, align 4
    //      %11 = load i32, i32* %2, align 4
    //      %12 = sub nsw i32 %11, 1
    //      store i32 %12, i32* %2, align 4
    let instr9 = Instruction::Load {
        ty: Type::I32,
        result: Register {
            id: "%8".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };

    let instr10 = Instruction::Load {
        ty: Type::I32,
        result: Register {
            id: "%9".to_string(),
        },
        ptr: Register {
            id: "%3".to_string(),
        },
    };

    let instr11 = Instruction::Op {
        ty: Type::I32,
        opcode: Opcode::Mul,
        result: Register {
            id: "%10".to_string(),
        },
        op1: Register {
            id: "%9".to_string(),
        },
        op2: Register {
            id: "%8".to_string(),
        },
    };

    let instr12 = Instruction::Store {
        ty: Type::I32,
        value: Register {
            id: "%10".to_string(),
        },
        ptr: Register {
            id: "%3".to_string(),
        },
    };

    let instr13 = Instruction::Load {
        ty: Type::I32,
        result: Register {
            id: "%11".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };

    let instr14 = Instruction::Op {
        ty: Type::I32,
        opcode: Opcode::Sub,
        result: Register {
            id: "%12".to_string(),
        },
        op1: Register {
            id: "%11".to_string(),
        },
        op2: Register {
            id: "1".to_string(),
        },
    };

    let instr15 = Instruction::Store {
        ty: Type::I32,
        value: Register {
            id: "%12".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };
    let loop_block = vec![instr9, instr10, instr11, instr12, instr13, instr14, instr15];
    //  }
    let instr_while = Instruction::While {
        cond: Register {
            id: "%6".to_string(),
        },
        cond_block,
        loop_block,
    };

    //  %14 = load i32, i32* %3, align 4
    let instr16 = Instruction::Load {
        ty: Type::I32,
        result: Register {
            id: "%14".to_string(),
        },
        ptr: Register {
            id: "%3".to_string(),
        },
    };

    //  ret i32 %14
    //}}

    let instructions = vec![
        instr1,
        instr2,
        instr3,
        instr4,
        instr5,
        instr6,
        instr_while,
        instr16,
    ];

    let michelson_code = compile(instructions);

    let file_name = "simple_while";
    let command_typecheck =
        format!("#tezos-client --mode mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();

    println!("{}", michelson_code);
}
