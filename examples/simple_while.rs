use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{
    Arg, Condition, Function, Instruction, MiniLlvm, Opcode, Register, Type,
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
    //%struct.Parameter = type {}
    //%struct.Storage   = type {}
    //%struct.Operation = type {}
    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    //
    //define dso_local void @smart_contract(
    //  %struct.Pair* noalias sret %pair,
    //  %struct.Parameter* byval(%struct.Parameter) align 8 %parameter,
    //  %struct.Storage* byval(%struct.Storage) align 8 %storage
    //) #0 {
    //  %1 = alloca Int, align 4
    //  %2 = alloca Int, align 4
    //  %3 = alloca Int, align 4
    //  store Int 0, Int* %1, align 4
    //  store Int 10, Int* %2, align 4
    //  store Int 1, Int* %3, align 4
    //  br label %4
    //
    //4:                                                ; preds = %7, %0
    //  %5 = load Int, Int* %2, align 4
    //  %6 = icmp slt Int 0, %5
    //  br i1 %6, label %7, label %13
    //
    //7:                                                ; preds = %4
    //  %8 = load Int, Int* %2, align 4
    //  %9 = load Int, Int* %3, align 4
    //  %10 = mul nsw Int %9, %8
    //  store Int %10, Int* %3, align 4
    //  %11 = load Int, Int* %2, align 4
    //  %12 = sub nsw Int %11, 1
    //  store Int %12, Int* %2, align 4
    //  br label %4
    //
    //13:                                               ; preds = %4
    //  %14 = load Int, Int* %3, align 4
    //  ret Int %14
    //}

    // mini LLVM IR
    //%struct.Parameter = type {}
    //%struct.Storage   = type {}
    //%struct.Operation = type {}
    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    //
    //define dso_local void @smart_contract(
    //  %struct.Pair* noalias sret %pair,
    //  %struct.Parameter* byval(%struct.Parameter) align 8 %parameter,
    //  %struct.Storage* byval(%struct.Storage) align 8 %storage
    //) #0 {
    //  %1 = alloca Int, align 4
    //  %2 = alloca Int, align 4
    //  %3 = alloca Int, align 4
    //  store Int 0, Int* %1, align 4
    //  store Int 10, Int* %2, align 4
    //  store Int 1, Int* %3, align 4
    //
    // while {
    //      %5 = load Int, Int* %2, align 4
    //      %6 = icmp slt Int 0, %5
    //      %6
    // } {
    //      %8 = load Int, Int* %2, align 4
    //      %9 = load Int, Int* %3, align 4
    //      %10 = mul nsw Int %9, %8
    //      store Int %10, Int* %3, align 4
    //      %11 = load Int, Int* %2, align 4
    //      %12 = sub nsw Int %11, 1
    //      store Int %12, Int* %2, align 4
    //  }
    //
    //  %14 = load Int, Int* %3, align 4
    //  ret Int %14
    //}

    //{{

    //  ret Int %14
    //}}

    let instructions = vec![
        //  %1 = alloca Int, align 4
        //  %2 = alloca Int, align 4
        //  %3 = alloca Int, align 4
        Instruction::Alloca {
            ptr: Register::new("%1"),
            ty: Type::Int,
        },
        Instruction::Alloca {
            ptr: Register::new("%2"),
            ty: Type::Int,
        },
        Instruction::Alloca {
            ptr: Register::new("%3"),
            ty: Type::Int,
        },
        //  store Int 0, Int* %1, align 4
        //  store Int 10, Int* %2, align 4
        //  store Int 1, Int* %3, align 4
        Instruction::Store {
            ty: Type::Int,
            value: Register::new("0"),
            ptr: Register::new("%1"),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Register::new("10"),
            ptr: Register::new("%2"),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Register::new("1"),
            ptr: Register::new("%3"),
        },
        Instruction::While {
            cond: Register::new("%6"),
            // while {
            //      %5 = load Int, Int* %2, align 4
            //      %6 = icmp slt Int 0, %5
            //      %6
            // }
            cond_block: vec![
                Instruction::Load {
                    ty: Type::Int,
                    result: Register::new("%5"),
                    ptr: Register::new("%2"),
                },
                Instruction::Icmp {
                    ty: Type::Bool,
                    cond: Condition::Slt,
                    result: Register::new("%6"),
                    op1: Register::new("0"),
                    op2: Register::new("%5"),
                },
            ],
            // {
            //      %8 = load Int, Int* %2, align 4
            //      %9 = load Int, Int* %3, align 4
            //      %10 = mul nsw Int %9, %8
            //      store Int %10, Int* %3, align 4
            //      %11 = load Int, Int* %2, align 4
            //      %12 = sub nsw Int %11, 1
            //      store Int %12, Int* %2, align 4
            loop_block: vec![
                Instruction::Load {
                    ty: Type::Int,
                    result: Register::new("%8"),
                    ptr: Register::new("%2"),
                },
                Instruction::Load {
                    ty: Type::Int,
                    result: Register::new("%9"),
                    ptr: Register::new("%3"),
                },
                Instruction::Op {
                    ty: Type::Int,
                    opcode: Opcode::Mul,
                    result: Register::new("%10"),
                    op1: Register::new("%9"),
                    op2: Register::new("%8"),
                },
                Instruction::Store {
                    ty: Type::Int,
                    value: Register::new("%10"),
                    ptr: Register::new("%3"),
                },
                Instruction::Load {
                    ty: Type::Int,
                    result: Register::new("%11"),
                    ptr: Register::new("%2"),
                },
                Instruction::Op {
                    ty: Type::Int,
                    opcode: Opcode::Sub,
                    result: Register::new("%12"),
                    op1: Register::new("%11"),
                    op2: Register::new("1"),
                },
                Instruction::Store {
                    ty: Type::Int,
                    value: Register::new("%12"),
                    ptr: Register::new("%2"),
                },
            ], //  }
        },
        //  %14 = load Int, Int* %3, align 4
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%14"),
            ptr: Register::new("%3"),
        },
    ];

    let parameter = Type::Struct {
        id: String::from("Parameter"),
        fields: vec![],
    };

    let storage = Type::Struct {
        id: String::from("Storage"),
        fields: vec![],
    };

    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    let pair = Type::Struct {
        id: String::from("Pair"),
        fields: vec![
            Type::Array {
                size: 0,
                elementtype: Box::new(Type::Operation),
            },
            storage.clone(),
        ],
    };

    let mini_llvm = MiniLlvm {
        structure_types: vec![parameter.clone(), storage.clone(), pair.clone()],
        functions: vec![Function {
            function_name: String::from("smart_contract"),
            result_type: Type::Int,
            argument_list: vec![
                Arg {
                    ty: Type::Ptr(Box::new(pair.clone())),
                    reg: Register::new("%pair"),
                },
                Arg {
                    ty: Type::Ptr(Box::new(parameter.clone())),
                    reg: Register::new("%parameter"),
                },
                Arg {
                    ty: Type::Ptr(Box::new(storage.clone())),
                    reg: Register::new("%storage"),
                },
            ],
            instructions,
        }],
    };

    let michelson_code = compile(mini_llvm);

    let file_name = "simple_while";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
