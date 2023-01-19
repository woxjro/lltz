/*
use lltz::compiler::compile;
use lltz::lltz_ir::{Instruction, Opcode, Register, Type};
use std::fs::File;
use std::io::prelude::*;
*/
fn main() {
    //int add(int a, int b) {
    //    return a + b;
    //}

    //int main()
    //{
    //    int a = 92;
    //    int b = 37;
    //    a = add(a, b);
    //    a = add(a, 1000);
    //    a = add(a, 10000);
    //    a = add(a, 100000);
    //
    //    return 0;
    //}

    //define dso_local i32 @add(i32 %0, i32 %1) #0 {
    //  %3 = alloca i32, align 4
    //  %4 = alloca i32, align 4
    //  store i32 %0, i32* %3, align 4
    //  store i32 %1, i32* %4, align 4
    //  %5 = load i32, i32* %3, align 4
    //  %6 = load i32, i32* %4, align 4
    //  %7 = add nsw i32 %5, %6
    //  ret i32 %7
    //}
    //
    //define dso_local i32 @main() #0 {
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32, align 4
    //  store i32 0, i32* %1, align 4
    //  store i32 92, i32* %2, align 4
    //  store i32 37, i32* %3, align 4
    //  %4 = load i32, i32* %2, align 4
    //  %5 = load i32, i32* %3, align 4
    //  %6 = call i32 @add(i32 %4, i32 %5)
    //  store i32 %6, i32* %2, align 4
    //  %7 = load i32, i32* %2, align 4
    //  %8 = call i32 @add(i32 %7, i32 1000)
    //  store i32 %8, i32* %2, align 4
    //  %9 = load i32, i32* %2, align 4
    //  %10 = call i32 @add(i32 %9, i32 10000)
    //  store i32 %10, i32* %2, align 4
    //  %11 = load i32, i32* %2, align 4
    //  %12 = call i32 @add(i32 %11, i32 100000)
    //  store i32 %12, i32* %2, align 4
    //  ret i32 0
    //}

    //{{
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32, align 4
    //  store i32 0, i32* %1, align 4
    //  store i32 92, i32* %2, align 4
    //  store i32 37, i32* %3, align 4
    //  %4 = load i32, i32* %2, align 4
    //  %5 = load i32, i32* %3, align 4

    /*
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

        let instr4 = Instruction::Store {
            ty: Type::I32,
            value: Register::new("0"),
            ptr: Register::new("%1"),
        };

        let instr5 = Instruction::Store {
            ty: Type::I32,
            value: Register::new("92"),
            ptr: Register::new("%2"),
        };
        let instr6 = Instruction::Store {
            ty: Type::I32,
            value: Register::new("37"),
            ptr: Register::new("%3"),
        };
        let instr7 = Instruction::Load {
            ty: Type::I32,
            result: Register::new("%4"),
            ptr: Register::new("%2"),
        };
        let instr8 = Instruction::Load {
            ty: Type::I32,
            result: Register::new("%5"),
            ptr: Register::new("%3"),
        };
    */
    //  %6 = call i32 @add(i32 %4, i32 %5)
    //  store i32 %6, i32* %2, align 4
    //  %7 = load i32, i32* %2, align 4
    //  %8 = call i32 @add(i32 %7, i32 1000)
    //  store i32 %8, i32* %2, align 4
    //  %9 = load i32, i32* %2, align 4

    //  %10 = call i32 @add(i32 %9, i32 10000)
    //  store i32 %10, i32* %2, align 4
    //  %11 = load i32, i32* %2, align 4
    //  %12 = call i32 @add(i32 %11, i32 100000)
    //  store i32 %12, i32* %2, align 4
    //  ret i32 0
    /*
        let instr12 = Instruction::Ret {
            ty: Type::I32,
            value: Register::new("0"),
        };
    */
    /*

    let instr8 = Instruction::Load {
        ty: Type::I32,
        result: Register::new("%5"),
        ptr: Register::new("%2"),
    };
    let instr9 = Instruction::Load {
        ty: Type::I32,
        result: Register::new("%6"),
        ptr: Register::new("%3"),
    };
    let instr10 = Instruction::Op {
        ty: Type::I32,
        opcode: Opcode::Add,
        result: Register::new("%7"),
        op1: Register::new("%5"),
        op2: Register::new("%6"),
    };
    let instr11 = Instruction::Store {
        ty: Type::I32,
        value: Register::new("%7"),
        ptr: Register::new("%4"),
    };

    let instr12 = Instruction::Ret {
        ty: Type::I32,
        value: Register::new("0"),
    };
    //}}

    //手順
    //1.  まず関数の引数および内部に出てくるレジスタのidを（関数の内部だけで一意）
    //    なものではなく、関数の外も含めて一意の者に置き換える
    //2. 関数内部をコンパイルしておき、関数呼び出しcallが呼ばれた際はargsで定義され
    //   ているレジスタを実際の引数のレジスタのidで置き換える
    //define dso_local i32 @add(i32 %add_0, i32 %add_1) #0 {
    //  %add_3 = alloca i32, align 4
    //  %add_4 = alloca i32, align 4
    //  store i32 %add_0, i32* %add_3, align 4
    //  store i32 %add_1, i32* %add_4, align 4
    //  %add_5 = load i32, i32* %add_3, align 4
    //  %add_6 = load i32, i32* %add_4, align 4
    //  %add_7 = add nsw i32 %add_5, %add_6
    //  ret i32 %add_7
    //}

    let instructions = vec![
        instr1, instr2, instr3, instr4, instr5, instr6, instr7, instr8, instr9, instr10, instr11,
        instr12,
    ];

    let michelson_code = compile(instructions);

    let file_name = "simple_call";
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
