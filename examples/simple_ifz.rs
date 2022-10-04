use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{Instruction, Opcode, Register, Type};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    // C program
    //int main() {
    //    int a = 8;
    //    int b;
    //    if (a == 0) {
    //        b = -1;
    //    } else {
    //        int c = 3;
    //        b = a * c;
    //    }
    //    return b;
    //}

    // LLVM IR
    //define dso_local i32 @main() #0 {
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32, align 4
    //  %4 = alloca i32, align 4
    //  store i32 0, i32* %1, align 4
    //  store i32 8, i32* %2, align 4
    //  %5 = load i32, i32* %2, align 4
    //  %6 = icmp eq i32 %5, 0
    //  br i1 %6, label %7, label %8
    //
    //7:                                                ; preds = %0
    //  store i32 -1, i32* %3, align 4
    //  br label %12
    //
    //8:                                                ; preds = %0
    //  store i32 3, i32* %4, align 4
    //  %9 = load i32, i32* %2, align 4
    //  %10 = load i32, i32* %4, align 4
    //  %11 = mul nsw i32 %9, %10
    //  store i32 %11, i32* %3, align 4
    //  br label %12
    //
    //12:                                               ; preds = %8, %7
    //  %13 = load i32, i32* %3, align 4
    //  ret i32 %13
    //}

    // mini LLVM IR
    //define dso_local i32 @main() #0 {
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32, align 4
    //  %4 = alloca i32, align 4
    //  store i32 0, i32* %1, align 4
    //  store i32 8, i32* %2, align 4
    //  %5 = load i32, i32* %2, align 4
    //  %6 = icmp eq i32 %5, 0
    //
    //  ifz i1 %6, {
    //      store i32 3, i32* %4, align 4
    //      %9 = load i32, i32* %2, align 4
    //      %10 = load i32, i32* %4, align 4
    //      %11 = mul nsw i32 %9, %10
    //      store i32 %11, i32* %3, align 4
    //  }, {
    //      store i32 -1, i32* %3, align 4
    //      br label %12
    //  }
    //
    //  %13 = load i32, i32* %3, align 4
    //  ret i32 %13
    //}

    //{{
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32, align 4
    //  %4 = alloca i32, align 4
    let instr1 = Instruction::Alloca {
        reg: Register {
            id: "%1".to_string(),
        },
        ty: Type::I32,
    };

    let instr2 = Instruction::Alloca {
        reg: Register {
            id: "%2".to_string(),
        },
        ty: Type::I32,
    };
    let instr3 = Instruction::Alloca {
        reg: Register {
            id: "%3".to_string(),
        },
        ty: Type::I32,
    };
    let instr4 = Instruction::Alloca {
        reg: Register {
            id: "%4".to_string(),
        },
        ty: Type::I32,
    };

    //  store i32 0, i32* %1, align 4
    //  store i32 8, i32* %2, align 4
    let instr5 = Instruction::Store {
        src: Register {
            id: "0".to_string(),
        },
        ptr: Register {
            id: "%1".to_string(),
        },
    };

    let instr6 = Instruction::Store {
        src: Register {
            id: "8".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };

    //  %5 = load i32, i32* %2, align 4
    let instr7 = Instruction::Load {
        dst: Register {
            id: "%5".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };

    //  %6 = icmp eq i32 %5, 0
    //
    //  ifz i1 %6, {
    //      store i32 3, i32* %4, align 4
    //      %9 = load i32, i32* %2, align 4
    //      %10 = load i32, i32* %4, align 4
    //      %11 = mul nsw i32 %9, %10
    //      store i32 %11, i32* %3, align 4
    //  }, {
    //      store i32 -1, i32* %3, align 4
    //      br label %12
    //  }
    //
    //  %13 = load i32, i32* %3, align 4
    //  ret i32 %13

    let instr9 = Instruction::Load {
        dst: Register {
            id: "%6".to_string(),
        },
        ptr: Register {
            id: "%3".to_string(),
        },
    };
    let instr10 = Instruction::Op {
        ty: Type::I32,
        opcode: Opcode::Add,
        dst: Register {
            id: "%7".to_string(),
        },
        reg1: Register {
            id: "%5".to_string(),
        },
        reg2: Register {
            id: "%6".to_string(),
        },
    };
    let instr11 = Instruction::Store {
        src: Register {
            id: "%7".to_string(),
        },
        ptr: Register {
            id: "%4".to_string(),
        },
    };

    let instr12 = Instruction::Ret {
        ty: Type::I32,
        reg: Register {
            id: "0".to_string(),
        },
    };
    //}}
    /*
        let instructions = vec![ instr1, instr2, instr3, instr4, instr5, instr6, instr7, instr8, instr9, instr10, instr11, instr12, ];

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
    */
}
