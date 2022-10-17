use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{
    Function, Instruction, MiniLlvm, Opcode, Register, Type,
};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //int main() {
    //    int a = 99;
    //    int* pa = &a;
    //    int** ppa = &pa;
    //    **ppa += 33;
    //    return 0;
    //}

    //define dso_local i32 @main() #0 {
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32*, align 8
    //  %4 = alloca i32**, align 8
    //  store i32 0, i32* %1, align 4
    //  store i32 99, i32* %2, align 4
    //  store i32* %2, i32** %3, align 8
    //  store i32** %3, i32*** %4, align 8
    //  %5 = load i32**, i32*** %4, align 8
    //  %6 = load i32*, i32** %5, align 8
    //  %7 = load i32, i32* %6, align 4
    //  %8 = add nsw i32 %7, 33
    //  store i32 %8, i32* %6, align 4
    //  ret i32 0
    //}

    //
    //
    //{{
    //
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32*, align 8
    //  %4 = alloca i32**, align 8
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
        ty: Type::Ptr(Box::new(Type::I32)),
    };

    let instr4 = Instruction::Alloca {
        ptr: Register::new("%4"),
        ty: Type::Ptr(Box::new(Type::Ptr(Box::new(Type::I32)))),
    };

    //  store i32 0, i32* %1, align 4
    //  store i32 99, i32* %2, align 4
    //  store i32* %2, i32** %3, align 8
    //  store i32** %3, i32*** %4, align 8
    let instr5 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("0"),
        ptr: Register::new("%1"),
    };

    let instr6 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("99"),
        ptr: Register::new("%2"),
    };
    let instr7 = Instruction::Store {
        ty: Type::Ptr(Box::new(Type::I32)),
        value: Register::new("%2"),
        ptr: Register::new("%3"),
    };

    let instr8 = Instruction::Store {
        ty: Type::Ptr(Box::new(Type::Ptr(Box::new(Type::I32)))),
        value: Register::new("%3"),
        ptr: Register::new("%4"),
    };

    //  %5 = load i32**, i32*** %4, align 8
    //  %6 = load i32*, i32** %5, align 8
    //  %7 = load i32, i32* %6, align 4

    let instr9 = Instruction::Load {
        ty: Type::Ptr(Box::new(Type::Ptr(Box::new(Type::I32)))),
        result: Register::new("%5"),
        ptr: Register::new("%4"),
    };
    let instr10 = Instruction::Load {
        ty: Type::Ptr(Box::new(Type::I32)),
        result: Register::new("%6"),
        ptr: Register::new("%5"),
    };

    let instr11 = Instruction::Load {
        ty: Type::I32,
        result: Register::new("%7"),
        ptr: Register::new("%6"),
    };

    //  %8 = add nsw i32 %7, 33
    //  store i32 %8, i32* %6, align 4
    let instr12 = Instruction::Op {
        ty: Type::I32,
        opcode: Opcode::Add,
        result: Register::new("%8"),
        op1: Register::new("%7"),
        op2: Register::new("33"),
    };
    let instr13 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("%8"),
        ptr: Register::new("%6"),
    };

    //  ret i32 0
    let instr14 = Instruction::Ret {
        ty: Type::I32,
        value: Register::new("0"),
    };
    //}}

    let instructions = vec![
        instr1, instr2, instr3, instr4, instr5, instr6, instr7, instr8, instr9, instr10, instr11,
        instr12, instr13, instr14,
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

    let file_name = "simple_pointer";
    let command_typecheck =
        format!("#tezos-client --mode mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
