use lltz::compiler::compile;
use lltz::lltz_ir::{Arg, Function, Instruction, Opcode, Program, Register, Type, Value};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //enum FKind {
    //    Ayu,
    //    Ika,
    //    Kisu
    //};
    //
    //enum Place {
    //    Fukui,
    //    Kyoto,
    //    Osaka,
    //    Shiga,
    //};
    //
    //struct Fish {
    //    enum FKind kind;
    //    enum Place place;
    //    int weight;
    //};
    //%struct.Parameter = type { Int, Int, Int, %struct.Fish }
    //%struct.Fish = type { Int, Int, Int }
    //%struct.Storage = type { Int, Int, Int, Int, %struct.Fish }
    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    //%struct.Operation = type {}
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local void @initial_value() #0 {
    //  %1 = alloca %struct.Parameter, align 4
    //  %2 = alloca %struct.Storage, align 4
    //  %3 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 0
    //  store Int 1, Int* %3, align 4
    //  %4 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 1
    //  store Int 5, Int* %4, align 4
    //  %5 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 2
    //  store Int 0, Int* %5, align 4
    //  %6 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 3
    //  %7 = getelementptr inbounds %struct.Fish, %struct.Fish* %6, Int 0, Int 0
    //  store Int 1, Int* %7, align 4
    //  %8 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 3
    //  %9 = getelementptr inbounds %struct.Fish, %struct.Fish* %8, Int 0, Int 1
    //  store Int 2, Int* %9, align 4
    //  %10 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 3
    //  %11 = getelementptr inbounds %struct.Fish, %struct.Fish* %10, Int 0, Int 2
    //  store Int 300, Int* %11, align 4
    //  %12 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 0
    //  store Int 0, Int* %12, align 4
    //  %13 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 1
    //  store Int 0, Int* %13, align 4
    //  %14 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 2
    //  store Int 0, Int* %14, align 4
    //  %15 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 3
    //  store Int 0, Int* %15, align 4
    //  %16 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 4
    //  %17 = getelementptr inbounds %struct.Fish, %struct.Fish* %16, Int 0, Int 0
    //  store Int 2, Int* %17, align 4
    //  %18 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 4
    //  %19 = getelementptr inbounds %struct.Fish, %struct.Fish* %18, Int 0, Int 1
    //  store Int 0, Int* %19, align 4
    //  %20 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 4
    //  %21 = getelementptr inbounds %struct.Fish, %struct.Fish* %20, Int 0, Int 2
    //  store Int 100, Int* %21, align 4
    //  ret void
    //}
    //
    //; Function Attrs: argmemonly nounwind willreturn
    //declare void @llvm.memcpy.p0i8.p0i8.i64(
    //  i8* noalias nocapture writeonly,
    //  i8* noalias nocapture readonly,
    //  i64, i1 immarg) #1
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local void @smart_contract(
    //  %struct.Pair* noalias sret %0,
    //  %struct.Parameter* byval(%struct.Parameter) align 8 %1,
    //  %struct.Storage* byval(%struct.Storage) align 8 %2
    //) #0 {
    //  %4 = alloca Int, align 4
    //  %5 = alloca Int, align 4
    //  %6 = alloca Int, align 4
    //  %7 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 2
    //  %8 = load Int, Int* %7, align 8
    //  store Int %8, Int* %4, align 4
    //  %9 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 0
    //  %10 = load Int, Int* %9, align 8
    //  store Int %10, Int* %5, align 4
    //  %11 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 1
    //  %12 = load Int, Int* %11, align 4
    //  store Int %12, Int* %6, align 4
    //  %13 = load Int, Int* %4, align 4
    //  %14 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 3
    //  %15 = load Int, Int* %14, align 4
    //  %16 = add nsw Int %15, %13
    //  store Int %16, Int* %14, align 4
    //  %17 = load Int, Int* %6, align 4
    //  %18 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 2
    //  %19 = load Int, Int* %18, align 8
    //  %20 = add nsw Int %19, %17
    //  store Int %20, Int* %18, align 8
    //  %21 = load Int, Int* %5, align 4
    //  %22 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 1
    //  %23 = load Int, Int* %22, align 4
    //  %24 = add nsw Int %23, %21
    //  store Int %24, Int* %22, align 4
    //  %25 = load Int, Int* %4, align 4
    //  %26 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 0
    //  %27 = load Int, Int* %26, align 8
    //  %28 = add nsw Int %27, %25
    //  store Int %28, Int* %26, align 8
    //  %29 = load Int, Int* %6, align 4
    //  %30 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 0
    //  %31 = load Int, Int* %30, align 8
    //  %32 = add nsw Int %31, %29
    //  store Int %32, Int* %30, align 8
    //  %33 = load Int, Int* %5, align 4
    //  %34 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 0
    //  %35 = load Int, Int* %34, align 8
    //  %36 = add nsw Int %35, %33
    //  store Int %36, Int* %34, align 8
    //  %37 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 4
    //  %38 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 3
    //  %39 = bitcast %struct.Fish* %37 to i8*
    //  %40 = bitcast %struct.Fish* %38 to i8*
    //  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %39, i8* align 4 %40, i64 12, i1 false)
    //  %41 = getelementptr inbounds %struct.Pair, %struct.Pair* %0, Int 0, Int 1
    //  %42 = bitcast %struct.Storage* %41 to i8*
    //  %43 = bitcast %struct.Storage* %2 to i8*
    //  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %42, i8* align 8 %43, i64 28, i1 false)
    //  ret void
    //}

    //%struct.Fish = type { Int, Int, Int }
    let fish = Type::Struct {
        id: String::from("Fish"),
        fields: vec![Type::Int, Type::Int, Type::Int],
    };

    //%struct.Parameter = type { Int, Int, Int, %struct.Fish }
    let parameter = Type::Struct {
        id: String::from("Parameter"),
        fields: vec![Type::Int, Type::Int, Type::Int, fish.clone()],
    };

    //%struct.Storage = type { Int, Int, Int, Int, %struct.Fish }
    let storage = Type::Struct {
        id: String::from("Storage"),
        fields: vec![Type::Int, Type::Int, Type::Int, Type::Int, fish.clone()],
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

    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local void @initial_value() #0 {
    let _initial_value: Vec<Instruction> = vec![
        //  %1 = alloca %struct.Parameter, align 4
        //  %2 = alloca %struct.Storage, align 4
        //  %3 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 0
        //  store Int 1, Int* %3, align 4
        //  %4 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 1
        //  store Int 5, Int* %4, align 4
        //  %5 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 2
        //  store Int 0, Int* %5, align 4
        //  %6 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 3
        //  %7 = getelementptr inbounds %struct.Fish, %struct.Fish* %6, Int 0, Int 0
        //  store Int 1, Int* %7, align 4
        //  %8 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 3
        //  %9 = getelementptr inbounds %struct.Fish, %struct.Fish* %8, Int 0, Int 1
        //  store Int 2, Int* %9, align 4
        //  %10 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 3
        //  %11 = getelementptr inbounds %struct.Fish, %struct.Fish* %10, Int 0, Int 2
        //  store Int 300, Int* %11, align 4
        //  %12 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 0
        //  store Int 0, Int* %12, align 4
        //  %13 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 1
        //  store Int 0, Int* %13, align 4
        //  %14 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 2
        //  store Int 0, Int* %14, align 4
        //  %15 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 3
        //  store Int 0, Int* %15, align 4
        //  %16 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 4
        //  %17 = getelementptr inbounds %struct.Fish, %struct.Fish* %16, Int 0, Int 0
        //  store Int 2, Int* %17, align 4
        //  %18 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 4
        //  %19 = getelementptr inbounds %struct.Fish, %struct.Fish* %18, Int 0, Int 1
        //  store Int 0, Int* %19, align 4
        //  %20 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 4
        //  %21 = getelementptr inbounds %struct.Fish, %struct.Fish* %20, Int 0, Int 2
        //  store Int 100, Int* %21, align 4
        //  ret void
    ];
    //}

    //define dso_local void @smart_contract(
    //      %struct.Pair* noalias sret %0,
    //      %struct.Parameter* byval(%struct.Parameter) align 8 %1,
    //      %struct.Storage* byval(%struct.Storage) align 8 %2
    //  ) #0 {
    let instructions = vec![
        //  %4 = alloca Int, align 4
        //  %5 = alloca Int, align 4
        //  %6 = alloca Int, align 4
        Instruction::Alloca {
            ptr: Register::new("%4"),
            ty: Type::Int,
        },
        Instruction::Alloca {
            ptr: Register::new("%5"),
            ty: Type::Int,
        },
        Instruction::Alloca {
            ptr: Register::new("%6"),
            ty: Type::Int,
        },
        //  %7 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 2
        //  %8 = load Int, Int* %7, align 8
        //  store Int %8, Int* %4, align 4
        Instruction::GetElementPtr {
            result: Register::new("%7"),
            ty: parameter.clone(),
            ptrval: Register::new("%1"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("2")),
            ],
        },
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%8"),
            ptr: Register::new("%7"),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Register(Register::new("%8")),
            ptr: Register::new("%4"),
        },
        //  %9 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 0
        //  %10 = load Int, Int* %9, align 8
        //  store Int %10, Int* %5, align 4
        Instruction::GetElementPtr {
            result: Register::new("%9"),
            ty: parameter.clone(),
            ptrval: Register::new("%1"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("0")),
            ],
        },
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%10"),
            ptr: Register::new("%9"),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Register(Register::new("%10")),
            ptr: Register::new("%5"),
        },
        //  %11 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 1
        //  %12 = load Int, Int* %11, align 4
        //  store Int %12, Int* %6, align 4
        Instruction::GetElementPtr {
            result: Register::new("%11"),
            ty: parameter.clone(),
            ptrval: Register::new("%1"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("1")),
            ],
        },
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%12"),
            ptr: Register::new("%11"),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Register(Register::new("%12")),
            ptr: Register::new("%6"),
        },
        //  %13 = load Int, Int* %4, align 4
        //  %14 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 3
        //  %15 = load Int, Int* %14, align 4
        //  %16 = add nsw Int %15, %13
        //  store Int %16, Int* %14, align 4
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%13"),
            ptr: Register::new("%4"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%14"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("3")),
            ],
        },
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%15"),
            ptr: Register::new("%14"),
        },
        Instruction::Op {
            ty: Type::Int,
            opcode: Opcode::Add,
            result: Register::new("%16"),
            op1: Value::Register(Register::new("%15")),
            op2: Value::Register(Register::new("%13")),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Register(Register::new("%16")),
            ptr: Register::new("%14"),
        },
        //  %17 = load Int, Int* %6, align 4
        //  %18 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 2
        //  %19 = load Int, Int* %18, align 8
        //  %20 = add nsw Int %19, %17
        //  store Int %20, Int* %18, align 8
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%17"),
            ptr: Register::new("%6"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%18"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("2")),
            ],
        },
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%19"),
            ptr: Register::new("%18"),
        },
        Instruction::Op {
            ty: Type::Int,
            opcode: Opcode::Add,
            result: Register::new("%20"),
            op1: Value::Register(Register::new("%19")),
            op2: Value::Register(Register::new("%17")),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Register(Register::new("%20")),
            ptr: Register::new("%18"),
        },
        //  %21 = load Int, Int* %5, align 4
        //  %22 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 1
        //  %23 = load Int, Int* %22, align 4
        //  %24 = add nsw Int %23, %21
        //  store Int %24, Int* %22, align 4
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%21"),
            ptr: Register::new("%5"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%22"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("1")),
            ],
        },
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%23"),
            ptr: Register::new("%22"),
        },
        Instruction::Op {
            ty: Type::Int,
            opcode: Opcode::Add,
            result: Register::new("%24"),
            op1: Value::Register(Register::new("%23")),
            op2: Value::Register(Register::new("%21")),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Register(Register::new("%24")),
            ptr: Register::new("%22"),
        },
        //  %25 = load Int, Int* %4, align 4
        //  %26 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 0
        //  %27 = load Int, Int* %26, align 8
        //  %28 = add nsw Int %27, %25
        //  store Int %28, Int* %26, align 8
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%25"),
            ptr: Register::new("%4"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%26"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("0")),
            ],
        },
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%27"),
            ptr: Register::new("%26"),
        },
        Instruction::Op {
            ty: Type::Int,
            opcode: Opcode::Add,
            result: Register::new("%28"),
            op1: Value::Register(Register::new("%27")),
            op2: Value::Register(Register::new("%25")),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Register(Register::new("%28")),
            ptr: Register::new("%26"),
        },
        //  %29 = load Int, Int* %6, align 4
        //  %30 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 0
        //  %31 = load Int, Int* %30, align 8
        //  %32 = add nsw Int %31, %29
        //  store Int %32, Int* %30, align 8
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%29"),
            ptr: Register::new("%6"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%30"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("0")),
            ],
        },
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%31"),
            ptr: Register::new("%30"),
        },
        Instruction::Op {
            ty: Type::Int,
            opcode: Opcode::Add,
            result: Register::new("%32"),
            op1: Value::Register(Register::new("%31")),
            op2: Value::Register(Register::new("%29")),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Register(Register::new("%32")),
            ptr: Register::new("%30"),
        },
        //  %33 = load Int, Int* %5, align 4
        //  %34 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 0
        //  %35 = load Int, Int* %34, align 8
        //  %36 = add nsw Int %35, %33
        //  store Int %36, Int* %34, align 8
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%33"),
            ptr: Register::new("%5"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%34"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("0")),
            ],
        },
        Instruction::Load {
            ty: Type::Int,
            result: Register::new("%35"),
            ptr: Register::new("%34"),
        },
        Instruction::Op {
            ty: Type::Int,
            opcode: Opcode::Add,
            result: Register::new("%36"),
            op1: Value::Register(Register::new("%35")),
            op2: Value::Register(Register::new("%33")),
        },
        Instruction::Store {
            ty: Type::Int,
            value: Value::Register(Register::new("%36")),
            ptr: Register::new("%34"),
        },
        //  %37 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, Int 0, Int 4
        //  %38 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, Int 0, Int 3
        Instruction::GetElementPtr {
            result: Register::new("%37"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("4")),
            ],
        },
        Instruction::GetElementPtr {
            result: Register::new("%38"),
            ty: parameter.clone(),
            ptrval: Register::new("%1"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("3")),
            ],
        },
        //  %39 = bitcast %struct.Fish* %37 to i8*
        //  %40 = bitcast %struct.Fish* %38 to i8*
        //  call void @llvm.memcpy.p0i8.p0i8.i64(
        //      i8* align 8 %39,
        //      i8* align 4 %40,
        //      i64 12, i1 false)
        Instruction::LlvmMemcpy {
            dest: Register::new("%37"),
            src: Register::new("%38"),
            ty: fish.clone(),
        },
        //  %41 = getelementptr inbounds %struct.Pair, %struct.Pair* %0, Int 0, Int 1
        Instruction::GetElementPtr {
            result: Register::new("%41"),
            ty: pair.clone(),
            ptrval: Register::new("%0"),
            subsequent: vec![
                (Type::Int, Register::new("0")),
                (Type::Int, Register::new("1")),
            ],
        },
        //  %42 = bitcast %struct.Storage* %41 to i8*
        //  %43 = bitcast %struct.Storage* %2 to i8*
        //  call void @llvm.memcpy.p0i8.p0i8.i64(
        //      i8* align 4 %42,
        //      i8* align 8 %43,
        //      i64 28, i1 false)
        Instruction::LlvmMemcpy {
            dest: Register::new("%41"),
            src: Register::new("%2"),
            ty: storage.clone(),
        },
        //  ret void
    ];

    let lltz_ir = Program {
        structure_types: vec![
            fish.clone(),
            storage.clone(),
            parameter.clone(),
            pair.clone(),
        ],
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

    let file_name = "complex_smartcontract";
    let explanation = "#struct Storage s = {
#    .total = 18,
#    .ayu_count = 10,
#    .ika_count = 5,
#    .kisu_count = 3,
#    .favorite = {
#        .kind = Kisu, //2
#        .place = Fukui, //0
#        .weight = 300,
#    }
#}
#struct Parameter p = {
#    .ayu_count = 3,
#    .ika_count = 6,
#    .kisu_count = 9,
#    .favorite = {
#        .kind = Ika, //1
#        .place = Osaka, //2
#        .weight = 400,
#    }
#};\n";
    let command_typecheck =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/{file_name}.tz on storage 'Pair 18 10 5 3 (Pair 2 0 300)' and input 'Pair 3 6 9 (Pair 1 2 400)' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{explanation}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
