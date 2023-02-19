use lltz::compiler::compile;
use lltz::lltz_ir::{Arg, Const, Function, Instruction, Opcode, Program, Register, Type, Value};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //typedef long int Mutez; // signed 64-bit
    //typedef long long int Int;
    //typedef unsigned long long int Nat;
    //#define DUMMY_AMOUNT 0
    //#define DUMMY_BALANCE 0
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
    //struct Pair {
    //    struct Operation ops[0];
    //    struct Storage storage;
    //};
    //
    //struct Pair smart_contract(struct Parameter param, struct Storage storage) {
    //    struct Pair p;
    //
    //    storage.amount += get_amount();
    //    storage.amount += param.amount;
    //
    //    storage.balance += get_balance();
    //    storage.balance += param.balance;
    //
    //    storage.total_voting_power += get_total_voting_power();
    //    storage.total_voting_power += param.total_voting_power;
    //
    //    p.storage = storage;
    //    return p;
    //};
    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    //%struct.Operation = type {}
    //%struct.Storage = type { Mutez, Mutez, Nat }
    //%struct.Parameter = type { Mutez, Mutez, Nat }
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local Mutez @get_amount() #0 {
    //  ret Mutez 0
    //}
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local Mutez @get_balance() #0 {
    //  ret Mutez 0
    //}
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local Mutez @get_total_voting_power() #0 {
    //  ret Nat 0
    //}
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local void @smart_contract(
    //      %struct.Pair* noalias sret %0,
    //      %struct.Parameter* byval(%struct.Parameter) align 8 %1,
    //      %struct.Storage* byval(%struct.Storage) align 8 %2
    //  ) #0 {
    //
    //
    //  %4 = call Mutez @get_amount()
    //  %5 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 0
    //  %6 = load Mutez, Mutez* %5, align 8
    //  %7 = add nsw Mutez %6, %4
    //  store Mutez %7, Mutez* %5, align 8
    //  %8 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 0
    //  %9 = load Mutez, Mutez* %8, align 8
    //  %10 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 0
    //  %11 = load Mutez, Mutez* %10, align 8
    //  %12 = add nsw Mutez %11, %9
    //  store Mutez %12, Mutez* %10, align 8
    //
    //
    //
    //  %13 = call Mutez @get_balance()
    //  %14 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 1
    //  %15 = load Mutez, Mutez* %14, align 8
    //  %16 = add nsw Mutez %15, %13
    //  store Mutez %16, Mutez* %14, align 8
    //  %17 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 1
    //  %18 = load Mutez, Mutez* %17, align 8
    //  %19 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 1
    //  %20 = load Mutez, Mutez* %19, align 8
    //  %21 = add nsw Mutez %20, %18
    //  store Mutez %21, Mutez* %19, align 8
    //
    //
    //
    //  %22 = call Nat @get_total_voting_power()
    //  %23 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 2
    //  %24 = load Nat, Nat* %23, align 8
    //  %25 = add Nat %24, %22
    //  store Nat %25, Nat* %23, align 8
    //  %26 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 2
    //  %27 = load Nat, Nat* %26, align 8
    //  %28 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 2
    //  %29 = load Nat, Nat* %28, align 8
    //  %30 = add Nat %29, %27
    //  store Nat %30, Nat* %28, align 8
    //
    //
    //
    //
    //  %31 = getelementptr inbounds %struct.Pair, %struct.Pair* %0, i32 0, i32 1
    //  %32 = bitcast %struct.Storage* %31 to i8*
    //  %33 = bitcast %struct.Storage* %2 to i8*
    //  call void @llvm.memcpy.p0i8.p0i8.Mutez(i8* align 8 %32, i8* align 8 %33, Mutez 24, i1 false)
    //  ret void
    //}
    //
    //

    let parameter = Type::Struct {
        id: String::from("Parameter"),
        fields: vec![Type::Mutez, Type::Mutez, Type::Nat],
    };

    let storage = Type::Struct {
        id: String::from("Storage"),
        fields: vec![Type::Mutez, Type::Mutez, Type::Nat],
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
        //  %4 = call Mutez @get_amount()
        //  %5 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 0
        //  %6 = load Mutez, Mutez* %5, align 8
        //  %7 = add nsw Mutez %6, %4
        //  store Mutez %7, Mutez* %5, align 8
        //  %8 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 0
        //  %9 = load Mutez, Mutez* %8, align 8
        //  %10 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 0
        //  %11 = load Mutez, Mutez* %10, align 8
        //  %12 = add nsw Mutez %11, %9
        //  store Mutez %12, Mutez* %10, align 8
        Instruction::MichelsonGetAmount {
            result: Register::new("%4"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%5"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(0))),
            ],
        },
        Instruction::Load {
            ty: Type::Mutez,
            result: Register::new("%6"),
            ptr: Register::new("%5"),
        },
        Instruction::Op {
            ty: Type::Mutez,
            opcode: Opcode::Add,
            result: Register::new("%7"),
            op1: Value::Register(Register::new("%6")),
            op2: Value::Register(Register::new("%4")),
        },
        Instruction::Store {
            ty: Type::Mutez,
            value: Value::Register(Register::new("%7")),
            ptr: Register::new("%5"),
        },
        //
        //
        //
        //  %13 = call Mutez @get_balance()
        //  %14 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 1
        //  %15 = load Mutez, Mutez* %14, align 8
        //  %16 = add nsw Mutez %15, %13
        //  store Mutez %16, Mutez* %14, align 8
        //  %17 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 1
        //  %18 = load Mutez, Mutez* %17, align 8
        //  %19 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 1
        //  %20 = load Mutez, Mutez* %19, align 8
        //  %21 = add nsw Mutez %20, %18
        //  store Mutez %21, Mutez* %19, align 8
        Instruction::MichelsonGetBalance {
            result: Register::new("%13"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%14"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(1))),
            ],
        },
        Instruction::Load {
            ty: Type::Mutez,
            result: Register::new("%15"),
            ptr: Register::new("%14"),
        },
        Instruction::Op {
            ty: Type::Mutez,
            opcode: Opcode::Add,
            result: Register::new("%16"),
            op1: Value::Register(Register::new("%15")),
            op2: Value::Register(Register::new("%13")),
        },
        Instruction::Store {
            ty: Type::Mutez,
            value: Value::Register(Register::new("%16")),
            ptr: Register::new("%14"),
        },
        //
        //
        //  %22 = call Nat @get_total_voting_power()
        //  %23 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 2
        //  %24 = load Nat, Nat* %23, align 8
        //  %25 = add Nat %24, %22
        //  store Nat %25, Nat* %23, align 8
        //  %26 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 2
        //  %27 = load Nat, Nat* %26, align 8
        //  %28 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 2
        //  %29 = load Nat, Nat* %28, align 8
        //  %30 = add Nat %29, %27
        //  store Nat %30, Nat* %28, align 8
        Instruction::MichelsonGetTotalVotingPower {
            result: Register::new("%22"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%23"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(2))),
            ],
        },
        Instruction::Load {
            ty: Type::Nat,
            result: Register::new("%24"),
            ptr: Register::new("%23"),
        },
        Instruction::Op {
            ty: Type::Nat,
            opcode: Opcode::Add,
            result: Register::new("%25"),
            op1: Value::Register(Register::new("%24")),
            op2: Value::Register(Register::new("%22")),
        },
        Instruction::Store {
            ty: Type::Nat,
            value: Value::Register(Register::new("%25")),
            ptr: Register::new("%23"),
        },
        //  %31 = getelementptr inbounds %struct.Pair, %struct.Pair* %0, i32 0, i32 1
        Instruction::GetElementPtr {
            result: Register::new("%31"),
            ty: pair.clone(),
            ptrval: Register::new("%0"),
            subsequent: vec![
                (Type::Int, Value::Const(Const::Int(0))),
                (Type::Int, Value::Const(Const::Int(1))),
            ],
        },
        //  %32 = bitcast %struct.Storage* %31 to i8*
        //  %33 = bitcast %struct.Storage* %2 to i8*
        //  call void @llvm.memcpy.p0i8.p0i8.Mutez(
        //      i8* align 8 %32,
        //      i8* align 8 %33,
        //      Mutez 24, i1 false)
        Instruction::LlvmMemcpy {
            dest: Register::new("%31"),
            src: Register::new("%2"),
            ty: storage.clone(),
        },
        //  ret void
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

    let file_name = "simple_blockchain_operations";
    let explanation = "";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Pair 1 2 3' and input 'Pair 0 0 0' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{explanation}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
