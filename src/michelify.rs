mod ast;
use crate::michelify::ast::StackType;
use crate::mlir;
use crate::mlir::ast::{Operation, Value};
use crate::mlir::dialect::michelson::ast::Type;
use michelson_ast::instruction::Instruction as MichelsonInstruction;
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
     * drop an unused stack region
     */
    let mut exit_instructions = exit(&smart_contract, &value_addresses, &type_heap_addresses);
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

///before:            (parameter, storage) |> stack_bottom
///after : [value region] |> [heap region] |> stack_bottom
fn stack_initialization(
    value_addresses: &HashMap<Value, usize>,
    type_heap_addresses: &HashMap<Type, usize>,
) -> Vec<MWrappedInstr> {
    let mut michelson_instructions = vec![
        instruction_row!(MichelsonInstruction::Comment(format!(
            "###### Stack Initialization ######{{"
        ))),
        //TODO: 引数が Option で包まなければいけない型の場合の処理をする
        instruction_row!(
            MichelsonInstruction::Unpair,
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

    michelson_instructions.push(instruction_row!(MichelsonInstruction::Comment(format!(
        "}}#################################"
    ))));

    michelson_instructions
}

///before: [value region] |> [heap region] |> stack_bottom
///after : [value region] |> [heap region] |> stack_bottom
fn compile_operations(
    smart_contract: &Operation,
    value_addresses: &HashMap<Value, usize>,
    _type_heap_addresses: &HashMap<Type, usize>,
) -> Vec<MWrappedInstr> {
    let mut instructions = vec![];

    let operations = smart_contract.regions[0].blocks[0].operations.to_owned();

    for operation in operations {
        let operation = mlir::dialect::Operation::from(operation);
        match operation {
            mlir::dialect::Operation::FuncOp { operation } => {
                use mlir::dialect::func;
                match operation {
                    func::ast::Operation::ReturnOp { .. } => {}
                }
            }
            mlir::dialect::Operation::MichelsonOp { operation } => {
                use mlir::dialect::michelson;
                match operation {
                    michelson::ast::Operation::GetAmountOp { result } => {
                        let address = *value_addresses.get(&result.get_value()).unwrap();
                        instructions.append(
                            &mut vec![
                                MichelsonInstruction::Comment(format!(
                                    "{} = michelson.get_amount() {{",
                                    result.get_value().get_id()
                                )),
                                MichelsonInstruction::Amount,
                                MichelsonInstruction::DigN(address),
                                MichelsonInstruction::Drop,
                                MichelsonInstruction::DugN(address - 1),
                                MichelsonInstruction::Comment("}".to_string()),
                            ]
                            .iter()
                            .map(|instr| instr.to_wrapped_instruction())
                            .collect::<Vec<_>>(),
                        );
                    }
                    michelson::ast::Operation::MakeListOp { result } => {
                        //スタック初期化の際に `nil ty` を既に積んでいるため何もする必要無し
                        instructions.append(
                            &mut vec![MichelsonInstruction::Comment(format!(
                                "{} = michelson.make_list() {{ }}",
                                result.get_value().get_id()
                            ))]
                            .iter()
                            .map(|instr| instr.to_wrapped_instruction())
                            .collect::<Vec<_>>(),
                        )
                    }
                    michelson::ast::Operation::MakePairOp { result, fst, snd } => {
                        let result_address = *value_addresses.get(&result.get_value()).unwrap();
                        let fst_address = *value_addresses.get(&fst.get_value()).unwrap();
                        let snd_address = *value_addresses.get(&snd.get_value()).unwrap();
                        instructions.append(
                            &mut vec![
                                MichelsonInstruction::Comment(format!(
                                    "{} = michelson.make_pair({}, {}) {{",
                                    result.get_value().get_id(),
                                    fst.get_value().get_id(),
                                    snd.get_value().get_id(),
                                )),
                                MichelsonInstruction::DupN(snd_address),
                                MichelsonInstruction::DupN(fst_address + 1),
                                MichelsonInstruction::Pair,
                                MichelsonInstruction::DigN(result_address),
                                MichelsonInstruction::Drop,
                                MichelsonInstruction::Some,
                                MichelsonInstruction::DugN(result_address - 1),
                                MichelsonInstruction::Comment("}".to_string()),
                            ]
                            .iter()
                            .map(|instr| instr.to_wrapped_instruction())
                            .collect::<Vec<_>>(),
                        )
                    }
                }
            }
        }
    }

    instructions
}

///before: [value region] |> [heap region] |> stack_bottom
///after :           (operations, storage) |> stack_bottom
fn exit(
    smart_contract: &Operation,
    value_addresses: &HashMap<Value, usize>,
    type_heap_addresses: &HashMap<Type, usize>,
) -> Vec<MWrappedInstr> {
    let return_op = smart_contract.regions[0].blocks[0]
        .operations
        .last()
        .unwrap()
        .to_owned();
    let value = return_op.operands[0].get_value();
    let address = *value_addresses.get(&value).unwrap();

    let mut instructions = vec![instruction_row!(MichelsonInstruction::Comment(format!(
        "############## Exit ##############{{"
    )))];

    instructions.append(
        &mut vec![
            MichelsonInstruction::DigN(address - 1),
            MichelsonInstruction::AssertSome,
            MichelsonInstruction::DugN(
                value_addresses.iter().len() + type_heap_addresses.iter().len() - 1,
            ),
        ]
        .iter()
        .map(|instr| instr.to_wrapped_instruction())
        .collect::<Vec<_>>(),
    );

    for _ in 0..(value_addresses.iter().len() + type_heap_addresses.iter().len()) - 1 {
        instructions.push(instruction_row!(MichelsonInstruction::Drop));
    }

    instructions.push(instruction_row!(MichelsonInstruction::Comment(format!(
        "}}#################################"
    ))));

    instructions
}
