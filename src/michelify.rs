mod ast;
mod phase;
use crate::mlir::ast::{Operation, Value};
use crate::mlir::dialect::michelson::ast::Type;
use michelson_ast;
use std::collections::HashMap;

pub fn compile(smart_contract: Operation) -> michelson_ast::program::Program {
    /*
     * Value を Key として,その Value の Michelson の Stack 上での位置を返す HashMap
     * Value の 1-index でのレジスタ領域における相対位置を返す事に注意
     */
    let mut value_addresses: HashMap<Value, usize> = HashMap::new();

    /*
     * type を Key として,その type の(Michelsonの Stack における)ヒープ領域内の
     * 相対位置を返す HashMap．1-indexであることに注意
     */
    let mut type_heap_addresses: HashMap<Type, usize> = HashMap::new();

    /*
     * Michelson のスタック領域における Value 領域での Value 確保において
     * 既に確保した Value の Value 領域での相対的ポインタを保持しておく為の変数
     * scan を行った後は使わない
     */
    let mut value_address_counter = 0;

    /*
     * Michelsonのスタック領域におけるヒープ領域での BIG_MAP 確保において
     * 既に確保した BIG_MAP のヒープ領域での相対的ポインタを保持しておく為の変数
     * scan を行った後は使わない
     */
    let mut type_heap_address_counter = 0;

    let (parameter, storage) = phase::get_signature(&smart_contract);
    let mut code = vec![];

    /*
     * scan operations
     */
    phase::scan(
        &smart_contract,
        &mut value_address_counter,
        &mut type_heap_address_counter,
        &mut value_addresses,
        &mut type_heap_addresses,
    );

    /*
     * stack initialization
     */
    let mut stack_initialization_instructions =
        phase::stack_initialization(&value_addresses, &type_heap_addresses);
    code.append(&mut stack_initialization_instructions);

    let get_address_closure =
        phase::get_get_address_closure(value_addresses.clone(), type_heap_addresses.clone());

    /*
     * compile operations
     */
    let mut compile_operations_instructions =
        phase::compile_operations(&smart_contract, get_address_closure.as_ref());
    code.append(&mut compile_operations_instructions);

    michelson_ast::program::Program {
        parameter,
        storage,
        code,
    }
}
