use lltz::compiler::compile;
use lltz::lltz_ir::{Arg, Function, Instruction, Program, Register, Type, Value};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    // %struct.ParameterForTransferTokens = type {}
    // %struct.RetPair = type { [1 x i32], %struct.Storage }
    // %struct.Storage = type {}
    // %struct.Parameter = type {}
    //
    // define void @smart_contract(%struct.RetPair* noalias sret(%struct.RetPair) align 4 %0, %struct.Parameter* byval(%struct.Parameter) align 8 %1, %struct.Storage* byval(%struct.Storage) align 8 %2) #0 !dbg !49 {
    //   %4 = alloca i8*, align 8
    //   %5 = alloca i64, align 8
    //   %6 = alloca i64, align 8
    //   %7 = alloca %struct.ParameterForTransferTokens, align 1
    //   %8 = alloca i32, align 4
    //   %9 = alloca i32, align 4
    //   call void @llvm.dbg.declare(metadata %struct.Parameter* %1, metadata !79, metadata !DIExpression()), !dbg !80
    //   call void @llvm.dbg.declare(metadata %struct.Storage* %2, metadata !81, metadata !DIExpression()), !dbg !82
    //   call void @llvm.dbg.declare(metadata %struct.RetPair* %0, metadata !83, metadata !DIExpression()), !dbg !84
    //   call void @llvm.dbg.declare(metadata i8** %4, metadata !85, metadata !DIExpression()), !dbg !86
    //   %10 = call i8* @get_source(), !dbg !87
    //   store i8* %10, i8** %4, align 8, !dbg !86
    //   call void @llvm.dbg.declare(metadata i64* %5, metadata !88, metadata !DIExpression()), !dbg !89
    //   %11 = call i64 @get_amount(), !dbg !90
    //   store i64 %11, i64* %5, align 8, !dbg !89
    //   call void @llvm.dbg.declare(metadata i64* %6, metadata !91, metadata !DIExpression()), !dbg !92
    //   %12 = call i64 @get_amount(), !dbg !93
    //   store i64 %12, i64* %6, align 8, !dbg !92
    //   call void @llvm.dbg.declare(metadata %struct.ParameterForTransferTokens* %7, metadata !94, metadata !DIExpression()), !dbg !95
    //   call void @llvm.dbg.declare(metadata i32* %8, metadata !96, metadata !DIExpression()), !dbg !97
    //   %13 = bitcast %struct.ParameterForTransferTokens* %7 to i8*, !dbg !98
    //   %14 = load i8*, i8** %4, align 8, !dbg !99
    //   %15 = call i32 @get_contract(i8* %13, i8* %14), !dbg !100
    //   store i32 %15, i32* %8, align 4, !dbg !97
    //   call void @llvm.dbg.declare(metadata i32* %9, metadata !101, metadata !DIExpression()), !dbg !102
    //   %16 = load i64, i64* %5, align 8, !dbg !103
    //   %17 = load i32, i32* %8, align 4, !dbg !104
    //   %18 = call i32 @transfer_tokens(i64 %16, i32 %17), !dbg !105
    //   store i32 %18, i32* %9, align 4, !dbg !102
    //   %19 = load i32, i32* %9, align 4, !dbg !106
    //   %20 = getelementptr inbounds %struct.RetPair, %struct.RetPair* %0, i32 0, i32 0, !dbg !107
    //   %21 = getelementptr inbounds [1 x i32], [1 x i32]* %20, i64 0, i64 0, !dbg !108
    //   store i32 %19, i32* %21, align 4, !dbg !109
    //   ret void, !dbg !110
    // }

    // define void @smart_contract(%struct.RetPair* noalias sret(%struct.RetPair) align 4 %0,
    //        %struct.Parameter* byval(%struct.Parameter) align 8 %1,
    //        %struct.Storage* byval(%struct.Storage) align 8 %2) #0 !dbg !49
    //{
    //   %4 = alloca Address
    //   %6 = alloca Mutez
    //   %8 = alloca (Contract unit)
    //   %9 = alloca Operation
    //   %10 = MichelsonGetSource();
    //   store Address %10, Address* %4
    //   %12 = MichelsonGetAmount()
    //   store Mutez %12, Mutez* %6
    //   %14 = load Address, Address* %4
    //   %option = MichelsonGetContract(Unit, Address %14)
    //   %15 = MichelsonAssertSome(%option);
    //   store (Contract unit) %15, (Contract unit)* %8
    //   %16 = load Mutez, Mutez* %6
    //   %17 = load (Contract unit), (Contract unit)* %8
    //   %18 = MichelsonTransferTokens(i64 %16, i32 %17)
    //   store Operation %18, Operation* %9
    //   %19 = load Operation, Operation* %9
    //   %20 = getelementptr inbounds %struct.RetPair, %struct.RetPair* %0, i32 0, i32 0
    //   %21 = getelementptr inbounds [1 x Operation], [1 x Operation]* %20, i64 0, i64 0
    //   store Operation %19, Operation* %21
    //   ret void
    // }

    let parameter = Type::Struct {
        id: String::from("Parameter"),
        fields: vec![],
    };

    let storage = Type::Struct {
        id: String::from("Storage"),
        fields: vec![],
    };
    //%struct.Pair = type { [1 x %struct.Operation], %struct.Storage }
    let pair = Type::Struct {
        id: String::from("Pair"),
        fields: vec![
            Type::Array {
                size: 1,
                elementtype: Box::new(Type::Operation),
            },
            storage.clone(),
        ],
    };

    let instructions = vec![
        //   %4 = alloca Address
        Instruction::Alloca {
            ptr: Register::new("%4"),
            ty: Type::Address,
        },
        //   %6 = alloca Mutez
        Instruction::Alloca {
            ptr: Register::new("%6"),
            ty: Type::Mutez,
        },
        //   %8 = alloca (Contract unit)
        Instruction::Alloca {
            ptr: Register::new("%8"),
            ty: Type::Contract(Box::new(parameter.clone())),
        },
        //   %9 = alloca Operation
        Instruction::Alloca {
            ptr: Register::new("%9"),
            ty: Type::Operation,
        },
        //   %10 = MichelsonGetSource();
        Instruction::MichelsonGetSource {
            result: Register::new("%10"),
        },
        //   store Address %10, Address* %4
        Instruction::Store {
            ty: Type::Address,
            value: Value::Register(Register::new("%10")),
            ptr: Register::new("%4"),
        },
        //   %12 = MichelsonGetAmount()
        Instruction::MichelsonGetAmount {
            result: Register::new("%12"),
        },
        //   store Mutez %12, Mutez* %6
        Instruction::Store {
            ty: Type::Mutez,
            value: Value::Register(Register::new("%12")),
            ptr: Register::new("%6"),
        },
        //   %14 = load Address, Address* %4
        Instruction::Load {
            ty: Type::Address,
            result: Register::new("%14"),
            ptr: Register::new("%4"),
        },
        //   %option = MichelsonContract(Unit, Address %14)
        Instruction::MichelsonContract {
            result: Register::new("%option"),
            ty: parameter.clone(),
            address: Register::new("%14"),
        },
        //   %15 = MichelsonAssertSome(%option);
        Instruction::MichelsonAssertSome {
            result: Register::new("%15"),
            ty: Type::Option(Box::new(Type::Contract(Box::new(parameter.clone())))),
            value: Register::new("%option"),
        },
        //   store (Contract unit) %15, (Contract unit)* %8
        Instruction::Store {
            ty: Type::Contract(Box::new(parameter.clone())),
            value: Value::Register(Register::new("%15")),
            ptr: Register::new("%8"),
        },
        //   %16 = load Mutez, Mutez* %6
        Instruction::Load {
            ty: Type::Mutez,
            result: Register::new("%16"),
            ptr: Register::new("%6"),
        },
        //   %17 = load (Contract unit), (Contract unit)* %8
        Instruction::Load {
            ty: Type::Contract(Box::new(parameter.clone())),
            result: Register::new("%17"),
            ptr: Register::new("%8"),
        },
        //   %18 = MichelsonTransferTokens(i64 %16, i32 %17)
        Instruction::MichelsonTransferTokens {
            result: Register::new("%18"),
            init: Register::new("%unknown"), //FIXME, TODO とりあえずいまは動く
            tokens: Register::new("%16"),
            contract: Register::new("%17"),
        },
        //   store Operation %18, Operation* %9
        Instruction::Store {
            ty: Type::Operation,
            value: Value::Register(Register::new("%18")),
            ptr: Register::new("%9"),
        },
        //   %19 = load Operation, Operation* %9
        Instruction::Load {
            ty: Type::Operation,
            result: Register::new("%19"),
            ptr: Register::new("%9"),
        },
        //   %20 = getelementptr inbounds %struct.RetPair, %struct.RetPair* %0, i32 0, i32 0
        Instruction::GetElementPtr {
            result: Register::new("%20"),
            ty: pair.clone(),
            ptrval: Register::new("%0"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("0")),
            ],
        },
        //   %21 = getelementptr inbounds [1 x Operation], [1 x Operation]* %20, i64 0, i64 0
        Instruction::GetElementPtr {
            result: Register::new("%21"),
            ty: Type::Array {
                size: 1,
                elementtype: Box::new(Type::Operation),
            },
            ptrval: Register::new("%20"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("0")),
            ],
        },
        //   store Operation %19, Operation* %21
        Instruction::Store {
            ty: Type::Operation,
            value: Value::Register(Register::new("%19")),
            ptr: Register::new("%21"),
        },
        //   ret void
    ];

    let lltz_ir = Program {
        structure_types: vec![parameter.clone(), storage.clone(), pair.clone()],
        functions: vec![Function {
            function_name: String::from("smart_contract"),
            result_type: Type::Int,
            // define void @smart_contract(%struct.RetPair* noalias sret(%struct.RetPair) align 4 %0,
            //        %struct.Parameter* byval(%struct.Parameter) align 8 %1,
            //        %struct.Storage* byval(%struct.Storage) align 8 %2) #0 !dbg !49
            argument_list: vec![
                Arg {
                    ty: Type::Ptr(Box::new(pair.clone())),
                    reg: Register::new("%0"),
                },
                Arg {
                    ty: Type::Ptr(Box::new(parameter.clone())),
                    reg: Register::new("%1"),
                },
                Arg {
                    ty: Type::Ptr(Box::new(storage.clone())),
                    reg: Register::new("%2"),
                },
            ],
            instructions,
        }],
    };

    let michelson_code = compile(lltz_ir);

    let file_name = "boomerang";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
