mod ast;
mod phase;
use crate::mlir::{
    ast::{Operation, Value},
    dialect::michelson::ast::Type,
};
use michelson_ast;
use std::collections::HashMap;

pub fn compile(
    smart_contract: Operation,
) -> Result<michelson_ast::program::Program, Box<dyn std::error::Error>> {
    /*
     * HashMap that returns the position of a Value on the Michelson Stack as its Key
     * Note that it returns the relative position in the register area of Value, using a 1-index
     */
    let mut value_addresses: HashMap<Value, usize> = HashMap::new();

    /*
     * HashMap that returns the relative position in the heap area of Types on
     * the Michelson Stack as its Key. Note that it uses a 1-index.
     */
    let mut type_heap_addresses: HashMap<Type, usize> = HashMap::new();

    /*
     * Variable used to store the relative pointer in the Value area of the Michelson stack
     * for the allocation of a Value in the Value area.
     * It is not used after `scan`.
     */
    let mut value_address_counter = 0;

    /*
     * Variable used to store the relative pointer in the heap area of the Michelson stack
     * for the allocation of a BIG_MAP in the heap area.
     * It is not used after `scan`.
     */
    let mut type_heap_address_counter = 0;

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

    let (parameter, storage) = phase::get_entrypoint_types(&smart_contract);

    /*
     * stack initialization
     */
    let mut code = phase::stack_initialization(&value_addresses, &type_heap_addresses);

    let get_address_closure =
        phase::get_get_address_closure(value_addresses.clone(), type_heap_addresses.clone());

    /*
     * compile operations
     */
    code.append(&mut phase::compile_operations(
        &smart_contract,
        get_address_closure.as_ref(),
    ));

    Ok(michelson_ast::program::Program {
        parameter,
        storage,
        code,
    })
}
