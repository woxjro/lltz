mod ast;
use crate::michelify::ast::StackType;
use crate::mlir::ast::{Operation, Value};
use crate::mlir::dialect::michelson::ast::Type;
use michelson_ast::instruction::Instruction as MInstr;
use michelson_ast::instruction_row;
use michelson_ast::program;
use michelson_ast::ty::Ty as MichelsonType;
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

    dbg!(&value_addresses);

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

fn get_signature(smart_contract: &Operation) -> (MichelsonType, MichelsonType) {
    let args = smart_contract.regions[0].blocks[0].arguments.to_owned();
    if args.len() == 2 {
        let storage_v = args[0].get_value();
        let param_v = args[1].get_value();
        (
            MichelsonType::from(storage_v.get_type()),
            MichelsonType::from(param_v.get_type()),
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
    value_address_counter: &mut usize,
    _type_heap_address_counter: &mut usize,
    value_addresses: &mut HashMap<Value, usize>,
    _type_heap_addresses: &mut HashMap<Type, usize>,
) {
    let operations = smart_contract.regions[0].blocks[0].operations.to_owned();
    for operation in operations {
        //TODO: While や If のような内部に Region を持つ命令の場合は再帰的に scan する必要がある
        for operand in operation.operands {
            let _ = value_addresses
                .entry(operand.get_value())
                .or_insert_with(|| {
                    *value_address_counter += 1;
                    *value_address_counter
                });
        }

        for result in operation.results {
            let _ = value_addresses
                .entry(result.get_value())
                .or_insert_with(|| {
                    *value_address_counter += 1;
                    *value_address_counter
                });
        }
    }
    let args = smart_contract.regions[0].blocks[0].arguments.to_owned();
    for arg in args {
        let _ = value_addresses.entry(arg.get_value()).or_insert_with(|| {
            *value_address_counter += 1;
            *value_address_counter
        });
    }
}

fn stack_initialization(
    value_addresses: &HashMap<Value, usize>,
    type_heap_addresses: &HashMap<Type, usize>,
) -> Vec<MWrappedInstr> {
    let mut michelson_instructions = vec![
        instruction_row!(MInstr::Comment(format!(
            "##################################"
        ))),
        instruction_row!(MInstr::Comment(format!(
            "###### Stack Initialization ######"
        ))),
        instruction_row!(MInstr::Comment(format!(
            "#################################{{"
        ))),
        //TODO: 引数が Option で包まなければいけない型の場合の処理をする
        instruction_row!(
            MInstr::Unpair,
            format!("(parameter, storage) => param : storage")
        ),
    ];

    let mut type_heap_addresses = type_heap_addresses
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>();
    type_heap_addresses.sort_by(|a, b| (a.1).cmp(&b.1));

    for (_ty, _address) in type_heap_addresses.iter().rev() {
        todo!()
    }

    let mut value_addresses = value_addresses
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>();
    value_addresses.sort_by(|a, b| (a.1).cmp(&b.1));
    for (i, (value, _address)) in value_addresses.iter().rev().enumerate() {
        if i <= 1 {
            continue; // parameter と storage の初期値の処理をスキップ
        }
        let stack_type: StackType = value.get_type().into();
        michelson_instructions.push(instruction_row!(
            stack_type.default_value_instruction(),
            format!(
                "{} : {}",
                value.get_id(),
                MichelsonType::from(value.get_type()).to_string()
            )
        ));
    }

    michelson_instructions.push(instruction_row!(MInstr::Comment(format!(
        "}}#################################"
    ))));

    michelson_instructions
}

fn compile_operations(
    _operation: &Operation,
    _value_addresses: &HashMap<Value, usize>,
    _type_heap_addresses: &HashMap<Type, usize>,
) -> Vec<MWrappedInstr> {
    //todo!()
    vec![]
}
fn construct_return_value(
    _operation: &Operation,
    _value_addresses: &HashMap<Value, usize>,
) -> Vec<MWrappedInstr> {
    //todo!()
    vec![]
}
fn exit(
    _value_addresses: &HashMap<Value, usize>,
    _type_heap_addresses: &HashMap<Type, usize>,
) -> Vec<MWrappedInstr> {
    //todo!()
    vec![]
}
