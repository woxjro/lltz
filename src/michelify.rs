use crate::mlir::ast::{Operation, Value};
use crate::mlir::dialect::michelson::ast::Type;
use michelson_ast::program;
use michelson_ast::ty::Ty as MTy;
use michelson_ast::wrapped_instruction::WrappedInstruction as MWrappedInstr;
use std::collections::HashMap;
pub fn compile(smart_contract: Operation) -> String {
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

    let (parameter, storage) = get_signature(&smart_contract);
    dbg!(&parameter);
    dbg!(&storage);
    let mut code = vec![];

    /*
     * scan operations
     */
    scan(
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
        stack_initialization(&value_addresses, &type_heap_addresses);
    code.append(&mut stack_initialization_instructions);

    /*
     * compile operations
     */
    let mut compile_operations_instructions =
        compile_operations(&smart_contract, &value_addresses, &type_heap_addresses);
    code.append(&mut compile_operations_instructions);

    /*
     * construct a return value
     */
    let mut construct_return_value_instructions =
        construct_return_value(&smart_contract, &value_addresses);
    code.append(&mut construct_return_value_instructions);

    /*
     * drop an unused stack region
     */
    let mut exit_instructions = exit(&value_addresses, &type_heap_addresses);
    code.append(&mut exit_instructions);

    let michelson_program = program::Program {
        parameter,
        storage,
        code,
    };

    michelson_program.to_string()
}

fn get_signature(smart_contract: &Operation) -> (MTy, MTy) {
    let args = smart_contract.regions[0].blocks[0].arguments.to_owned();
    if args.len() == 2 {
        let storage_v = args[0].get_value();
        let param_v = args[1].get_value();
        (
            storage_v.get_type().michelify(),
            param_v.get_type().michelify(),
        )
    } else {
        panic!(
            "A smart_contract function is being given {} arguments instead of 2.",
            args.len()
        )
    }
}

fn scan(
    smart_contract: &Operation,
    _value_address_counter: &mut usize,
    _type_heap_address_counter: &mut usize,
    _value_addresses: &mut HashMap<Value, usize>,
    _type_heap_addresses: &mut HashMap<Type, usize>,
) {
    let args = smart_contract.regions[0].blocks[0].arguments.to_owned();
    for _arg in args {
        todo!()
    }
    todo!()
}
fn stack_initialization(
    _value_addresses: &HashMap<Value, usize>,
    _type_heap_addresses: &HashMap<Type, usize>,
) -> Vec<MWrappedInstr> {
    todo!()
}
fn compile_operations(
    _operation: &Operation,
    _value_addresses: &HashMap<Value, usize>,
    _type_heap_addresses: &HashMap<Type, usize>,
) -> Vec<MWrappedInstr> {
    todo!()
}
fn construct_return_value(
    _operation: &Operation,
    _value_addresses: &HashMap<Value, usize>,
) -> Vec<MWrappedInstr> {
    todo!()
}
fn exit(
    _value_addresses: &HashMap<Value, usize>,
    _type_heap_addresses: &HashMap<Type, usize>,
) -> Vec<MWrappedInstr> {
    todo!()
}
