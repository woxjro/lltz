use crate::michelify::ast::StackType;
use crate::mlir;
use crate::mlir::ast::{Operation, Value};
use crate::mlir::dialect::michelson::ast::Type;
use michelson_ast::instruction::Instruction as MichelsonInstruction;
use michelson_ast::instruction_row;
use michelson_ast::ty::Ty as MichelsonType;
use michelson_ast::wrapped_instruction::WrappedInstruction as MWrappedInstr;
use std::collections::HashMap;

pub enum GetAddressClosureArg {
    Value(Value),
    Type(Type),
    StackSize,
}

pub fn get_get_address_closure(
    value_addresses: HashMap<Value, usize>,
    type_heap_addresses: HashMap<Type, usize>,
) -> Box<dyn Fn(GetAddressClosureArg) -> usize> {
    Box::new(move |arg| match arg {
        GetAddressClosureArg::Value(value) => *value_addresses.get(&value).unwrap(),
        GetAddressClosureArg::Type(ty) => {
            value_addresses.len() + type_heap_addresses.get(&ty).unwrap()
        }
        GetAddressClosureArg::StackSize => value_addresses.len() + type_heap_addresses.len(),
    })
}

pub fn get_signature(smart_contract: &Operation) -> (MichelsonType, MichelsonType) {
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

pub fn scan(
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
pub fn stack_initialization(
    value_addresses: &HashMap<Value, usize>,
    type_heap_addresses: &HashMap<Type, usize>,
) -> Vec<MWrappedInstr> {
    let mut michelson_instructions = vec![
        instruction_row!(MichelsonInstruction::Comment(format!(
            "------ stack initialization ------ {{"
        ))),
        //TODO: 引数が Option で包まなければいけない型の場合の処理をする
        instruction_row!(
            MichelsonInstruction::Unpair,
            format!("(parameter, storage) => param |> storage")
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
        "---------------------------------- }}"
    ))));

    michelson_instructions
}

///before: [value region] |> [heap region] |> stack_bottom
///after : [value region] |> [heap region] |> stack_bottom
pub fn compile_operations(
    smart_contract: &Operation,
    get_address_closure: &(dyn Fn(GetAddressClosureArg) -> usize),
) -> Vec<MWrappedInstr> {
    let mut instructions = vec![];

    let operations = smart_contract.regions[0].blocks[0].operations.to_owned();

    for operation in operations {
        let operation = mlir::dialect::Operation::from(operation);
        match operation {
            mlir::dialect::Operation::MichelsonOp { operation } => {
                use mlir::dialect::michelson;
                match operation {
                    michelson::ast::Operation::GetUnitOp { result } => {
                        let address =
                            (*get_address_closure)(GetAddressClosureArg::Value(result.get_value()));
                        instructions.append(
                            &mut vec![
                                MichelsonInstruction::Comment(format!(
                                    "{} = michelson.get_unit() {{",
                                    result.get_value().get_id()
                                )),
                                MichelsonInstruction::Unit,
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
                    michelson::ast::Operation::GetAmountOp { result } => {
                        let address =
                            (*get_address_closure)(GetAddressClosureArg::Value(result.get_value()));
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
                        let result_address =
                            (*get_address_closure)(GetAddressClosureArg::Value(result.get_value()));
                        let fst_address =
                            (*get_address_closure)(GetAddressClosureArg::Value(fst.get_value()));
                        let snd_address =
                            (*get_address_closure)(GetAddressClosureArg::Value(snd.get_value()));
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
            mlir::dialect::Operation::FuncOp { operation } => {
                use mlir::dialect::func;
                match operation {
                    //before: [value region] |> [heap region] |> stack_bottom
                    //after :           (operations, storage) |> stack_bottom
                    func::ast::Operation::ReturnOp { operands } => {
                        let value = operands[0].get_value();
                        let address =
                            (*get_address_closure)(GetAddressClosureArg::Value(value.to_owned()));
                        let stack_size = (*get_address_closure)(GetAddressClosureArg::StackSize);
                        instructions.append(&mut vec![instruction_row!(
                            MichelsonInstruction::Comment(format!(
                                "func.return {} {{",
                                value.get_id()
                            ))
                        )]);

                        instructions.append(
                            &mut vec![
                                MichelsonInstruction::DigN(address - 1),
                                MichelsonInstruction::AssertSome,
                                MichelsonInstruction::DugN(stack_size - 1),
                            ]
                            .iter()
                            .map(|instr| instr.to_wrapped_instruction())
                            .collect::<Vec<_>>(),
                        );

                        for _ in 0..stack_size - 1 {
                            instructions.push(instruction_row!(MichelsonInstruction::Drop));
                        }

                        instructions.push(instruction_row!(MichelsonInstruction::Comment(
                            "}".to_string()
                        )));
                    }
                }
            }
        }
    }

    instructions
}
