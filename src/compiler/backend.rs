//! 以下のコンパイルフローにおけるLLTZ IRからMichelsonへのコンパイル（Backend）を担当するモジュール
//! LLVM IR ---> LLTZ IR ---> Michelson

mod helper;
mod inject;
mod scan;
use crate::compiler::utils;
use crate::lltz_ir::{
    Arg, BackendType, Condition, Function, Instruction, Opcode, Register, Type, Value,
};
use michelson_ast::formatter;
use michelson_ast::instruction::Instruction as MInstr;
use michelson_ast::instruction_wrapper::InstructionWrapper as MInstrWrapper;
use michelson_ast::ty::Ty as MTy;
use michelson_ast::val::Val as MVal;
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
    memory_ty2stack_ptr: &mut HashMap<BackendType, usize>,
    register2ty: &mut HashMap<Register, BackendType>,
) {
    scan::scan_structure_types(memory_ty2stack_ptr, memory_ptr, &structure_types);

    scan::scan_argument_list(register2stack_ptr, register2ty, stack_ptr, &argument_list);

    scan::scan_registers_and_memory(
        register2stack_ptr,
        register2ty,
        memory_ty2stack_ptr,
        stack_ptr,
        memory_ptr,
        &structure_types,
        &instructions,
    );
}

///（主に）Programの`smart_contract_function`を受け取り，そのargument_listである
///スマートコントラクト引数をメモリ領域に格納する．
pub fn inject_argument_list(
    smart_contract_function: &Function,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
) -> Vec<MInstrWrapper> {
    let mut res = vec![];
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

    res
}

///michelson_codeを受け取り、レジスタ領域とメモリ領域を構築するMichelson命令を発行する．
///レジスタ型環境（register2ty, register2stack_ptr）とメモリ型環境（memory_ty2stack_ptr）
///を受け取り,それらに相当するMichelson命令をスタックにPUSHする
/// before:                               (storage, parameter)
/// after:  (storage, parameter):[register area]:[memory area]
pub fn stack_initialization(
    michelson_code: String,
    tab: &str,
    register2stack_ptr: &HashMap<Register, usize>,
    register2ty: &HashMap<Register, BackendType>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
) -> String {
    let mut michelson_instructions = vec![];
    let memory_ty2stack_ptr = memory_ty2stack_ptr.clone();
    let mut memory_ty2stack_ptr_sorted = memory_ty2stack_ptr
        .iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<Vec<_>>();
    memory_ty2stack_ptr_sorted.sort_by(|a, b| (a.1).cmp(&b.1));
    for (ty, _v) in memory_ty2stack_ptr_sorted.iter().rev() {
        let ty_str = ty.to_memory_ty().to_string();

        let llvm_ty_string = ty.get_name();
        let comment = format!("memory for {llvm_ty_string}");

        michelson_instructions.append(&mut vec![
            format!("PUSH int 0;"),
            format!("EMPTY_MAP int {ty_str}; # {comment}"),
            format!("PAIR;"),
        ]);
    }

    let mut register2stack_ptr_sorted = register2stack_ptr.iter().collect::<Vec<_>>();
    register2stack_ptr_sorted.sort_by(|a, b| (b.1).cmp(a.1));

    for (reg, _ptr) in register2stack_ptr_sorted {
        let ty = register2ty.get(reg).unwrap();
        let val = if Register::is_const(reg) {
            //reg.parse::<i32>().unwrap()
            reg.get_id()
        } else {
            BackendType::default_value(&ty)
        };
        let michelson_ty = ty.to_memory_ty().to_string();
        let llvm_ty_string = ty.get_name();

        let comment = if Register::is_const(reg) {
            let val = if val.len() >= 6 {
                let sval = &val[1..5];
                format!("{sval}..")
            } else {
                val.clone()
            };
            format!("for const {val} : {llvm_ty_string}")
        } else {
            let id = reg.get_id();
            format!("for reg {id} : {llvm_ty_string}")
        };
        michelson_instructions.push(match ty {
            BackendType::Operation => format!("{val}; # {comment}"),
            BackendType::Contract(_) => format!("{val}; # {comment}"),
            BackendType::Option(inner) => {
                if Register::is_const(reg) {
                    let michelson_ty = inner.to_memory_ty().to_string();
                    format!("PUSH {michelson_ty} {val}; SOME; # {comment}")
                } else {
                    format!("{val}; # {comment}")
                }
            }
            _ => format!("PUSH {michelson_ty} {val}; # {comment}"),
        });
    }
    //(param, storage)を一番上に持ってくる
    michelson_instructions.push(format!(
        "DIG {};",
        register2stack_ptr.len() + memory_ty2stack_ptr.len()
    ));
    format!(
        "{michelson_code}{}",
        utils::format(&michelson_instructions, tab, 1)
    )
}

///LLTZ IRの命令列instructionsを受け取り，それらの挙動をエミュレートする
///Michelson コードを発行する関数．
///レジスタ型環境（register2ty（これは今回は無し）, register2stack_ptr）と
///メモリ型環境（memory_ty2stack_ptr）を参考にコンパイルしていく.
///tab,tab_depthはMichelsonコードのフォーマットのために使う
pub fn compile_instructions(
    register2stack_ptr: &HashMap<Register, usize>,
    register2ty: &HashMap<Register, BackendType>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
    instructions: &Vec<Instruction>,
) -> Vec<MInstrWrapper> {
    let mut res = vec![];
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
                let memory_ptr = memory_ty2stack_ptr.get(&BackendType::from(ty)).unwrap();

                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "store {} {}, {}* {} {{",
                        Type::get_name(ty),
                        value.to_string(),
                        Type::get_name(ty),
                        ptr.get_id()
                    ))],
                    vec![
                        match value {
                            Value::Register(register) => {
                                MInstr::DupN(*register2stack_ptr.get(&register).unwrap())
                            }
                            Value::Const(cnst) => cnst.get_push_instruction(),
                        },
                        MInstr::Some,
                        MInstr::DigN(register2stack_ptr.len() + memory_ptr),
                        MInstr::Unpair,
                        MInstr::DigN(2),
                        MInstr::DupN(*register2stack_ptr.get(&ptr).unwrap() + 3),
                        MInstr::Update,
                        MInstr::Pair,
                        MInstr::DugN(register2stack_ptr.len() + memory_ptr - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::Load { result, ty, ptr } => {
                let memory_ptr = memory_ty2stack_ptr.get(&BackendType::from(ty)).unwrap();

                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "{} = load {}, {}* {} {{",
                        result.get_id(),
                        Type::get_name(ty),
                        Type::get_name(ty),
                        ptr.get_id()
                    ))],
                    vec![
                        MInstr::DupN(register2stack_ptr.len() + memory_ptr),
                        MInstr::Car,
                        MInstr::DupN(register2stack_ptr.get(&ptr).unwrap() + 1),
                        MInstr::Get,
                        MInstr::AssertSome,
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(*register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::GetElementPtr {
                result,
                ty,
                ptrval,
                subsequent,
            } => {
                let memory_ptr = memory_ty2stack_ptr.get(&BackendType::from(ty)).unwrap();

                // FIXME TODO: subsequent[1]で決め打ちで取得しているので直したい.
                //              (...が, これ以外無い気がする)
                let (_, reg) = &subsequent[1];

                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "{} = getElementPtr {}, {}*, {} {{",
                        result.get_id(),
                        Type::get_name(ty),
                        Type::get_name(ty),
                        ptrval.get_id()
                    ))],
                    vec![
                        MInstr::DupN(register2stack_ptr.len() + memory_ptr),
                        MInstr::Car, // bm
                        MInstr::DupN(register2stack_ptr.get(&ptrval).unwrap() + 1),
                        MInstr::Get,        //some(map)
                        MInstr::AssertSome, //map
                        MInstr::DupN(register2stack_ptr.get(&reg).unwrap() + 1), //int:map
                        MInstr::Get,
                        MInstr::AssertSome, //ptr
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
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
                    MInstrWrapper::Comment(format!("if {{",)),
                    MInstr::DupN(*register2stack_ptr.get(&reg).unwrap()).to_instruction_wrapper(),
                    MInstr::If { instr1, instr2 }.to_instruction_wrapper(),
                    MInstrWrapper::Comment("}".to_string()),
                ];

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

                let mut instr: Vec<MInstrWrapper> = vec![];
                instr.append(&mut loop_instr.clone());
                instr.append(&mut cond_instr.clone());
                instr.push(
                    MInstr::DupN(*register2stack_ptr.get(&cond).unwrap()).to_instruction_wrapper(),
                );

                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!("while {{",))],
                    cond_instr.clone(),
                    vec![
                        MInstr::DupN(*register2stack_ptr.get(&cond).unwrap())
                            .to_instruction_wrapper(),
                        MInstr::Loop { instr }.to_instruction_wrapper(),
                    ],
                    vec![MInstrWrapper::Comment("}".to_string())],
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
                    vec![MInstrWrapper::Comment(format!(
                        "{} = {} {} {} {} {{",
                        result.get_id(),
                        opcode.to_string(),
                        Type::get_name(ty),
                        op1.get_id(),
                        op2.get_id(),
                    ))],
                    vec![
                        MInstr::DupN(*register2stack_ptr.get(&op2).unwrap()),
                        MInstr::DupN(*register2stack_ptr.get(&op1).unwrap() + 1),
                        match opcode {
                            Opcode::Add => MInstr::Add,
                            Opcode::Sub => MInstr::Sub,
                            Opcode::Mul => MInstr::Mul,
                        },
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(*register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
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
                    vec![MInstrWrapper::Comment(format!(
                        "{} = icmp {} {} {{", //TODO: icmp -> cond.to_string()
                        result.get_id(),
                        op1.get_id(),
                        op2.get_id(),
                    ))],
                    vec![
                        MInstr::DupN(*register2stack_ptr.get(&op1).unwrap()),
                        MInstr::DupN(register2stack_ptr.get(&op2).unwrap() + 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
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
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetAmount { result } => {
                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "{} = MichelsonGetAmount {{",
                        result.get_id()
                    ))],
                    vec![
                        MInstr::Amount,
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(*register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetBalance { result } => {
                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "{} = MichelsonGetBalance {{",
                        result.get_id()
                    ))],
                    vec![
                        MInstr::Balance,
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(*register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetTotalVotingPower { result } => {
                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "{} = MichelsonGetTotalVotingPower {{",
                        result.get_id()
                    ))],
                    vec![
                        MInstr::TotalVotingPower,
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(*register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetLevel { result } => {
                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "{} = MichelsonGetLevel {{",
                        result.get_id()
                    ))],
                    vec![
                        MInstr::Level,
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(*register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetSender { result } => {
                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "{} = MichelsonGetSender {{",
                        result.get_id()
                    ))],
                    vec![
                        MInstr::Sender,
                        MInstr::Some, // to (option address)
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(*register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetSource { result } => {
                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "{} = MichelsonGetSource {{",
                        result.get_id()
                    ))],
                    vec![
                        MInstr::Source,
                        MInstr::Some, // to (option address)
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(*register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetSelfAddress { result } => {
                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "{} = MichelsonGetSelfAddress {{",
                        result.get_id()
                    ))],
                    vec![
                        MInstr::SelfAddress,
                        MInstr::Some, // to (option address)
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(*register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonGetSelf { result } => {
                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "{} = MichelsonGetSelf {{",
                        result.get_id()
                    ))],
                    vec![
                        MInstr::Slf,
                        MInstr::Some, // to (option contract <ty>)
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(*register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonContract {
                result,
                ty,
                address,
            } => {
                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "{} = MichelsonContract {{",
                        result.get_id()
                    ))],
                    vec![
                        MInstr::DupN(*register2stack_ptr.get(&address).unwrap()),
                        MInstr::AssertSome,
                        MInstr::Contract {
                            ty: ty.to_entrypoint_ty(),
                        },
                        MInstr::Some,
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
            Instruction::MichelsonAssertSome { result, ty, value } => {
                let mut instructions = vec![
                    vec![MInstrWrapper::Comment(format!(
                        "{} = MichelsonAssertSome {} {} {{",
                        result.get_id(),
                        BackendType::from(ty).to_michelson_ty().to_string(),
                        value.get_id()
                    ))],
                    vec![
                        MInstr::DupN(*register2stack_ptr.get(&value).unwrap()),
                        MInstr::AssertSome,
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(*register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
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
                    vec![MInstrWrapper::Comment(format!(
                        "{} = MichelsonTransferTokens {} {} {} {{",
                        result.get_id(),
                        init.get_id(),
                        tokens.get_id(),
                        contract.get_id()
                    ))],
                    vec![
                        MInstr::DupN(*register2stack_ptr.get(&contract).unwrap()),
                        MInstr::AssertSome,
                        MInstr::DupN(register2stack_ptr.get(&tokens).unwrap() + 1),
                        MInstr::Unit, // FIXME: unit しか対応していない...
                        MInstr::TransferTokens,
                        MInstr::Some,
                        MInstr::DigN(*register2stack_ptr.get(&result).unwrap()),
                        MInstr::Drop,
                        MInstr::DugN(register2stack_ptr.get(&result).unwrap() - 1),
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
                    vec![MInstrWrapper::Comment("}".to_string())],
                ]
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();

                res.append(&mut instructions);
            }
        };
    }
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
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
) -> Vec<MInstrWrapper> {
    let Arg {
        reg,
        ty: pair_ty_ptr,
    } = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(ty) {
            Type::Struct { id, fields: _ } => id == String::from("Pair"),
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
            Type::Struct { id, fields: _ } => id == String::from("Storage"),
            _ => false,
        })
        .unwrap();

    let storage_ty = Type::deref(storage_ty_ptr);
    let storage_memory_ptr = memory_ty2stack_ptr
        .get(&BackendType::from(&storage_ty))
        .unwrap();
    let pair_memory_ptr = memory_ty2stack_ptr
        .get(&BackendType::deref(&BackendType::from(pair_ty_ptr)))
        .unwrap();
    let mut michelson_instructions = vec![];
    michelson_instructions.append(&mut vec![
        MInstrWrapper::Comment(format!("encode Storage {{")),
        MInstr::DupN(register2stack_ptr.len() + pair_memory_ptr).to_instruction_wrapper(),
        MInstr::Car.to_instruction_wrapper(),
        MInstr::Push {
            ty: MTy::Int,
            val: MVal::Int((*register2stack_ptr.get(reg).unwrap()).try_into().unwrap()),
        }
        .to_instruction_wrapper(),
        MInstr::Get.to_instruction_wrapper(),
        MInstr::AssertSome.to_instruction_wrapper(),
        MInstr::Push {
            ty: MTy::Int,
            val: MVal::Int(1),
        }
        .to_instruction_wrapper(), // StorageのIndex(=1)
        MInstr::Get.to_instruction_wrapper(),
        MInstr::AssertSome.to_instruction_wrapper(), //Storage Ptr
        MInstr::DupN(register2stack_ptr.len() + storage_memory_ptr + 1).to_instruction_wrapper(),
        MInstr::Car.to_instruction_wrapper(),
        MInstr::Swap.to_instruction_wrapper(),
        MInstr::Get.to_instruction_wrapper(),
        MInstr::AssertSome.to_instruction_wrapper(), //Storage MAP Instance
    ]);

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
                    MInstr::PairN(fields.len())
                        .to_instruction_wrapper_with_comment(&format!("PACK Struct {{ {id} }}")),
                    MInstr::Swap.to_instruction_wrapper(),
                    MInstr::Drop
                        .to_instruction_wrapper_with_comment(&format!("Storage MAP Instance")),
                ]);
            } else if fields.len() == 1 {
                todo!()
            } else {
                michelson_instructions.push(MInstr::Drop.to_instruction_wrapper());
                michelson_instructions.push(MInstr::Unit.to_instruction_wrapper());
            }
        }
        _ => {
            panic!("StorageがStruct型ではなくPrimitive型になっています.")
        }
    }
    michelson_instructions.push(MInstrWrapper::Comment("}".to_string()));

    michelson_instructions
}

///FIXME: 方針はあっているが少しややこしい.
fn retrieve_storage_field_from_memory(
    field_idx: usize,
    field: &Type,
    path: Vec<usize>,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
) -> Vec<MInstrWrapper> {
    let memory_ptr = memory_ty2stack_ptr.get(&BackendType::from(field)).unwrap();
    match field {
        Type::Struct {
            id: child_id,
            fields: child_fields,
        } => {
            //TODO: child_fields.len() > 2, == 1, == 0で場合分け
            let mut michelson_instructions: Vec<MInstrWrapper> = vec![
                MInstrWrapper::Comment("{".to_string()),
                MInstr::DupN(path[path.len() - 1])
                    .to_instruction_wrapper_with_comment("MAP instance"),
                MInstr::Push {
                    ty: MTy::Int,
                    val: MVal::Int(field_idx.try_into().unwrap()),
                }
                .to_instruction_wrapper(),
                MInstr::Get.to_instruction_wrapper(),
                MInstr::AssertSome.to_instruction_wrapper(),
                MInstr::DupN(
                    register2stack_ptr.len() + memory_ptr + path.iter().sum::<usize>() + 1,
                )
                .to_instruction_wrapper_with_comment(&format!("memory: {}", Type::get_name(field))),
                MInstr::Car.to_instruction_wrapper(),
                MInstr::Swap.to_instruction_wrapper(),
                MInstr::Get.to_instruction_wrapper(),
                MInstr::AssertSome.to_instruction_wrapper(),
            ];
            for (child_field_idx, child_field) in child_fields.iter().enumerate().rev() {
                let new_path =
                    vec![path.clone(), vec![child_fields.len() - child_field_idx]].concat();
                michelson_instructions.append(&mut retrieve_storage_field_from_memory(
                    child_field_idx,
                    child_field,
                    new_path,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                ));
            }
            michelson_instructions.append(&mut vec![
                MInstr::PairN(child_fields.len())
                    .to_instruction_wrapper_with_comment(&format!("PACK Struct {{ {child_id} }}")),
                MInstr::Swap.to_instruction_wrapper(),
                MInstr::Drop
                    .to_instruction_wrapper_with_comment(&format!("child field MAP Instance")),
                MInstrWrapper::Comment("}".to_string()),
            ]);

            michelson_instructions
        }
        _ => {
            vec![
                MInstrWrapper::Comment("{".to_string()),
                MInstr::DupN(path[path.len() - 1])
                    .to_instruction_wrapper_with_comment("MAP instance"),
                MInstr::Push {
                    ty: MTy::Int,
                    val: MVal::Int(field_idx.try_into().unwrap()),
                }
                .to_instruction_wrapper(),
                MInstr::Get.to_instruction_wrapper(),
                MInstr::AssertSome.to_instruction_wrapper(),
                MInstr::DupN(
                    register2stack_ptr.len() + memory_ptr + path.iter().sum::<usize>() + 1,
                )
                .to_instruction_wrapper_with_comment(&format!("memory: {}", Type::get_name(field))),
                MInstr::Car.to_instruction_wrapper(),
                MInstr::Swap.to_instruction_wrapper(),
                MInstr::Get.to_instruction_wrapper(),
                MInstr::AssertSome.to_instruction_wrapper(),
                MInstrWrapper::Comment("}".to_string()),
            ]
        }
    }
}

/// input:                     encoded_storage :[register]:[memory]
///output:  ([list operation], encoded_storage):[register]:[memory]
pub fn retrieve_operations_from_memory(
    smart_contract_function: &Function,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
) -> Vec<MInstrWrapper> {
    let Arg {
        reg,
        ty: pair_ty_ptr,
    } = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(ty) {
            Type::Struct { id, fields: _ } => id == String::from("Pair"),
            _ => false,
        })
        .unwrap();

    let pair_memory_ptr = memory_ty2stack_ptr
        .get(&BackendType::from(&Type::deref(&pair_ty_ptr)))
        .unwrap();

    let _pair_fields = match Type::deref(&pair_ty_ptr) {
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
        .get(&BackendType::from(operation_arr_ty))
        .unwrap();

    let mut michelson_instructions: Vec<MInstrWrapper> = vec![
        MInstrWrapper::Comment("retrieve operations from memory {".to_string()),
        MInstr::Nil { ty: MTy::Operation }.to_instruction_wrapper(), //(nil operation) : storage : ...
        MInstr::DupN(register2stack_ptr.len() + pair_memory_ptr + 2)
            .to_instruction_wrapper_with_comment("pair_memory : (nil operation) : storage : ..."),
        MInstr::Car.to_instruction_wrapper(),
        MInstr::DupN(register2stack_ptr.get(reg).unwrap() + 3).to_instruction_wrapper(),
        MInstr::Get.to_instruction_wrapper(),
        MInstr::AssertSome.to_instruction_wrapper(), // pair_map_instance : (nil operation) : storage : ...
        MInstr::Push {
            ty: MTy::Int,
            val: MVal::Int(0),
        }
        .to_instruction_wrapper(), //FIXME: '0'番目に[size x operation]が入っている事を決め打ち
        MInstr::Get.to_instruction_wrapper(),
        MInstr::AssertSome.to_instruction_wrapper(), // [size x operation]* : (nil operation) : storage : ...
        MInstr::DupN(register2stack_ptr.len() + operation_arr_memory_ptr + 3)
            .to_instruction_wrapper(),
        MInstr::Car.to_instruction_wrapper(),
        MInstr::Swap.to_instruction_wrapper(),
        MInstr::Get.to_instruction_wrapper(),
        MInstr::AssertSome.to_instruction_wrapper(), // ([size x operation] MAP instance) : (nil operation) : storage : ...
    ];

    let size = *match operation_arr_ty {
        Type::Array {
            size,
            elementtype: _,
        } => size,
        _ => panic!(),
    };

    let operation_memory_ptr = memory_ty2stack_ptr
        .get(&BackendType::from(&Type::Operation))
        .unwrap();

    // input: ([size x operation] MAP instance) : (list operation) : encoded_storage :[register]:[memory]
    //output: ([size x operation] MAP instance) : (list operation) : encoded_storage :[register]:[memory]
    for idx in 0..size {
        michelson_instructions.append(&mut vec![
            MInstr::Dup.to_instruction_wrapper(),
            MInstr::Push {
                ty: MTy::Int,
                val: MVal::Int(idx.try_into().unwrap()),
            }
            .to_instruction_wrapper(),
            MInstr::Get.to_instruction_wrapper(),
            MInstr::AssertSome.to_instruction_wrapper(), // ptr : map-instance
            MInstr::DupN(register2stack_ptr.len() + operation_memory_ptr + 4)
                .to_instruction_wrapper(),
            MInstr::Car.to_instruction_wrapper(), // operation_memory : ptr : map-instance
            MInstr::Swap.to_instruction_wrapper(),
            MInstr::Get.to_instruction_wrapper(),
            MInstr::AssertSome.to_instruction_wrapper(), // option operation : map-instance
            MInstr::IfNone {
                instr1: vec![],
                instr2: vec![MInstr::DigN(2), MInstr::Swap, MInstr::Cons, MInstr::DugN(1)]
                    .iter()
                    .map(|instr| instr.to_instruction_wrapper())
                    .collect::<Vec<_>>(),
            }
            .to_instruction_wrapper(),
        ]);
    }

    michelson_instructions.append(&mut vec![
        MInstr::Drop.to_instruction_wrapper(),
        MInstr::Pair.to_instruction_wrapper(),
        MInstrWrapper::Comment("}".to_string()),
    ]);
    michelson_instructions
}

///(将来的にはこの関数はなくなるかもしれない)
///レジスタ型環境（register2ty（これは今回は無し）, register2stack_ptr）と
///メモリ型環境（memory_ty2stack_ptr）に相当するMichelsonスタックをDROPする
pub fn exit(
    michelson_code: String,
    tab: &str,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
    structure_types: &Vec<Type>,
) -> String {
    let mut instructions = vec![];
    instructions.push(
        MInstr::DugN(register2stack_ptr.len() + memory_ty2stack_ptr.len())
            .to_instruction_wrapper_with_comment(
                "move a (list operation, storage) to the stack bottom",
            ),
    );
    //後処理:レジスタ領域・メモリ領域をDROPする
    for _ in 0..(register2stack_ptr.iter().len() + memory_ty2stack_ptr.iter().len()) {
        instructions.push(MInstr::Drop.to_instruction_wrapper());
    }

    let parameter_michelson_ty = structure_types
        .iter()
        .find(|ty| match ty {
            Type::Struct { id, fields: _ } => id == &String::from("Parameter"),
            _ => false,
        })
        .expect("Parameter型が宣言されていません.")
        .to_entrypoint_ty()
        .to_string();
    let storage_michelson_ty = structure_types
        .iter()
        .find(|ty| match ty {
            Type::Struct { id, fields: _ } => id == &String::from("Storage"),
            _ => false,
        })
        .expect("Storage型が宣言されていません.")
        .to_entrypoint_ty()
        .to_string();

    format!(
        "
parameter {parameter_michelson_ty};
storage {storage_michelson_ty};
code {{
{}
     }}
",
        format!(
            "{michelson_code}{}\n",
            formatter::format(&instructions, 1, tab)
        )
    )
}
