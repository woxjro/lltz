//! Programの中をコンパイル前に事前に走査し, 出てきうるレジスタ, メモリの数や型などを
//! 洗い出しておくといった事前分析を担当するモジュール
use super::helper;
use crate::lltz_ir::{Arg, BackendType, Const, Instruction, Register, Type, Value};
use std::collections::HashMap;

/// 構造体宣言を事前に走査し, 必要なメモリの型を洗い出しておく関数
pub fn scan_structure_types(
    memory_ty2stack_ptr: &mut HashMap<BackendType, usize>,
    memory_ptr: &mut usize,
    structure_types: &Vec<Type>,
) {
    for structure_type in structure_types {
        match memory_ty2stack_ptr.get(&BackendType::from(structure_type)) {
            //既にtyが登録されていたらexit
            Some(_) => {}
            _ => {
                match structure_type {
                    Type::Struct { id: _, fields } => {
                        for field in fields {
                            helper::scan::scan_memory4alloca(
                                field.clone(),
                                memory_ty2stack_ptr,
                                memory_ptr,
                            );
                        }
                        //登録する
                        let _ = memory_ty2stack_ptr
                            .entry(BackendType::from(structure_type))
                            .or_insert_with(|| {
                                *memory_ptr += 1;
                                *memory_ptr
                            });
                    }
                    _ => {
                        panic!("Struct型宣言にPrimitive型が混ざっています")
                    }
                }
            }
        };
    }
}

/// （主にsmart_contract関数の）Programのargument_listを受け取り, その中に出てくる
/// レジスタなどを洗い出しておく関数
pub fn scan_argument_list(
    register2stack_ptr: &mut HashMap<Register, usize>,
    register2ty: &mut HashMap<Register, BackendType>,
    stack_ptr: &mut usize,
    argument_list: &Vec<Arg>,
) {
    //vec![
    //    Arg {
    //        ty: Type::Ptr(Box::new(pair.clone())),
    //        reg: Register::new("%0"),
    //    },
    //    Arg {
    //        ty: Type::Ptr(Box::new(parameter.clone())),
    //        reg: Register::new("%1"),
    //    },
    //    Arg {
    //        ty: Type::Ptr(Box::new(storage.clone())),
    //        reg: Register::new("%2"),
    //    },
    //]
    for Arg { ty, reg } in argument_list {
        let _ = register2stack_ptr.entry(reg.clone()).or_insert_with(|| {
            *stack_ptr += 1;
            *stack_ptr
        });

        register2ty
            .entry(reg.clone())
            .or_insert(BackendType::from(ty));
    }
}

///Step.0
///まず与えられたLLTZ IRの命令列（instructions）を事前に走査して
///命令列に出現しうる型やレジスタの種類・数などを把握する
///つまり、レジスタ型環境（register2ty, register2stack_ptr）と
///メモリ型環境（memory_ty2stack_ptr）の可変参照を受け取っておき、これらを構築する
pub fn scan_registers_and_memory(
    register2stack_ptr: &mut HashMap<Register, usize>,
    register2ty: &mut HashMap<Register, BackendType>,
    memory_ty2stack_ptr: &mut HashMap<BackendType, usize>,
    stack_ptr: &mut usize,
    memory_ptr: &mut usize,
    structure_types: &Vec<Type>,
    instructions: &Vec<Instruction>,
) {
    for instruction in instructions {
        match instruction {
            Instruction::Alloca { ptr, ty } => {
                let _ = register2stack_ptr.entry(ptr.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                //NOTE: ptrはType::Ptr(ty)のポインタ型であることに注意
                register2ty
                    .entry(ptr.clone())
                    .or_insert(BackendType::from(&Type::Ptr(Box::new(ty.clone()))));

                //（レジスタは上記で良いんだけど、）Struct型の場合は内部にも, メモリの型を
                // 保持している（ケースがほとんどである）ので再帰的に調べる必要がある
                helper::scan::scan_memory4alloca(ty.clone(), memory_ty2stack_ptr, memory_ptr);
            }
            Instruction::Store { ty, value, ptr } => {
                match value {
                    Value::Register(register) => {
                        let _ = register2stack_ptr
                            .entry(register.clone())
                            .or_insert_with(|| {
                                *stack_ptr += 1;
                                *stack_ptr
                            });
                        register2ty
                            .entry(register.clone())
                            .or_insert(BackendType::from(ty));
                    }
                    Value::Const(_) => {
                        //nothing to do
                    }
                };

                //NOTE: ptrはType::Ptr(ty)のポインタ型であることに注意
                register2ty
                    .entry(ptr.clone())
                    .or_insert(BackendType::from(&Type::Ptr(Box::new(ty.clone()))));

                let _ = register2stack_ptr.entry(ptr.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
            }
            Instruction::Load { result, ty, ptr } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                //NOTE: ptrはType::Ptr(ty)のポインタ型であることに注意
                register2ty
                    .entry(ptr.clone())
                    .or_insert(BackendType::from(&Type::Ptr(Box::new(ty.clone()))));

                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::from(ty));

                let _ = register2stack_ptr.entry(ptr.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
            }
            Instruction::GetElementPtr {
                result,
                ty,
                ptrval,
                subsequent,
            } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                let _ = register2stack_ptr.entry(ptrval.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                //NOTE: ptrはType::Ptr(ty)のポインタ型であることに注意
                register2ty
                    .entry(ptrval.clone())
                    .or_insert(BackendType::from(&Type::Ptr(Box::new(ty.clone()))));

                // FIXME: elementに対するpointer型であって, Struct*ではない...
                // しかしそれを知るすべがない. constをregisterに入れてしまっているため...
                // => とりあえず、SubsequentにはConstしか来ないと仮定してParse
                match ty {
                    Type::Struct { id: _, fields } => {
                        // 構造体に対するGEPのindexはconstしか来ない．
                        match &subsequent[1].1 {
                            Value::Const(cnst) => {
                                let idx = match cnst {
                                    Const::Int(i) => (*i).try_into().unwrap(),
                                    _ => panic!(
                                        "GetElementPtr の idx に int 型以外の型は使えません．"
                                    ),
                                };
                                let ty = fields.iter().nth(idx).unwrap();
                                register2ty
                                    .entry(result.clone())
                                    .or_insert(BackendType::from(&Type::Ptr(Box::new(ty.clone()))));
                            }
                            Value::Register(_) => {
                                panic!("GetElementPtr の idx に register は使えません．")
                            }
                        };
                    }
                    Type::Array {
                        size: _,
                        elementtype,
                    } => {
                        // 配列に対するGEPのindexはconst, registerの両方がありえる
                        register2ty
                            .entry(result.clone())
                            .or_insert(BackendType::from(&Type::Ptr(elementtype.clone())));
                        let _ = memory_ty2stack_ptr
                            .entry(BackendType::from(elementtype))
                            .or_insert_with(|| {
                                *memory_ptr += 1;
                                *memory_ptr
                            });
                    }
                    _ => {
                        panic!("Primitive型に対してGetElementPtrは使えません.")
                    }
                }

                for (ty, value) in subsequent {
                    match value {
                        Value::Register(register) => {
                            let _ =
                                register2stack_ptr
                                    .entry(register.clone())
                                    .or_insert_with(|| {
                                        *stack_ptr += 1;
                                        *stack_ptr
                                    });
                            register2ty
                                .entry(register.clone())
                                .or_insert(BackendType::from(ty));
                        }
                        Value::Const(_) => {
                            //nothing to do
                        }
                    }
                }
            }
            Instruction::If {
                reg,
                code_block_t,
                code_block_f,
            } => {
                let _ = register2stack_ptr.entry(reg.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                register2ty.entry(reg.clone()).or_insert(BackendType::Bool);
                scan_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    stack_ptr,
                    memory_ptr,
                    structure_types,
                    code_block_t,
                );
                scan_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    stack_ptr,
                    memory_ptr,
                    structure_types,
                    code_block_f,
                );
            }
            Instruction::While {
                cond: _,
                cond_block,
                loop_block,
            } => {
                scan_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    stack_ptr,
                    memory_ptr,
                    structure_types,
                    cond_block,
                );
                scan_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    stack_ptr,
                    memory_ptr,
                    structure_types,
                    loop_block,
                );
            }
            Instruction::Op {
                ty,
                opcode: _,
                result,
                op1,
                op2,
            } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::from(ty));

                match op1 {
                    Value::Register(register) => {
                        let _ = register2stack_ptr
                            .entry(register.clone())
                            .or_insert_with(|| {
                                *stack_ptr += 1;
                                *stack_ptr
                            });
                        register2ty
                            .entry(register.clone())
                            .or_insert(BackendType::from(ty));
                    }
                    Value::Const(_) => {
                        //nothing to do
                    }
                };

                match op2 {
                    Value::Register(register) => {
                        let _ = register2stack_ptr
                            .entry(register.clone())
                            .or_insert_with(|| {
                                *stack_ptr += 1;
                                *stack_ptr
                            });

                        register2ty
                            .entry(register.clone())
                            .or_insert(BackendType::from(ty));
                    }
                    Value::Const(_) => {
                        //nothing to do
                    }
                };
            }
            Instruction::LlvmMemcpy {
                dest: _,
                src: _,
                ty: _,
            } => {
                //dest, srcともに既にレジスタ環境に無ければ行けない事を考えると
                //これらの処理はいらないかもしれない
                /*
                let _ = register2stack_ptr.entry(dest.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                let _ = register2stack_ptr.entry(src.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                */
            }
            Instruction::Icmp {
                result,
                cond: _,
                ty,
                op1,
                op2,
            } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::Bool);

                match op1 {
                    Value::Register(register) => {
                        let _ = register2stack_ptr
                            .entry(register.clone())
                            .or_insert_with(|| {
                                *stack_ptr += 1;
                                *stack_ptr
                            });
                        register2ty
                            .entry(register.clone())
                            .or_insert(BackendType::from(ty));
                    }
                    Value::Const(_) => {
                        //nothing to do
                    }
                };

                match op2 {
                    Value::Register(register) => {
                        let _ = register2stack_ptr
                            .entry(register.clone())
                            .or_insert_with(|| {
                                *stack_ptr += 1;
                                *stack_ptr
                            });
                        register2ty
                            .entry(register.clone())
                            .or_insert(BackendType::from(ty));
                    }
                    Value::Const(_) => {
                        //nothing to do
                    }
                };
            }
            Instruction::MichelsonGetAmount { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::Mutez);

                let _ = memory_ty2stack_ptr
                    .entry(BackendType::Mutez)
                    .or_insert_with(|| {
                        *memory_ptr += 1;
                        *memory_ptr
                    });
            }
            Instruction::MichelsonGetBalance { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::Mutez);

                let _ = memory_ty2stack_ptr
                    .entry(BackendType::Mutez)
                    .or_insert_with(|| {
                        *memory_ptr += 1;
                        *memory_ptr
                    });
            }
            Instruction::MichelsonGetTotalVotingPower { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::Nat);

                let _ = memory_ty2stack_ptr
                    .entry(BackendType::Nat)
                    .or_insert_with(|| {
                        *memory_ptr += 1;
                        *memory_ptr
                    });
            }
            Instruction::MichelsonGetLevel { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::Nat);

                let _ = memory_ty2stack_ptr
                    .entry(BackendType::Nat)
                    .or_insert_with(|| {
                        *memory_ptr += 1;
                        *memory_ptr
                    });
            }
            Instruction::MichelsonGetSender { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::from(&Type::Address));

                let _ = memory_ty2stack_ptr
                    .entry(BackendType::from(&Type::Address))
                    .or_insert_with(|| {
                        *memory_ptr += 1;
                        *memory_ptr
                    });
            }
            Instruction::MichelsonGetSource { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::from(&Type::Address));

                let _ = memory_ty2stack_ptr
                    .entry(BackendType::from(&Type::Address))
                    .or_insert_with(|| {
                        *memory_ptr += 1;
                        *memory_ptr
                    });
            }
            Instruction::MichelsonGetSelfAddress { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::from(&Type::Address));

                let _ = memory_ty2stack_ptr
                    .entry(BackendType::from(&Type::Address))
                    .or_insert_with(|| {
                        *memory_ptr += 1;
                        *memory_ptr
                    });
            }
            Instruction::MichelsonGetSelf { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                let parameter = structure_types
                    .iter()
                    .find(|ty| match ty {
                        Type::Struct { id, fields: _ } => id == &String::from("Parameter"),
                        _ => false,
                    })
                    .unwrap();
                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::from(&Type::Contract(Box::new(
                        parameter.clone(),
                    ))));

                let _ = memory_ty2stack_ptr
                    .entry(BackendType::from(&Type::Contract(Box::new(
                        parameter.clone(),
                    ))))
                    .or_insert_with(|| {
                        *memory_ptr += 1;
                        *memory_ptr
                    });
            }
            Instruction::MichelsonContract {
                result,
                ty,
                address,
            } => {
                let _ = register2stack_ptr
                    .entry(address.clone())
                    .or_insert_with(|| {
                        *stack_ptr += 1;
                        *stack_ptr
                    });

                register2ty
                    .entry(address.clone())
                    .or_insert(BackendType::Address);

                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::from(&Type::Option(Box::new(Type::Contract(
                        Box::new(ty.clone()),
                    )))));
            }
            Instruction::MichelsonAssertSome { result, ty, value } => match ty {
                Type::Option(child_ty) => {
                    let _ = register2stack_ptr.entry(value.clone()).or_insert_with(|| {
                        *stack_ptr += 1;
                        *stack_ptr
                    });

                    register2ty
                        .entry(value.clone())
                        .or_insert(BackendType::from(ty));

                    let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                        *stack_ptr += 1;
                        *stack_ptr
                    });

                    register2ty
                        .entry(result.clone())
                        .or_insert(BackendType::from(child_ty));
                }
                _ => panic!("Option型以外にはASSERT_SOMEは使えません"),
            },
            Instruction::MichelsonTransferTokens {
                result,
                init: _,
                tokens,
                contract: _,
            } => {
                let _ = register2stack_ptr.entry(tokens.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty
                    .entry(tokens.clone())
                    .or_insert(BackendType::from(&Type::Mutez));

                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty
                    .entry(result.clone())
                    .or_insert(BackendType::from(&Type::Operation));
            }
        };
    }
}
