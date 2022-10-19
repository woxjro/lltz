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
    //declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #1
    //
    //; Function Attrs: noinline nounwind optnone uwtable
    //define dso_local void @smart_contract(%struct.Pair* noalias sret %0, %struct.Parameter* byval(%struct.Parameter) align 8 %1, %struct.Storage* byval(%struct.Storage) align 8 %2) #0 {
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
        fields: vec![Type::I32, Type::I32, Type::I32, fish.clone()],
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

    let mini_llvm = MiniLlvm {
        structure_types: vec![
            fish.clone(),
            operation.clone(),
            pair.clone(),
            parameter.clone(),
            storage.clone(),
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
                instructions: vec![
                //  %4 = getelementptr inbounds %struct.Pair, %struct.Pair* %0, i32 0, i32 1
                //  %5 = bitcast %struct.Storage* %4 to i8*
                //  %6 = bitcast %struct.Storage* %2 to i8*
                //  call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 4 %5, i8* align 8 %6, i64 36, i1 false)
                //  ret void
                ],
            },
        ],
    };

    let michelson_code = compile(mini_llvm);

    let file_name = "simple_smartcontract";
    let command_typecheck =
        format!("#tezos-client --mode mockup typecheck script ./examples/out/{file_name}.tz\n");
    let command_mock =
        format!("#tezos-client --mode mockup run script ./examples/out/{file_name}.tz on storage 'Unit' and input 'Unit' --trace-stack\n");
    let contents = format!("{command_typecheck}{command_mock}{michelson_code}");
    let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
    file.write_all(contents.as_bytes()).unwrap();
}
