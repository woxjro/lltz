//! LLTZ IR を Michelson へのコンパイルする module
use super::lltz_ir::Program;
use crate::lltz_ir::{InnerType, Register, Type};
use std::collections::HashMap;
mod michelify;
mod utils;
use michelson_ast::program;

///入力として LLTZ IR プログラム Program を受け取り，
///その挙動をエミュレートするMichelsonコードを返す関数
pub fn compile(lltz_ir: Program) -> String {
    /*
     * RegisterをKeyとして,そのRegisterのMichelsonのStack上での位置を返すHashMap
     * Registerの1-indexでのレジスタ領域における相対位置を返す事に注意
     */
    let mut register2stack_ptr = HashMap::new();

    /*
     * RegisterをKeyとして,そのRegisterのLLTZの型を返すHashMap
     */
    let mut register2ty: HashMap<Register, InnerType> = HashMap::new();

    /*
     * tyをKeyとして,そのtyの(MichelsonのStackにおける)メモリ領域内の相対位置を返すHashMap
     * 1-indexであることに注意
     */
    let mut memory_ty2stack_ptr: HashMap<InnerType, usize> = HashMap::new();

    /*
     * Michelsonのスタック領域におけるレジスタ領域でのレジスタ確保において
     * 既に確保したレジスタのレジスタ領域での相対的ポインタを保持しておく為の変数
     * scan_registers_and_memoryを行った後は使わない
     */
    let mut stack_ptr = 0;

    /*
     * Michelsonのスタック領域におけるメモリ領域でのBIG_MAP確保において
     * 既に確保したBIG_MAPのメモリ領域での相対的ポインタを保持しておく為の変数
     * scan_registers_and_memoryを行った後は使わない
     */
    let mut memory_ptr = 0;

    let smart_contract_function = lltz_ir
        .functions
        .iter()
        .find(|f| f.function_name == String::from("smart_contract"))
        .expect("A `smart_contract` function corresponding to your smart contract entry point is not defined.");

    michelify::scan(
        &lltz_ir.structure_types,
        &smart_contract_function.argument_list,
        &smart_contract_function.instructions,
        &mut stack_ptr,
        &mut register2stack_ptr,
        &mut memory_ptr,
        &mut memory_ty2stack_ptr,
        &mut register2ty,
    );

    drop(stack_ptr);
    drop(memory_ptr);

    println!(
        "{}",
        utils::print_michelson_initial_stack_status(
            &register2stack_ptr,
            &register2ty,
            &memory_ty2stack_ptr,
        )
    );

    let mut code = vec![];

    /* スタックの初期化 */
    let mut stack_initialization_instructions =
        michelify::stack_initialization(&register2stack_ptr, &register2ty, &memory_ty2stack_ptr);
    let stack_initialization_sum = stack_initialization_instructions
        .iter()
        .map(|instr| instr.count())
        .sum::<usize>();
    code.append(&mut stack_initialization_instructions);

    /* 引数をメモリ領域に挿入 */
    let mut inject_argument_list_instructions = michelify::inject_argument_list(
        smart_contract_function,
        &register2stack_ptr,
        &memory_ty2stack_ptr,
    );
    let inject_argument_list_sum = inject_argument_list_instructions
        .iter()
        .map(|instr| instr.count())
        .sum::<usize>();

    code.append(&mut inject_argument_list_instructions);

    /* 各命令を模倣するMichelsonコードを発行 */
    let mut compiled_instruction = michelify::compile_instructions(
        &register2stack_ptr,
        &register2ty,
        &memory_ty2stack_ptr,
        &smart_contract_function.instructions,
    );
    let compiled_instruction_sum = compiled_instruction
        .iter()
        .map(|instr| instr.count())
        .sum::<usize>();

    code.append(&mut compiled_instruction);

    /* 返り値 storage を構築 */
    let mut retrieve_storage_from_memory_instructions = michelify::retrieve_storage_from_memory(
        smart_contract_function,
        &register2stack_ptr,
        &memory_ty2stack_ptr,
    );
    let retrieve_storage_from_memory_instructions_sum = retrieve_storage_from_memory_instructions
        .iter()
        .map(|instr| instr.count())
        .sum::<usize>();

    code.append(&mut retrieve_storage_from_memory_instructions);

    /* 返り値 operation list を構築 */
    let mut retrieve_operations_from_memory_instructions =
        michelify::retrieve_operations_from_memory(
            smart_contract_function,
            &register2stack_ptr,
            &memory_ty2stack_ptr,
        );
    let retrieve_operations_from_memory_instructions_sum =
        retrieve_operations_from_memory_instructions
            .iter()
            .map(|instr| instr.count())
            .sum::<usize>();

    code.append(&mut retrieve_operations_from_memory_instructions);

    /* スタックの処理 */
    let mut exit_instructions = michelify::exit(&register2stack_ptr, &memory_ty2stack_ptr);
    let exit_instructions_sum = exit_instructions
        .iter()
        .map(|instr| instr.count())
        .sum::<usize>();

    code.append(&mut exit_instructions);
    println!(
        "{0: >12} | {1: >12} | {2: >7} | {3: >12} | {4: >12} | {5: >5} | {6: >6}",
        "stack init", "inject args", "compile", "retrieve st", "retrieve ops", "exit", "total"
    );
    println!(
        "{0: >12} | {1: >12} | {2: >7} | {3: >12} | {4: >12} | {5: >5} | {6: >6}",
        stack_initialization_sum,
        inject_argument_list_sum,
        compiled_instruction_sum,
        retrieve_storage_from_memory_instructions_sum,
        retrieve_operations_from_memory_instructions_sum,
        exit_instructions_sum,
        code.iter().map(|instr| instr.count()).sum::<usize>()
    );

    let parameter = lltz_ir
        .structure_types
        .iter()
        .find(|ty| match ty {
            Type::Struct { id, fields: _ } => id == &String::from("Parameter"),
            _ => false,
        })
        .expect("A structure `Parameter` corresponding to your smart contract argument `parameter` is not defined.")
        .to_entrypoint_ty();

    let storage = lltz_ir
        .structure_types
        .iter()
        .find(|ty| match ty {
            Type::Struct { id, fields: _ } => id == &String::from("Storage"),
            _ => false,
        })
        .expect("A structure `Storage` corresponding to your smart contract argument `storage` is not defined.")
        .to_entrypoint_ty();

    let michelson_program = program::Program {
        parameter,
        storage,
        code,
    };

    michelson_program.to_string()
}
