use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{
    Condition, Function, Instruction, MiniLlvm, Opcode, Register, Type,
};
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

    //  store i32 0, i32* %1, align 4
    //  store i32 10, i32* %2, align 4
    //  store i32 1, i32* %3, align 4
    let instr4 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("0"),
        ptr: Register::new("%1"),
    };

    let instr5 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("10"),
        ptr: Register::new("%2"),
    };

    let instr6 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("1"),
        ptr: Register::new("%3"),
    };

    // while {
    //      %5 = load i32, i32* %2, align 4
    //      %6 = icmp slt i32 0, %5
    //      %6
    // }
    let instr7 = Instruction::Load {
        ty: Type::I32,
        result: Register::new("%5"),
        ptr: Register::new("%2"),
    };

    let instr8 = Instruction::Icmp {
        ty: Type::I1,
        cond: Condition::Slt,
        result: Register::new("%6"),
        op1: Register::new("0"),
        op2: Register::new("%5"),
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
        result: Register::new("%8"),
        ptr: Register::new("%2"),
    };

    let instr10 = Instruction::Load {
        ty: Type::I32,
        result: Register::new("%9"),
        ptr: Register::new("%3"),
    };

    let instr11 = Instruction::Op {
        ty: Type::I32,
        opcode: Opcode::Mul,
        result: Register::new("%10"),
        op1: Register::new("%9"),
        op2: Register::new("%8"),
    };

    let instr12 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("%10"),
        ptr: Register::new("%3"),
    };

    let instr13 = Instruction::Load {
        ty: Type::I32,
        result: Register::new("%11"),
        ptr: Register::new("%2"),
    };

    let instr14 = Instruction::Op {
        ty: Type::I32,
        opcode: Opcode::Sub,
        result: Register::new("%12"),
        op1: Register::new("%11"),
        op2: Register::new("1"),
    };

    let instr15 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("%12"),
        ptr: Register::new("%2"),
    };
    let loop_block = vec![instr9, instr10, instr11, instr12, instr13, instr14, instr15];
    //  }
    let instr_while = Instruction::While {
        cond: Register::new("%6"),
        cond_block,
        loop_block,
    };

    //  %14 = load i32, i32* %3, align 4
    let instr16 = Instruction::Load {
        ty: Type::I32,
        result: Register::new("%14"),
        ptr: Register::new("%3"),
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

    let file_name = "simple_while";
    let command_typecheck =
        format!("#tezos-client --mode mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
