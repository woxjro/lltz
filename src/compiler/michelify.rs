//! 以下のコンパイルフローにおけるLLTZ IRからMichelsonへのコンパイル（michelify）を担当するモジュール
//! LLVM IR ---> LLTZ IR ---> Michelson

mod helper;
mod inject;
mod scan;
use crate::lltz_ir::{
    Arg, Condition, Function, InnerType, Instruction, Opcode, Register, Type, Value,
};
use michelson_ast::instruction::Instruction as MInstr;
use michelson_ast::instruction_row;
use michelson_ast::ty::Ty as MTy;
use michelson_ast::val::Val as MVal;
use michelson_ast::wrapped_instruction::WrappedInstruction as MWrappedInstr;
use std::collections::HashMap;

///Programの構造体宣言，引数リスト，命令列を受け取り，それらに現れるレジスタ，メモリや型
///などを調べる．
pub fn scan(
    structure_types: &Vec<Type>,
    argument_list: &Vec<Arg>,
    instructions: &Vec<Instruction>,
    stack_ptr: &mut usize,
    register2stack_ptr: &mut HashMap<Register, usize>,
    memory_ptr: &mut usize,
    memory_ty2stack_ptr: &mut HashMap<InnerType, usize>,
    register2ty: &mut HashMap<Register, InnerType>,
) {
    scan::scan_structure_types(memory_ty2stack_ptr, memory_ptr, structure_types);

    scan::scan_argument_list(register2stack_ptr, register2ty, stack_ptr, argument_list);

    scan::scan_registers_and_memory(
        register2stack_ptr,
        register2ty,
        memory_ty2stack_ptr,
        stack_ptr,
        memory_ptr,
        structure_types,
        instructions,
    );
}

///（主に）Programの`smart_contract_function`を受け取り，そのargument_listである
///スマートコントラクト引数をメモリ領域に格納する．
pub fn inject_argument_list(
    smart_contract_function: &Function,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
) -> Vec<MWrappedInstr> {
    let mut res = vec![
        instruction_row!(MInstr::Comment("######## Inject Arguments ########".to_string())),
        instruction_row!(MInstr::Comment("#################################{".to_string())),
    ];
    res.append(&mut inject::inject_storage(
        smart_contract_function,
        register2stack_ptr,
        memory_ty2stack_ptr,
    ));

    res.append(&mut inject::inject_parameter(
        smart_contract_function,
        register2stack_ptr,
        memory_ty2stack_ptr,
    ));

    res.append(&mut inject::inject_pair(
        smart_contract_function,
        register2stack_ptr,
        memory_ty2stack_ptr,
    ));

    res.push(instruction_row!(MInstr::Comment("}#################################".to_string())));
    res
}

///michelson_codeを受け取り、レジスタ領域とメモリ領域を構築するMichelson命令を発行する．
///レジスタ型環境（register2ty, register2stack_ptr）とメモリ型環境（memory_ty2stack_ptr）
///を受け取り,それらに相当するMichelson命令をスタックにPUSHする
/// before:                               (storage, parameter)
/// after:  (storage, parameter):[register area]:[memory area]
pub fn stack_initialization(
    register2stack_ptr: &HashMap<Register, usize>,
    register2ty: &HashMap<Register, InnerType>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
) -> Vec<MWrappedInstr> {
    let mut michelson_instructions = vec![
        instruction_row!(MInstr::Comment("##################################".to_string())),
        instruction_row!(MInstr::Comment("###### Stack Initialization ######".to_string())),
        instruction_row!(MInstr::Comment("#################################{".to_string())),
    ];
    let memory_ty2stack_ptr = memory_ty2stack_ptr.clone();
    let mut memory_ty2stack_ptr_sorted = memory_ty2stack_ptr
        .iter()
        .map(|(k, v)| (k.clone(), *v))
        .collect::<Vec<_>>();
    memory_ty2stack_ptr_sorted.sort_by(|a, b| (a.1).cmp(&b.1));
    for (ty, _v) in memory_ty2stack_ptr_sorted.iter().rev() {
        let comment = format!("memory for {lltz_ty_name}", lltz_ty_name = ty.get_name());

        michelson_instructions.append(&mut vec![
            instruction_row!(MInstr::Push {
                ty: MTy::Int,
                val: MVal::Int(0),
            }),
            instruction_row!(
                MInstr::EmptyMap {
                    kty: MTy::Int,
                    vty: ty.to_memory_ty(),
                },
                comment
            ),
            instruction_row!(MInstr::Pair),
        ]);
    }

    let mut register2stack_ptr_sorted = register2stack_ptr.iter().collect::<Vec<_>>();
    register2stack_ptr_sorted.sort_by(|a, b| (b.1).cmp(a.1));

    for (reg, _ptr) in register2stack_ptr_sorted {
        let ty = register2ty.get(reg).unwrap();

        let comment = format!(
            "for reg {id} : {lltz_ty_name}",
            lltz_ty_name = ty.get_name(),
            id = reg.get_id()
        );

        match ty {
            InnerType::Option(_inner) => {
                michelson_instructions.push(instruction_row!(
                    InnerType::default_value_instruction(ty),
                    comment
                ));
            }
            _ => michelson_instructions.push(instruction_row!(
                InnerType::default_value_instruction(ty),
                comment
            )),
        };
    }
    //(param, storage)を一番上に持ってくる
    michelson_instructions.push(instruction_row!(MInstr::DigN(
        register2stack_ptr.len() + memory_ty2stack_ptr.len()
    )));
    michelson_instructions.push(instruction_row!(MInstr::Comment("}#################################".to_string())));

    michelson_instructions
}

///LLTZ IRの命令列instructionsを受け取り，それらの挙動をエミュレートする
///Michelson コードを発行する関数．
///レジスタ型環境（register2ty（これは今回は無し）, register2stack_ptr）と
///メモリ型環境（memory_ty2stack_ptr）を参考にコンパイルしていく.
///tab,tab_depthはMichelsonコードのフォーマットのために使う
pub fn compile_instructions(
    register2stack_ptr: &HashMap<Register, usize>,
    register2ty: &HashMap<Register, InnerType>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
    instructions: &Vec<Instruction>,
) -> Vec<MWrappedInstr> {
    let mut res = vec![
        instruction_row!(MInstr::Comment("###### Compile Instructions ######".to_string())),
        instruction_row!(MInstr::Comment("#################################{".to_string())),
    ];
    for instruction in instructions {
        match instruction {
            Instruction::Alloca { ptr, ty } => {
                res.append(&mut helper::alloca::exec_alloca(
                    ptr,
                    ty,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                ));
            }
            Instruction::Store { ty, value, ptr } => {
                let memory_ptr = memory_ty2stack_ptr.get(&InnerType::from(ty)).unwrap();

                let mut instructions = vec![
                    [MInstr::Comment(format!(
                        "store {} {}, {}* {} {{",
                        Type::get_name(ty),
                        value.to_string(),
                        Type::get_name(ty),
                        ptr.get_id()
                    ))]
                    .iter()
                    .map(|instr| instr.to_wrapped_instruction())
                    .collect::<Vec<_>>(),
                    match value {
                        Value::Register(register) => {
                            vec![MInstr::DupN(*register2stack_ptr.get(register).unwrap())]
                        }
                        Value::Const(cnst) => {
                            if cnst.has_default_value() {
                                vec![cnst.get_push_instruction()]
                            } else {
                                vec![cnst.get_push_instruction(), MInstr::Some]
                            }
                        }
                    }
                    .iter()
                    .map(|instr| instr.to_wrapped_instruction())
                    .collect::<Vec<_>>(),
                    vec![
                        MInstr::Some,
                        MInstr::DigN(register2stack_ptr.len() + memory_ptr),
                        MInstr::Unpair,
                        MInstr::DigN(2),
                        MInstr::DupN(*register2stack_ptr.get(ptr).unwrap() + 3),
                        MInstr::Update,
                        MInstr::Pair,
                        MInstr::DugN(register2stack_ptr.len() + memory_ptr - 1),
                        MInstr::Comment("}".to_string()),
                    ]
                    .iter()
                    .map(|instr| instr.to_wrapped_instruction())
                    .collect::<Vec<_>>(),
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::Load { result, ty, ptr } => {
                let memory_ptr = memory_ty2stack_ptr.get(&InnerType::from(ty)).unwrap();

                let mut instructions = vec![
                    MInstr::Comment(format!(
                        "{} = load {}, {}* {} {{",
                        result.get_id(),
                        Type::get_name(ty),
                        Type::get_name(ty),
                        ptr.get_id()
                    )),
                    MInstr::DupN(register2stack_ptr.len() + memory_ptr),
                    MInstr::Car,
                    MInstr::DupN(register2stack_ptr.get(ptr).unwrap() + 1),
                    MInstr::Get,
                    MInstr::AssertSome,
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(*register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::GetElementPtr {
                result,
                ty,
                ptrval,
                subsequent,
            } => {
                let memory_ptr = memory_ty2stack_ptr.get(&InnerType::from(ty)).unwrap();

                // FIXME TODO: subsequent[1]で決め打ちで取得しているので直したい.
                //              (...が, これ以外無い気がする)
                let (_, value) = &subsequent[1];

                let mut instructions = vec![
                    MInstr::Comment(format!(
                        "{} = getElementPtr {}, {}*, {} {{",
                        result.get_id(),
                        Type::get_name(ty),
                        Type::get_name(ty),
                        ptrval.get_id()
                    )),
                    MInstr::DupN(register2stack_ptr.len() + memory_ptr),
                    MInstr::Car, // bm
                    MInstr::DupN(register2stack_ptr.get(ptrval).unwrap() + 1),
                    MInstr::Get,        //some(map)
                    MInstr::AssertSome, //map
                    match value {
                        Value::Register(register) => {
                            MInstr::DupN(register2stack_ptr.get(register).unwrap() + 1)
                            //int:map
                        }
                        Value::Const(cnst) => cnst.get_push_instruction(),
                    },
                    MInstr::Get,
                    MInstr::AssertSome, //ptr
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::If {
                reg,
                code_block_t,
                code_block_f,
            } => {
                let instr1 = compile_instructions(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    code_block_t,
                );
                let instr2 = compile_instructions(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    code_block_f,
                );

                let mut instructions = vec![
                    MInstr::Comment("if {".to_string()),
                    MInstr::DupN(*register2stack_ptr.get(reg).unwrap()),
                    MInstr::If { instr1, instr2 },
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::While {
                cond,
                cond_block,
                loop_block,
            } => {
                /*
                 * cond
                 * DUP id
                 * LOOP {
                 *  loop_body
                 *  cond
                 *  DUP id
                 * }
                 */
                let cond_instr = compile_instructions(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    cond_block,
                );

                let loop_instr = compile_instructions(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    loop_block,
                );

                let mut instr: Vec<MWrappedInstr> = vec![];
                instr.append(&mut loop_instr.clone());
                instr.append(&mut cond_instr.clone());
                instr.push(
                    MInstr::DupN(*register2stack_ptr.get(cond).unwrap()).to_wrapped_instruction(),
                );

                let mut instructions = vec![
                    vec![MInstr::Comment("while {".to_string()).to_wrapped_instruction()],
                    cond_instr.clone(),
                    vec![
                        MInstr::DupN(*register2stack_ptr.get(cond).unwrap())
                            .to_wrapped_instruction(),
                        MInstr::Loop { instr }.to_wrapped_instruction(),
                    ],
                    vec![MInstr::Comment("}".to_string()).to_wrapped_instruction()],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::Op {
                ty,
                opcode,
                result,
                op1,
                op2,
            } => {
                let mut instructions = vec![
                    MInstr::Comment(format!(
                        "{} = {} {} {} {} {{",
                        result.get_id(),
                        opcode.to_string(),
                        Type::get_name(ty),
                        op1.to_string(),
                        op2.to_string(),
                    )),
                    match op2 {
                        Value::Register(register) => {
                            MInstr::DupN(*register2stack_ptr.get(register).unwrap())
                        }
                        Value::Const(cnst) => cnst.get_push_instruction(),
                    },
                    match op1 {
                        Value::Register(register) => {
                            MInstr::DupN(*register2stack_ptr.get(register).unwrap() + 1)
                        }
                        Value::Const(cnst) => cnst.get_push_instruction(),
                    },
                    match opcode {
                        Opcode::Add => MInstr::Add,
                        Opcode::Sub => MInstr::Sub,
                        Opcode::Mul => MInstr::Mul,
                    },
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(*register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::LlvmMemcpy { dest, src, ty } => {
                res.append(&mut helper::llvm_memcpy::exec_llvm_memcpy(
                    dest,
                    src,
                    ty,
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                ));
            }
            Instruction::Icmp {
                result,
                cond,
                ty: _,
                op1,
                op2,
            } => {
                let mut instructions = vec![
                    vec![
                        MInstr::Comment(format!(
                            "{} = icmp {} {} {{", //TODO: icmp -> cond.to_string()
                            result.get_id(),
                            op1.to_string(),
                            op2.to_string(),
                        )),
                        match op1 {
                            Value::Register(register) => {
                                MInstr::DupN(*register2stack_ptr.get(register).unwrap())
                            }
                            Value::Const(cnst) => cnst.get_push_instruction(),
                        },
                        match op2 {
                            Value::Register(register) => {
                                MInstr::DupN(register2stack_ptr.get(register).unwrap() + 1)
                            }
                            Value::Const(cnst) => cnst.get_push_instruction(),
                        },
                    ]
                    .iter()
                    .map(|instr| instr.to_wrapped_instruction())
                    .collect::<Vec<_>>(),
                    // TODO: 他のConditionについても実装
                    match cond {
                        Condition::Eq => {
                            vec![MInstr::Compare, MInstr::Eq]
                        }
                        Condition::Slt => {
                            vec![MInstr::Sub, MInstr::Gt]
                        }
                        _ => {
                            vec![MInstr::Compare]
                        }
                    }
                    .iter()
                    .map(|instr| instr.to_wrapped_instruction())
                    .collect::<Vec<_>>(),
                    vec![
                        MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(register2stack_ptr.get(result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_wrapped_instruction())
                    .collect::<Vec<_>>(),
                    vec![MInstr::Comment("}".to_string()).to_wrapped_instruction()],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetAmount { result } => {
                let mut instructions = vec![
                    MInstr::Comment(format!("{} = MichelsonGetAmount {{", result.get_id())),
                    MInstr::Amount,
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(*register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetBalance { result } => {
                let mut instructions = vec![
                    MInstr::Comment(format!("{} = MichelsonGetBalance {{", result.get_id())),
                    MInstr::Balance,
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(*register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetTotalVotingPower { result } => {
                let mut instructions = vec![
                    MInstr::Comment(format!(
                        "{} = MichelsonGetTotalVotingPower {{",
                        result.get_id()
                    )),
                    MInstr::TotalVotingPower,
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(*register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetLevel { result } => {
                let mut instructions = vec![
                    MInstr::Comment(format!("{} = MichelsonGetLevel {{", result.get_id())),
                    MInstr::Level,
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(*register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetSender { result } => {
                let mut instructions = vec![
                    MInstr::Comment(format!("{} = MichelsonGetSender {{", result.get_id())),
                    MInstr::Sender,
                    MInstr::Some, // to (option address)
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(*register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetSource { result } => {
                let mut instructions = vec![
                    MInstr::Comment(format!("{} = MichelsonGetSource {{", result.get_id())),
                    MInstr::Source,
                    MInstr::Some, // to (option address)
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(*register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetSelfAddress { result } => {
                let mut instructions = vec![
                    MInstr::Comment(format!("{} = MichelsonGetSelfAddress {{", result.get_id())),
                    MInstr::SelfAddress,
                    MInstr::Some, // to (option address)
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(*register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetSelf { result } => {
                let mut instructions = vec![
                    MInstr::Comment(format!("{} = MichelsonGetSelf {{", result.get_id())),
                    MInstr::Slf,
                    MInstr::Some, // to (option contract <ty>)
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(*register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonContract {
                result,
                ty,
                address,
            } => {
                let mut instructions = vec![
                    MInstr::Comment(format!("{} = MichelsonContract {{", result.get_id())),
                    MInstr::DupN(*register2stack_ptr.get(address).unwrap()),
                    MInstr::AssertSome,
                    MInstr::Contract {
                        ty: ty.to_entrypoint_ty(),
                    },
                    MInstr::Some,
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonAssertSome { result, ty, value } => {
                let mut instructions = vec![
                    MInstr::Comment(format!(
                        "{} = MichelsonAssertSome {} {} {{",
                        result.get_id(),
                        InnerType::from(ty).to_michelson_ty().to_string(),
                        value.get_id()
                    )),
                    MInstr::DupN(*register2stack_ptr.get(value).unwrap()),
                    MInstr::AssertSome,
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(*register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonTransferTokens {
                result,
                init,
                tokens,
                contract,
            } => {
                let mut instructions = vec![
                    MInstr::Comment(format!(
                        "{} = MichelsonTransferTokens {} {} {} {{",
                        result.get_id(),
                        init.get_id(),
                        tokens.to_string(),
                        contract.get_id()
                    )),
                    MInstr::DupN(*register2stack_ptr.get(contract).unwrap()),
                    MInstr::AssertSome,
                    match tokens {
                        Value::Register(register) => {
                            MInstr::DupN(register2stack_ptr.get(register).unwrap() + 1)
                        }
                        Value::Const(cnst) => cnst.get_push_instruction(),
                    },
                    MInstr::Unit, // FIXME: unit しか対応していない...
                    MInstr::TransferTokens,
                    MInstr::Some,
                    MInstr::DigN(*register2stack_ptr.get(result).unwrap()),
                    MInstr::Drop,
                    MInstr::DugN(register2stack_ptr.get(result).unwrap() - 1),
                    MInstr::Comment("}".to_string()),
                ]
                .iter()
                .map(|instr| instr.to_wrapped_instruction())
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
        };
    }

    res.push(instruction_row!(MInstr::Comment("}#################################".to_string())));

    res
}

///Michelsonコントラクトとして最後の返り値の準備をする段階において、
///返り値となるStorageをメモリ領域から回収し, Michelsonの入力Storageの
///型に合わせた状態でスタックのトップに持ってくる関数
/// input:                 [register]:[memory]
///output: encoded_storage:[register]:[memory]
pub fn retrieve_storage_from_memory(
    smart_contract_function: &Function,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
) -> Vec<MWrappedInstr> {
    let Arg {
        reg,
        ty: pair_ty_ptr,
    } = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(ty) {
            Type::Struct { id, fields: _ } => id == *"Pair",
            _ => false,
        })
        .unwrap();

    let Arg {
        reg: _,
        ty: storage_ty_ptr,
    } = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(ty) {
            Type::Struct { id, fields: _ } => id == *"Storage",
            _ => false,
        })
        .unwrap();

    let storage_ty = Type::deref(storage_ty_ptr);
    let storage_memory_ptr = memory_ty2stack_ptr
        .get(&InnerType::from(&storage_ty))
        .unwrap();
    let pair_memory_ptr = memory_ty2stack_ptr
        .get(&InnerType::deref(&InnerType::from(pair_ty_ptr)))
        .unwrap();
    let mut michelson_instructions = vec![];
    michelson_instructions.append(
        &mut vec![
            MInstr::Comment("Construct a storage {".to_string()),
            MInstr::DupN(register2stack_ptr.len() + pair_memory_ptr),
            MInstr::Car,
            MInstr::Push {
                ty: MTy::Int,
                val: MVal::Int((*register2stack_ptr.get(reg).unwrap()).try_into().unwrap()),
            },
            MInstr::Get,
            MInstr::AssertSome,
            MInstr::Push {
                ty: MTy::Int,
                val: MVal::Int(1),
            }, // StorageのIndex(=1)
            MInstr::Get,
            MInstr::AssertSome, //Storage Ptr
            MInstr::DupN(register2stack_ptr.len() + storage_memory_ptr + 1),
            MInstr::Car,
            MInstr::Swap,
            MInstr::Get,
            MInstr::AssertSome, //Storage MAP Instance
        ]
        .iter()
        .map(|instr| instr.to_wrapped_instruction())
        .collect::<Vec<_>>(),
    );

    match storage_ty.clone() {
        Type::Struct { id, fields } => {
            if fields.len() >= 2 {
                //逆順にスタックにencodeしたものを積んでいき、最後にPAIR nまとめる.
                for (field_idx, field) in fields.iter().enumerate().rev() {
                    michelson_instructions.append(&mut retrieve_storage_field_from_memory(
                        field_idx,
                        field,
                        vec![fields.len() - field_idx],
                        register2stack_ptr,
                        memory_ty2stack_ptr,
                    ));
                }
                michelson_instructions.append(&mut vec![
                    instruction_row!(
                        MInstr::PairN(fields.len()),
                        format!("PACK Struct {{ {id} }}")
                    ),
                    MInstr::Swap.to_wrapped_instruction(),
                    instruction_row!(MInstr::Drop, format!("Storage MAP Instance")),
                ]);
            } else if fields.len() == 1 {
                todo!()
            } else {
                michelson_instructions.push(MInstr::Drop.to_wrapped_instruction());
                michelson_instructions.push(MInstr::Unit.to_wrapped_instruction());
            }
        }
        _ => {
            panic!("StorageがStruct型ではなくPrimitive型になっています.")
        }
    }
    michelson_instructions.push(MInstr::Comment("}".to_string()).to_wrapped_instruction());

    michelson_instructions
}

///FIXME: 方針はあっているが少しややこしい.
fn retrieve_storage_field_from_memory(
    field_idx: usize,
    field: &Type,
    path: Vec<usize>,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
) -> Vec<MWrappedInstr> {
    let memory_ptr = memory_ty2stack_ptr.get(&InnerType::from(field)).unwrap();
    match field {
        Type::Struct {
            id: child_id,
            fields: child_fields,
        } => {
            //TODO: child_fields.len() > 2, == 1, == 0で場合分け
            let mut michelson_instructions: Vec<MWrappedInstr> = vec![
                MInstr::Comment("{".to_string()).to_wrapped_instruction(),
                instruction_row!(MInstr::DupN(path[path.len() - 1]), format!("MAP instance")),
                MInstr::Push {
                    ty: MTy::Int,
                    val: MVal::Int(field_idx.try_into().unwrap()),
                }
                .to_wrapped_instruction(),
                MInstr::Get.to_wrapped_instruction(),
                MInstr::AssertSome.to_wrapped_instruction(),
                instruction_row!(
                    MInstr::DupN(
                        register2stack_ptr.len() + memory_ptr + path.iter().sum::<usize>() + 1,
                    ),
                    format!("memory: {}", Type::get_name(field))
                ),
                MInstr::Car.to_wrapped_instruction(),
                MInstr::Swap.to_wrapped_instruction(),
                MInstr::Get.to_wrapped_instruction(),
                MInstr::AssertSome.to_wrapped_instruction(),
            ];
            for (child_field_idx, child_field) in child_fields.iter().enumerate().rev() {
                let new_path =
                    [path.clone(), vec![child_fields.len() - child_field_idx]].concat();
                michelson_instructions.append(&mut retrieve_storage_field_from_memory(
                    child_field_idx,
                    child_field,
                    new_path,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                ));
            }
            michelson_instructions.append(&mut vec![
                instruction_row!(
                    MInstr::PairN(child_fields.len()),
                    format!("PACK Struct {{ {child_id} }}")
                ),
                MInstr::Swap.to_wrapped_instruction(),
                instruction_row!(MInstr::Drop, format!("child field MAP Instance")),
                MInstr::Comment("}".to_string()).to_wrapped_instruction(),
            ]);

            michelson_instructions
        }
        _ => {
            vec![
                MInstr::Comment("{".to_string()).to_wrapped_instruction(),
                instruction_row!(MInstr::DupN(path[path.len() - 1]), format!("MAP instance")),
                MInstr::Push {
                    ty: MTy::Int,
                    val: MVal::Int(field_idx.try_into().unwrap()),
                }
                .to_wrapped_instruction(),
                MInstr::Get.to_wrapped_instruction(),
                MInstr::AssertSome.to_wrapped_instruction(),
                instruction_row!(
                    MInstr::DupN(
                        register2stack_ptr.len() + memory_ptr + path.iter().sum::<usize>() + 1,
                    ),
                    format!("memory: {}", Type::get_name(field))
                ),
                MInstr::Car.to_wrapped_instruction(),
                MInstr::Swap.to_wrapped_instruction(),
                MInstr::Get.to_wrapped_instruction(),
                MInstr::AssertSome.to_wrapped_instruction(),
                instruction_row!(MInstr::Comment("}".to_string())),
            ]
        }
    }
}

/// input:                     encoded_storage :[register]:[memory]
///output:  ([list operation], encoded_storage):[register]:[memory]
pub fn retrieve_operations_from_memory(
    smart_contract_function: &Function,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
) -> Vec<MWrappedInstr> {
    let Arg {
        reg,
        ty: pair_ty_ptr,
    } = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(ty) {
            Type::Struct { id, fields: _ } => id == *"Pair",
            _ => false,
        })
        .unwrap();

    let pair_memory_ptr = memory_ty2stack_ptr
        .get(&InnerType::from(&Type::deref(pair_ty_ptr)))
        .unwrap();

    let _pair_fields = match Type::deref(pair_ty_ptr) {
        Type::Struct { id: _, fields } => fields,
        _ => panic!(),
    };
    let operation_arr_ty = _pair_fields
        .iter()
        .find(|&field| match field {
            Type::Array { .. } => true,
            _ => false,
        })
        .unwrap();

    let operation_arr_memory_ptr = memory_ty2stack_ptr
        .get(&InnerType::from(operation_arr_ty))
        .unwrap();

    let mut michelson_instructions: Vec<MWrappedInstr> = vec![
        instruction_row!(MInstr::Comment("Construct a operation list {".to_string())),
        instruction_row!(
            MInstr::Nil { ty: MTy::Operation },
            format!("(nil operation) : storage : ...")
        ),
        instruction_row!(
            MInstr::DupN(register2stack_ptr.len() + pair_memory_ptr + 2),
            format!("pair_memory : (nil operation) : storage : ...",)
        ),
        instruction_row!(MInstr::Car),
        instruction_row!(MInstr::DupN(register2stack_ptr.get(reg).unwrap() + 3)),
        instruction_row!(MInstr::Get),
        instruction_row!(
            MInstr::AssertSome,
            format!("pair_map_instance : (nil operation) : storage : ...")
        ),
        instruction_row!(MInstr::Push {
            ty: MTy::Int,
            val: MVal::Int(0),
        }), //FIXME? NOTE: '0'番目に[size x operation]が入っている事を決め打ち
        instruction_row!(MInstr::Get),
        instruction_row!(MInstr::AssertSome), // [size x operation]* : (nil operation) : storage : ...
        instruction_row!(MInstr::DupN(
            register2stack_ptr.len() + operation_arr_memory_ptr + 3
        )),
        instruction_row!(MInstr::Car),
        instruction_row!(MInstr::Swap),
        instruction_row!(MInstr::Get),
        instruction_row!(MInstr::AssertSome), // ([size x operation] MAP instance) : (nil operation) : storage : ...
    ];

    let size = *match operation_arr_ty {
        Type::Array {
            size,
            elementtype: _,
        } => size,
        _ => panic!(),
    };

    let operation_memory_ptr = memory_ty2stack_ptr
        .get(&InnerType::from(&Type::Operation))
        .unwrap();

    // input: ([size x operation] MAP instance) : (list operation) : encoded_storage :[register]:[memory]
    //output: ([size x operation] MAP instance) : (list operation) : encoded_storage :[register]:[memory]
    for idx in 0..size {
        michelson_instructions.append(
            &mut vec![
                MInstr::Dup,
                MInstr::Push {
                    ty: MTy::Int,
                    val: MVal::Int(idx.try_into().unwrap()),
                },
                MInstr::Get,
                MInstr::AssertSome, // ptr : map-instance
                MInstr::DupN(register2stack_ptr.len() + operation_memory_ptr + 4),
                MInstr::Car, // operation_memory : ptr : map-instance
                MInstr::Swap,
                MInstr::Get,
                MInstr::AssertSome, // option operation : map-instance
                MInstr::IfNone {
                    instr1: vec![],
                    instr2: vec![MInstr::DigN(2), MInstr::Swap, MInstr::Cons, MInstr::DugN(1)]
                        .iter()
                        .map(|instr| instr.to_wrapped_instruction())
                        .collect::<Vec<_>>(),
                },
            ]
            .iter()
            .map(|instr| instr.to_wrapped_instruction())
            .collect::<Vec<_>>(),
        );
    }

    michelson_instructions.append(
        &mut vec![MInstr::Drop, MInstr::Pair, MInstr::Comment("}".to_string())]
            .iter()
            .map(|instr| instr.to_wrapped_instruction())
            .collect::<Vec<_>>(),
    );
    michelson_instructions
}

///レジスタ領域とメモリ領域をDROPする
pub fn exit(
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
) -> Vec<MWrappedInstr> {
    let mut instructions = vec![
        instruction_row!(MInstr::Comment("###################################".to_string())),
        instruction_row!(MInstr::Comment("############### Exit ##############".to_string())),
        instruction_row!(MInstr::Comment("##################################{".to_string())),
    ];

    instructions.push(instruction_row!(
        MInstr::DugN(register2stack_ptr.len() + memory_ty2stack_ptr.len()),
        format!("move a (list operation, storage) to the stack bottom")
    ));
    //後処理:レジスタ領域・メモリ領域をDROPする
    for _ in 0..(register2stack_ptr.iter().len() + memory_ty2stack_ptr.iter().len()) {
        instructions.push(instruction_row!(MInstr::Drop));
    }

    instructions.push(instruction_row!(MInstr::Comment("}##################################".to_string())));

    instructions
}
