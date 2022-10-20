use super::mini_llvm::MiniLlvm;
use std::collections::HashMap;
mod backend;
mod utils;
pub fn compile(mini_llvm: MiniLlvm) -> String {
    /*
     * RegisterをKeyとして,そのRegisterのMichelsonのStack上での位置を返すHashMap
     * Registerの1-indexでのレジスタ領域における相対位置を返す事に注意
     */
    let mut register2stack_ptr = HashMap::new();

    /*
     * RegisterをKeyとして,そのRegisterのLLVMの型を返すHashMap
     */
    let mut register2ty = HashMap::new();

    /*
     * tyをKeyとして,そのtyの(MichelsonのStackにおける)メモリ領域内の相対位置を返すHashMap
     * 1-indexであることに注意
     */
    let mut memory_ty2stack_ptr = HashMap::new();

    /*
     * Michelsonのスタック領域におけるレジスタ領域でのレジスタ確保において
     * 既に確保したレジスタのレジスタ領域での相対的ポインタを保持しておく為の変数
     * analyse_registers_and_memoryを行った後は使わない
     */
    let mut stack_ptr = 0;

    /*
     * Michelsonのスタック領域におけるメモリ領域でのBIG_MAP確保において
     * 既に確保したBIG_MAPのメモリ領域での相対的ポインタを保持しておく為の変数
     * analyse_registers_and_memoryを行った後は使わない
     */
    let mut memory_ptr = 0;

    let mut michelson_code = String::new();
    let tab = "       ";
    let tab_depth = 1;

    let smart_contract_function = mini_llvm
        .functions
        .iter()
        .find(|f| f.function_name == String::from("smart_contract"))
        .unwrap();

    backend::analyse_structure_types(
        &mut memory_ty2stack_ptr,
        &mut memory_ptr,
        &mini_llvm.structure_types,
    );

    backend::analyse_argument_list(
        &mut register2stack_ptr,
        &mut register2ty,
        &mut stack_ptr,
        &smart_contract_function.argument_list,
    );

    backend::analyse_registers_and_memory(
        &mut register2stack_ptr,
        &mut register2ty,
        &mut memory_ty2stack_ptr,
        &mut stack_ptr,
        &mut memory_ptr,
        &smart_contract_function.instructions,
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

    michelson_code = backend::prepare(
        michelson_code,
        tab,
        &register2stack_ptr,
        &register2ty,
        &memory_ty2stack_ptr,
    );

    // TODO: ここの３つをまとめたい {{
    michelson_code = backend::prepare_storage(
        smart_contract_function,
        michelson_code,
        tab,
        tab_depth,
        &register2stack_ptr,
        &memory_ty2stack_ptr,
    );

    michelson_code = backend::prepare_parameter(
        smart_contract_function,
        michelson_code,
        tab,
        tab_depth,
        &register2stack_ptr,
        &memory_ty2stack_ptr,
    );

    michelson_code = backend::prepare_pair(
        smart_contract_function,
        michelson_code,
        tab,
        tab_depth,
        &register2stack_ptr,
        &memory_ty2stack_ptr,
    );
    // }}

    michelson_code = backend::body(
        michelson_code,
        tab,
        tab_depth,
        &register2stack_ptr,
        &register2ty,
        &memory_ty2stack_ptr,
        &smart_contract_function.instructions,
    );

    //後処理:レジスタ領域・メモリ領域をDROPする
    michelson_code = backend::exit(
        michelson_code,
        tab,
        &register2stack_ptr,
        &memory_ty2stack_ptr,
        &mini_llvm.structure_types,
    );
    michelson_code
}
