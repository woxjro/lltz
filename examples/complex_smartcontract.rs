use mini_llvm_michelson_compiler::compiler::compile;
use mini_llvm_michelson_compiler::mini_llvm::{
    Arg, Function, Instruction, MiniLlvm, Opcode, Register, Type,
};
use std::fs::File;
use std::io::prelude::*;
fn main() {
    //%struct.Parameter = type { i32, i32, i32, %struct.Fish }
    //%struct.Fish = type { i32, i32, i32 }
    //%struct.Storage = type { i32, i32, i32, i32, %struct.Fish }
    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    //%struct.Operation = type {}
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local void @initial_value() #0 {
    //  %1 = alloca %struct.Parameter, align 4
    //  %2 = alloca %struct.Storage, align 4
    //  %3 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 0
    //  store i32 1, i32* %3, align 4
    //  %4 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 1
    //  store i32 5, i32* %4, align 4
    //  %5 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 2
    //  store i32 0, i32* %5, align 4
    //  %6 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 3
    //  %7 = getelementptr inbounds %struct.Fish, %struct.Fish* %6, i32 0, i32 0
    //  store i32 1, i32* %7, align 4
    //  %8 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 3
    //  %9 = getelementptr inbounds %struct.Fish, %struct.Fish* %8, i32 0, i32 1
    //  store i32 2, i32* %9, align 4
    //  %10 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 3
    //  %11 = getelementptr inbounds %struct.Fish, %struct.Fish* %10, i32 0, i32 2
    //  store i32 300, i32* %11, align 4
    //  %12 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 0
    //  store i32 0, i32* %12, align 4
    //  %13 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 1
    //  store i32 0, i32* %13, align 4
    //  %14 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 2
    //  store i32 0, i32* %14, align 4
    //  %15 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 3
    //  store i32 0, i32* %15, align 4
    //  %16 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 4
    //  %17 = getelementptr inbounds %struct.Fish, %struct.Fish* %16, i32 0, i32 0
    //  store i32 2, i32* %17, align 4
    //  %18 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 4
    //  %19 = getelementptr inbounds %struct.Fish, %struct.Fish* %18, i32 0, i32 1
    //  store i32 0, i32* %19, align 4
    //  %20 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 4
    //  %21 = getelementptr inbounds %struct.Fish, %struct.Fish* %20, i32 0, i32 2
    //  store i32 100, i32* %21, align 4
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
    //  %4 = alloca i32, align 4
    //  %5 = alloca i32, align 4
    //  %6 = alloca i32, align 4
    //  %7 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 2
    //  %8 = load i32, i32* %7, align 8
    //  store i32 %8, i32* %4, align 4
    //  %9 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 0
    //  %10 = load i32, i32* %9, align 8
    //  store i32 %10, i32* %5, align 4
    //  %11 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 1
    //  %12 = load i32, i32* %11, align 4
    //  store i32 %12, i32* %6, align 4
    //  %13 = load i32, i32* %4, align 4
    //  %14 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 3
    //  %15 = load i32, i32* %14, align 4
    //  %16 = add nsw i32 %15, %13
    //  store i32 %16, i32* %14, align 4
    //  %17 = load i32, i32* %6, align 4
    //  %18 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 2
    //  %19 = load i32, i32* %18, align 8
    //  %20 = add nsw i32 %19, %17
    //  store i32 %20, i32* %18, align 8
    //  %21 = load i32, i32* %5, align 4
    //  %22 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 1
    //  %23 = load i32, i32* %22, align 4
    //  %24 = add nsw i32 %23, %21
    //  store i32 %24, i32* %22, align 4
    //  %25 = load i32, i32* %4, align 4
    //  %26 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 0
    //  %27 = load i32, i32* %26, align 8
    //  %28 = add nsw i32 %27, %25
    //  store i32 %28, i32* %26, align 8
    //  %29 = load i32, i32* %6, align 4
    //  %30 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 0
    //  %31 = load i32, i32* %30, align 8
    //  %32 = add nsw i32 %31, %29
    //  store i32 %32, i32* %30, align 8
    //  %33 = load i32, i32* %5, align 4
    //  %34 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 0
    //  %35 = load i32, i32* %34, align 8
    //  %36 = add nsw i32 %35, %33
    //  store i32 %36, i32* %34, align 8
    //  %37 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 4
    //  %38 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 3
    //  %39 = bitcast %struct.Fish* %37 to i8*
    //  %40 = bitcast %struct.Fish* %38 to i8*
    //  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 8 %39, i8* align 4 %40, i64 12, i1 false)
    //  %41 = getelementptr inbounds %struct.Pair, %struct.Pair* %0, i32 0, i32 1
    //  %42 = bitcast %struct.Storage* %41 to i8*
    //  %43 = bitcast %struct.Storage* %2 to i8*
    //  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %42, i8* align 8 %43, i64 28, i1 false)
    //  ret void
    //}

    //%struct.Fish = type { i32, i32, i32 }
    let fish = Type::Struct {
        id: String::from("Fish"),
        fields: vec![Type::I32, Type::I32, Type::I32],
    };

    //%struct.Parameter = type { i32, i32, i32, %struct.Fish }
    let parameter = Type::Struct {
        id: String::from("Parameter"),
        fields: vec![Type::I32, Type::I32, Type::I32, fish.clone()],
    };

    //%struct.Storage = type { i32, i32, i32, i32, %struct.Fish }
    let storage = Type::Struct {
        id: String::from("Storage"),
        fields: vec![Type::I32, Type::I32, Type::I32, Type::I32, fish.clone()],
    };

    //%struct.Operation = type {}
    let operation = Type::Struct {
        id: String::from("Operation"),
        fields: vec![],
    };

    //%struct.Pair = type { [0 x %struct.Operation], %struct.Storage }
    let pair = Type::Struct {
        id: String::from("Pair"),
        // FIXME: [0 x %struct.Operation]にしたい.
        //        配列をサポートしていない
        fields: vec![operation.clone(), storage.clone()],
    };

    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local void @initial_value() #0 {
    let initial_value: Vec<Instruction> = vec![
        //  %1 = alloca %struct.Parameter, align 4
        //  %2 = alloca %struct.Storage, align 4
        //  %3 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 0
        //  store i32 1, i32* %3, align 4
        //  %4 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 1
        //  store i32 5, i32* %4, align 4
        //  %5 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 2
        //  store i32 0, i32* %5, align 4
        //  %6 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 3
        //  %7 = getelementptr inbounds %struct.Fish, %struct.Fish* %6, i32 0, i32 0
        //  store i32 1, i32* %7, align 4
        //  %8 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 3
        //  %9 = getelementptr inbounds %struct.Fish, %struct.Fish* %8, i32 0, i32 1
        //  store i32 2, i32* %9, align 4
        //  %10 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 3
        //  %11 = getelementptr inbounds %struct.Fish, %struct.Fish* %10, i32 0, i32 2
        //  store i32 300, i32* %11, align 4
        //  %12 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 0
        //  store i32 0, i32* %12, align 4
        //  %13 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 1
        //  store i32 0, i32* %13, align 4
        //  %14 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 2
        //  store i32 0, i32* %14, align 4
        //  %15 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 3
        //  store i32 0, i32* %15, align 4
        //  %16 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 4
        //  %17 = getelementptr inbounds %struct.Fish, %struct.Fish* %16, i32 0, i32 0
        //  store i32 2, i32* %17, align 4
        //  %18 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 4
        //  %19 = getelementptr inbounds %struct.Fish, %struct.Fish* %18, i32 0, i32 1
        //  store i32 0, i32* %19, align 4
        //  %20 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 4
        //  %21 = getelementptr inbounds %struct.Fish, %struct.Fish* %20, i32 0, i32 2
        //  store i32 100, i32* %21, align 4
        //  ret void
    ];
    //}

    //define dso_local void @smart_contract(
    //      %struct.Pair* noalias sret %0,
    //      %struct.Parameter* byval(%struct.Parameter) align 8 %1,
    //      %struct.Storage* byval(%struct.Storage) align 8 %2
    //  ) #0 {
    let instructions = vec![
        //  %4 = alloca i32, align 4
        //  %5 = alloca i32, align 4
        //  %6 = alloca i32, align 4
        Instruction::Alloca {
            ptr: Register::new("%4"),
            ty: Type::I32,
        },
        Instruction::Alloca {
            ptr: Register::new("%5"),
            ty: Type::I32,
        },
        Instruction::Alloca {
            ptr: Register::new("%6"),
            ty: Type::I32,
        },
        //  %7 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 2
        //  %8 = load i32, i32* %7, align 8
        //  store i32 %8, i32* %4, align 4
        Instruction::GetElementPtr {
            result: Register::new("%7"),
            ty: parameter.clone(),
            ptrval: Register::new("%1"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("2")),
            ],
        },
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%8"),
            ptr: Register::new("%7"),
        },
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("8"),
            ptr: Register::new("%4"),
        },
        //  %9 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 0
        //  %10 = load i32, i32* %9, align 8
        //  store i32 %10, i32* %5, align 4
        Instruction::GetElementPtr {
            result: Register::new("%9"),
            ty: parameter.clone(),
            ptrval: Register::new("%1"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("0")),
            ],
        },
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%10"),
            ptr: Register::new("%9"),
        },
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("10"),
            ptr: Register::new("%5"),
        },
        //  %11 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 1
        //  %12 = load i32, i32* %11, align 4
        //  store i32 %12, i32* %6, align 4
        Instruction::GetElementPtr {
            result: Register::new("%11"),
            ty: parameter.clone(),
            ptrval: Register::new("%1"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("1")),
            ],
        },
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%12"),
            ptr: Register::new("%11"),
        },
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("12"),
            ptr: Register::new("%6"),
        },
        //  %13 = load i32, i32* %4, align 4
        //  %14 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 3
        //  %15 = load i32, i32* %14, align 4
        //  %16 = add nsw i32 %15, %13
        //  store i32 %16, i32* %14, align 4
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%13"),
            ptr: Register::new("%4"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%14"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("3")),
            ],
        },
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%15"),
            ptr: Register::new("%14"),
        },
        Instruction::Op {
            ty: Type::I32,
            opcode: Opcode::Add,
            result: Register::new("%16"),
            op1: Register::new("%15"),
            op2: Register::new("%13"),
        },
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("16"),
            ptr: Register::new("%14"),
        },
        //  %17 = load i32, i32* %6, align 4
        //  %18 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 2
        //  %19 = load i32, i32* %18, align 8
        //  %20 = add nsw i32 %19, %17
        //  store i32 %20, i32* %18, align 8
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%17"),
            ptr: Register::new("%6"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%18"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("2")),
            ],
        },
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%19"),
            ptr: Register::new("%18"),
        },
        Instruction::Op {
            ty: Type::I32,
            opcode: Opcode::Add,
            result: Register::new("%20"),
            op1: Register::new("%19"),
            op2: Register::new("%17"),
        },
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("%20"),
            ptr: Register::new("%18"),
        },
        //  %21 = load i32, i32* %5, align 4
        //  %22 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 1
        //  %23 = load i32, i32* %22, align 4
        //  %24 = add nsw i32 %23, %21
        //  store i32 %24, i32* %22, align 4
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%21"),
            ptr: Register::new("%5"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%22"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("1")),
            ],
        },
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%23"),
            ptr: Register::new("%22"),
        },
        Instruction::Op {
            ty: Type::I32,
            opcode: Opcode::Add,
            result: Register::new("%24"),
            op1: Register::new("%23"),
            op2: Register::new("%21"),
        },
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("%24"),
            ptr: Register::new("%22"),
        },
        //  %25 = load i32, i32* %4, align 4
        //  %26 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 0
        //  %27 = load i32, i32* %26, align 8
        //  %28 = add nsw i32 %27, %25
        //  store i32 %28, i32* %26, align 8
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%25"),
            ptr: Register::new("%4"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%26"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("0")),
            ],
        },
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%27"),
            ptr: Register::new("%26"),
        },
        Instruction::Op {
            ty: Type::I32,
            opcode: Opcode::Add,
            result: Register::new("%28"),
            op1: Register::new("%27"),
            op2: Register::new("%25"),
        },
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("%28"),
            ptr: Register::new("%26"),
        },
        //  %29 = load i32, i32* %6, align 4
        //  %30 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 0
        //  %31 = load i32, i32* %30, align 8
        //  %32 = add nsw i32 %31, %29
        //  store i32 %32, i32* %30, align 8
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%29"),
            ptr: Register::new("%6"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%30"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("0")),
            ],
        },
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%31"),
            ptr: Register::new("%30"),
        },
        Instruction::Op {
            ty: Type::I32,
            opcode: Opcode::Add,
            result: Register::new("%32"),
            op1: Register::new("%31"),
            op2: Register::new("%29"),
        },
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("%32"),
            ptr: Register::new("%30"),
        },
        //  %33 = load i32, i32* %5, align 4
        //  %34 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 0
        //  %35 = load i32, i32* %34, align 8
        //  %36 = add nsw i32 %35, %33
        //  store i32 %36, i32* %34, align 8
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%33"),
            ptr: Register::new("%5"),
        },
        Instruction::GetElementPtr {
            result: Register::new("%34"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("0")),
            ],
        },
        Instruction::Load {
            ty: Type::I32,
            result: Register::new("%35"),
            ptr: Register::new("%34"),
        },
        Instruction::Op {
            ty: Type::I32,
            opcode: Opcode::Add,
            result: Register::new("%36"),
            op1: Register::new("%35"),
            op2: Register::new("%33"),
        },
        Instruction::Store {
            ty: Type::I32,
            value: Register::new("%36"),
            ptr: Register::new("%34"),
        },
        //  %37 = getelementptr inbounds %struct.Storage, %struct.Storage* %2, i32 0, i32 4
        //  %38 = getelementptr inbounds %struct.Parameter, %struct.Parameter* %1, i32 0, i32 3
        Instruction::GetElementPtr {
            result: Register::new("%37"),
            ty: storage.clone(),
            ptrval: Register::new("%2"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("4")),
            ],
        },
        Instruction::GetElementPtr {
            result: Register::new("%38"),
            ty: parameter.clone(),
            ptrval: Register::new("%1"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("3")),
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
        //  %41 = getelementptr inbounds %struct.Pair, %struct.Pair* %0, i32 0, i32 1
        Instruction::GetElementPtr {
            result: Register::new("%41"),
            ty: pair.clone(),
            ptrval: Register::new("%0"),
            subsequent: vec![
                (Type::I32, Register::new("0")),
                (Type::I32, Register::new("1")),
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

    let mini_llvm = MiniLlvm {
        structure_types: vec![
            fish.clone(),
            operation.clone(),
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
                result_type: Type::I32,
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

    let michelson_code = compile(mini_llvm);

    let file_name = "complex_smartcontract";
    let command_typecheck =
        format!("#tezos-client --mode mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup run script ./examples/out/{file_name}.tz on storage 'Pair 1 2 3 4 (Pair 5 6 7)' and input 'Pair 8 9 10 (Pair 11 12 13)' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
