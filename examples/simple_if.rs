use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{
    Condition, Function, Instruction, MiniLlvm, Register, Type,
};
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
    //   } {
    //      store i32 777, i32* %3, align 4
    //   }
    //
    //   ret i32 0
    // }

    //{{
    //   %1 = alloca i32, align 4
    //   %2 = alloca i32, align 4
    //   %3 = alloca i32, align 4
    let instr1 = Instruction::Alloca {
        ptr: Register::new("%1"),
        ty: Type::I32,
    };

    let instr2 = Instruction::Alloca {
        ptr: Register::new("%2"),
        ty: Type::I32,
    };
    let instr3 = Instruction::Alloca {
        ptr: Register::new("%3"),
        ty: Type::I32,
    };

    //   store i32 0, i32* %1, align 4
    //   store i32 0, i32* %2, align 4
    //   %4 = load i32, i32* %2, align 4
    let instr4 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("0"),
        ptr: Register::new("%1"),
    };

    let instr5 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("0"),
        ptr: Register::new("%2"),
    };

    let instr6 = Instruction::Load {
        ty: Type::I32,
        result: Register::new("%4"),
        ptr: Register::new("%2"),
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
        result: Register::new("%5"),
        op1: Register::new("%4"),
        op2: Register::new("0"),
    };

    let instr_t = Instruction::Store {
        ty: Type::I32,
        value: Register::new("777"),
        ptr: Register::new("%3"),
    };

    let instr_f = Instruction::Store {
        ty: Type::I32,
        value: Register::new("444"),
        ptr: Register::new("%3"),
    };

    let instr8 = Instruction::If {
        reg: Register::new("%5"),
        code_block_t: vec![instr_t],
        code_block_f: vec![instr_f],
    };

    //   ret i32 0
    //}}

    let instructions = vec![
        instr1, instr2, instr3, instr4, instr5, instr6, instr7, instr8,
    ];

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
