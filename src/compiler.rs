//! LLTZ IR を Michelson へのコンパイルする module
use super::lltz_ir::Program;
use crate::lltz_ir::{BackendType, Register, Type};
use std::collections::HashMap;
mod backend;
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
    let mut register2ty: HashMap<Register, BackendType> = HashMap::new();

    /*
     * tyをKeyとして,そのtyの(MichelsonのStackにおける)メモリ領域内の相対位置を返すHashMap
     * 1-indexであることに注意
     */
    let mut memory_ty2stack_ptr: HashMap<BackendType, usize> = HashMap::new();

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
        .unwrap();

    backend::scan(
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

    dbg!(&register2stack_ptr);
    dbg!(&register2ty);
    dbg!(&memory_ty2stack_ptr);
    println!(
        "{}",
        utils::print_michelson_initial_stack_status(
            &register2stack_ptr,
            &register2ty,
            &memory_ty2stack_ptr,
        )
    );

    let mut code = vec![];
    code.append(&mut backend::stack_initialization(
        &register2stack_ptr,
        &register2ty,
        &memory_ty2stack_ptr,
    ));

    code.append(&mut backend::inject_argument_list(
        smart_contract_function,
        &register2stack_ptr,
        &memory_ty2stack_ptr,
    ));

    code.append(&mut backend::compile_instructions(
        &register2stack_ptr,
        &register2ty,
        &memory_ty2stack_ptr,
        &smart_contract_function.instructions,
    ));

    code.append(&mut backend::retrieve_storage_from_memory(
        smart_contract_function,
        &register2stack_ptr,
        &memory_ty2stack_ptr,
    ));

    code.append(&mut backend::retrieve_operations_from_memory(
        smart_contract_function,
        &register2stack_ptr,
        &memory_ty2stack_ptr,
    ));

    code.append(&mut backend::exit(
        &register2stack_ptr,
        &memory_ty2stack_ptr,
    ));

    let parameter = lltz_ir
        .structure_types
        .iter()
        .find(|ty| match ty {
            Type::Struct { id, fields: _ } => id == &String::from("Parameter"),
            _ => false,
        })
        .expect("Parameter型が宣言されていません.")
        .to_entrypoint_ty();

    let storage = lltz_ir
        .structure_types
        .iter()
        .find(|ty| match ty {
            Type::Struct { id, fields: _ } => id == &String::from("Storage"),
            _ => false,
        })
        .expect("Storage型が宣言されていません.")
        .to_entrypoint_ty();

    let michelson_program = program::Program {
        parameter,
        storage,
        code,
    };

    michelson_program.to_string()
}
