use lltz::compiler::compile;
use lltz::lltz_ir::{
    Arg, Function, Instruction, Program, Register, Type,
};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //typedef long int Mutez; // signed 64-bit
    //typedef long long int Int;
    //typedef unsigned long long int Nat;
    //typedef char* Address;
    //#define DUMMY_AMOUNT 0
    //#define DUMMY_BALANCE 0
    //#define DUMMY_ADDRESS "KT1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx"
    //#define DUMMY_TOTAL_VOTING_POWER 0
    //struct Parameter {
    //    Mutez amount;
    //    Mutez balance;
    //    Nat total_voting_power;
    //};
    //struct Storage {
    //    Mutez amount;
    //    Mutez balance;
    //    Nat total_voting_power;
    //};
    //
    //struct Operation {};
    //
    //Mutez get_amount() {
    //    return DUMMY_AMOUNT;
    //}
    //
    //Mutez get_balance() {
    //    return DUMMY_BALANCE;
    //}
    //
    //Nat get_total_voting_power() {
    //    return DUMMY_TOTAL_VOTING_POWER;
    //}
    //
    //Nat get_level() {
    //    return DUMMY_LEVEL;
    //}
    //
    //Address get_self_address() {
    //    return DUMMY_ADDRESS;
    //}
    //
    //Address get_sender() {
    //    return DUMMY_ADDRESS;
    //}
    //
    //Address get_source() {
    //    return DUMMY_ADDRESS;
    //}
    //
    //struct Pair {
    //    struct Operation ops[0];
    //    struct Storage storage;
    //};
    //
    //struct Pair smart_contract(struct Parameter param, struct Storage storage) {
    //    struct Pair p;
    //    Address sender = get_sender();
    //    Address source = get_source();
    //    Address self_address = get_self_address();
    //    //p.storage = storage;
    //    return p;
    //};

    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    //%struct.Operation = type {}
    //%struct.Storage = type {}
    //%struct.Parameter = type {}
    //
    //@.str = private unnamed_addr constant [37 x i8] c"KT1xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx\00", align 1
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local i64 @get_amount() #0 !dbg !7 {
    //  ret i64 0, !dbg !13
    //}
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local i64 @get_balance() #0 !dbg !14 {
    //  ret i64 0, !dbg !15
    //}
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local i64 @get_total_voting_power() #0 !dbg !16 {
    //  ret i64 0, !dbg !21
    //}
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local i8* @get_self_address() #0 !dbg !22 {
    //  ret i8* getelementptr inbounds ([37 x i8], [37 x i8]* @.str, i64 0, i64 0), !dbg !28
    //}
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local i8* @get_sender() #0 !dbg !29 {
    //  ret i8* getelementptr inbounds ([37 x i8], [37 x i8]* @.str, i64 0, i64 0), !dbg !30
    //}
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local i8* @get_source() #0 !dbg !31 {
    //  ret i8* getelementptr inbounds ([37 x i8], [37 x i8]* @.str, i64 0, i64 0), !dbg !32
    //}
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local void @smart_contract(
    //        %struct.Pair* noalias sret %0,
    //        %struct.Parameter* byval(%struct.Parameter) align 8 %1,
    //        %struct.Storage* byval(%struct.Storage) align 8 %2
    //    ) #0 !dbg !33 {
    //  %4 = alloca i8*, align 8
    //  %5 = alloca i8*, align 8
    //  %6 = alloca i8*, align 8
    //  call void @llvm.dbg.declare(metadata %struct.Parameter* %1, metadata !54, metadata !DIExpression()), !dbg !55
    //  call void @llvm.dbg.declare(metadata %struct.Storage* %2, metadata !56, metadata !DIExpression()), !dbg !57
    //  call void @llvm.dbg.declare(metadata %struct.Pair* %0, metadata !58, metadata !DIExpression()), !dbg !59
    //  call void @llvm.dbg.declare(metadata i8** %4, metadata !60, metadata !DIExpression()), !dbg !61
    //  %7 = call i8* @get_sender(), !dbg !62
    //  store i8* %7, i8** %4, align 8, !dbg !61
    //  call void @llvm.dbg.declare(metadata i8** %5, metadata !63, metadata !DIExpression()), !dbg !64
    //  %8 = call i8* @get_source(), !dbg !65
    //  store i8* %8, i8** %5, align 8, !dbg !64
    //  call void @llvm.dbg.declare(metadata i8** %6, metadata !66, metadata !DIExpression()), !dbg !67
    //  %9 = call i8* @get_self_address(), !dbg !68
    //  store i8* %9, i8** %6, align 8, !dbg !67
    //  ret void, !dbg !69
    //}

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

    //}

    //define dso_local void @smart_contract(
    //      %struct.Pair* noalias sret %0,
    //      %struct.Parameter* byval(%struct.Parameter) align 8 %1,
    //      %struct.Storage* byval(%struct.Storage) align 8 %2
    //  ) #0 {
    let instructions = vec![
        //  %4 = alloca i8*, align 8
        //  %5 = alloca i8*, align 8
        //  %6 = alloca i8*, align 8
        //  %100 = alloca Nat;
        Instruction::Alloca {
            ptr: Register::new("%4"),
            ty: Type::Address,
        },
        Instruction::Alloca {
            ptr: Register::new("%5"),
            ty: Type::Address,
        },
        Instruction::Alloca {
            ptr: Register::new("%6"),
            ty: Type::Address,
        },
        Instruction::Alloca {
            ptr: Register::new("%100"),
            ty: Type::Nat,
        },
        //  %7 = call i8* @get_sender(), !dbg !62
        //  store i8* %7, i8** %4, align 8, !dbg !61
        Instruction::MichelsonGetSender {
            result: Register::new("%7"),
        },
        Instruction::Store {
            ty: Type::Address,
            value: Register::new("%7"),
            ptr: Register::new("%4"),
        },
        //  %8 = call i8* @get_source(), !dbg !65
        //  store i8* %8, i8** %5, align 8, !dbg !64
        Instruction::MichelsonGetSource {
            result: Register::new("%8"),
        },
        Instruction::Store {
            ty: Type::Address,
            value: Register::new("%8"),
            ptr: Register::new("%5"),
        },
        //  %9 = call i8* @get_self_address(), !dbg !68
        //  store i8* %9, i8** %6, align 8, !dbg !67
        Instruction::MichelsonGetSelfAddress {
            result: Register::new("%9"),
        },
        Instruction::Store {
            ty: Type::Address,
            value: Register::new("%9"),
            ptr: Register::new("%6"),
        },
        //  %11 = call i64 @get_level(), !dbg !73
        //  store i64 %11, i64* %7, align 8, !dbg !72
        Instruction::MichelsonGetSelfAddress {
            result: Register::new("%101"),
        },
        Instruction::Store {
            ty: Type::Address,
            value: Register::new("%101"),
            ptr: Register::new("%100"),
        },
        //  ret void, !dbg !69
    ];

    let lltz_ir = Program {
        structure_types: vec![storage.clone(), parameter.clone(), pair.clone()],
        functions: vec![
            //define dso_local void @smart_contract(
            //      %struct.Pair* noalias sret %0,
            //      %struct.Parameter* byval(%struct.Parameter) align 8 %1,
            //      %struct.Storage* byval(%struct.Storage) align 8 %2
            //) #0 {
            Function {
                //define dso_local void @smart_contract(
                function_name: String::from("smart_contract"),
                result_type: Type::Int,
                //      %struct.Pair* noalias sret %0,
                //      %struct.Parameter* byval(%struct.Parameter) align 8 %1,
                //      %struct.Storage* byval(%struct.Storage) align 8 %2
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
            },
        ],
    };

    let michelson_code = compile(lltz_ir);

    let file_name = "simple_blockchain_operations2";
    let explanation = "";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{explanation}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
