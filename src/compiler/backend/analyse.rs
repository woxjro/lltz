//! MiniLlvmの中をコンパイル前に事前に走査し, 出てきうるレジスタ, メモリの数や型などを
//! 洗い出しておくといった事前分析を担当するモジュール
use super::helper;
use crate::mini_llvm::{Arg, Instruction, Register, Type};
use std::collections::HashMap;

/// 構造体宣言を事前に走査し, 必要なメモリの型を洗い出しておく関数
pub fn analyse_structure_types(
    memory_ty2stack_ptr: &mut HashMap<Type, usize>,
    memory_ptr: &mut usize,
    structure_types: &Vec<Type>,
) {
    for structure_type in structure_types {
        match memory_ty2stack_ptr.get(&structure_type) {
            //既にtyが登録されていたらexit
            Some(_) => {}
            _ => {
                match structure_type {
                    Type::Struct { id: _, fields } => {
                        for field in fields {
                            helper::analyse::analyse_memory4alloca(
                                field.clone(),
                                memory_ty2stack_ptr,
                                memory_ptr,
                            );
                        }
                        //登録する
                        let _ = memory_ty2stack_ptr
                            .entry(structure_type.clone())
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

/// （主にsmart_contract関数の）MiniLlvmのargument_listを受け取り, その中に出てくる
/// レジスタなどを洗い出しておく関数
pub fn analyse_argument_list(
    register2stack_ptr: &mut HashMap<Register, usize>,
    register2ty: &mut HashMap<Register, Type>,
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

        register2ty.entry(reg.clone()).or_insert(ty.clone());
    }
}

///Step.0
///まず与えられたLLVM IRの命令列（instructions）を事前に走査して
///命令列に出現しうる型やレジスタの種類・数などを把握する
///つまり、レジスタ型環境（register2ty, register2stack_ptr）と
///メモリ型環境（memory_ty2stack_ptr）の可変参照を受け取っておき、これらを構築する
pub fn analyse_registers_and_memory(
    register2stack_ptr: &mut HashMap<Register, usize>,
    register2ty: &mut HashMap<Register, Type>,
    memory_ty2stack_ptr: &mut HashMap<Type, usize>,
    stack_ptr: &mut usize,
    memory_ptr: &mut usize,
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
                    .or_insert(Type::Ptr(Box::new(ty.clone())));

                //（レジスタは上記で良いんだけど、）Struct型の場合は内部にも, メモリの型を
                // 保持している（ケースがほとんどである）ので再帰的に調べる必要がある
                helper::analyse::analyse_memory4alloca(ty.clone(), memory_ty2stack_ptr, memory_ptr);
            }
            Instruction::Store { ty, value, ptr } => {
                let _ = register2stack_ptr.entry(value.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                //即値を仮想レジスタとして扱うのでこの処理は（少なくとも今は）必要
                //NOTE: ptrはType::Ptr(ty)のポインタ型であることに注意
                register2ty
                    .entry(ptr.clone())
                    .or_insert(Type::Ptr(Box::new(ty.clone())));

                //即値を仮想レジスタとして扱うのでこの処理は（少なくとも今は）必要
                register2ty.entry(value.clone()).or_insert(ty.clone());
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
                    .or_insert(Type::Ptr(Box::new(ty.clone())));

                register2ty.entry(result.clone()).or_insert(ty.clone());

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
                //result: Register
                //ty: Type,
                //ptrval: Register,
                //subsequent: Vec<(Type, Register)>,
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
                    .or_insert(Type::Ptr(Box::new(ty.clone())));

                // FIXME: elementに対するpointer型であって, Struct*ではない...
                // しかしそれを知るすべがない. constをregisterに入れてしまっているため...
                // => とりあえず、SubsequentにはConstしか来ないと仮定してParse
                match ty {
                    Type::Struct { id: _, fields } => {
                        let idx = subsequent[1].1.get_id().parse::<usize>().unwrap();
                        let t = fields.iter().nth(idx).unwrap();
                        register2ty
                            .entry(result.clone())
                            .or_insert(Type::Ptr(Box::new(t.clone())));
                    }
                    _ => {
                        panic!("Primitive型に対してGetElementPtrは使えません.")
                    }
                }

                for (ty, reg) in subsequent {
                    let _ = register2stack_ptr.entry(reg.clone()).or_insert_with(|| {
                        *stack_ptr += 1;
                        *stack_ptr
                    });
                    register2ty.entry(reg.clone()).or_insert(ty.clone());
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
                register2ty.entry(reg.clone()).or_insert(Type::Bool);
                analyse_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    stack_ptr,
                    memory_ptr,
                    code_block_t,
                );
                analyse_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    stack_ptr,
                    memory_ptr,
                    code_block_f,
                );
            }
            Instruction::While {
                cond: _,
                cond_block,
                loop_block,
            } => {
                analyse_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    stack_ptr,
                    memory_ptr,
                    cond_block,
                );
                analyse_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    stack_ptr,
                    memory_ptr,
                    loop_block,
                );
            }
            Instruction::Call { .. } => {
                todo!()
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
                let _ = register2stack_ptr.entry(op1.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                let _ = register2stack_ptr.entry(op2.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                register2ty.entry(op1.clone()).or_insert(ty.clone());
                register2ty.entry(op2.clone()).or_insert(ty.clone());
                register2ty.entry(result.clone()).or_insert(ty.clone());
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
            Instruction::Ret { ty, value } => {
                let _ = register2stack_ptr.entry(value.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                register2ty.entry(value.clone()).or_insert(ty.clone());
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
                register2ty.entry(result.clone()).or_insert(ty.clone());
                let _ = register2stack_ptr.entry(op1.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                let _ = register2stack_ptr.entry(op2.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
            }
            Instruction::MichelsonGetAmount { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty.entry(result.clone()).or_insert(Type::Mutez);

                let _ = memory_ty2stack_ptr.entry(Type::Mutez).or_insert_with(|| {
                    *memory_ptr += 1;
                    *memory_ptr
                });
            }
            Instruction::MichelsonGetBalance { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty.entry(result.clone()).or_insert(Type::Mutez);

                let _ = memory_ty2stack_ptr.entry(Type::Mutez).or_insert_with(|| {
                    *memory_ptr += 1;
                    *memory_ptr
                });
            }
            Instruction::MichelsonGetTotalVotingPower { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty.entry(result.clone()).or_insert(Type::Nat);

                let _ = memory_ty2stack_ptr.entry(Type::Nat).or_insert_with(|| {
                    *memory_ptr += 1;
                    *memory_ptr
                });
            }
            Instruction::MichelsonGetLevel { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty.entry(result.clone()).or_insert(Type::Nat);

                let _ = memory_ty2stack_ptr.entry(Type::Nat).or_insert_with(|| {
                    *memory_ptr += 1;
                    *memory_ptr
                });
            }
            Instruction::MichelsonGetSender { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty.entry(result.clone()).or_insert(Type::Address);

                let _ = memory_ty2stack_ptr.entry(Type::Address).or_insert_with(|| {
                    *memory_ptr += 1;
                    *memory_ptr
                });
            }
            Instruction::MichelsonGetSource { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty.entry(result.clone()).or_insert(Type::Address);

                let _ = memory_ty2stack_ptr.entry(Type::Address).or_insert_with(|| {
                    *memory_ptr += 1;
                    *memory_ptr
                });
            }
            Instruction::MichelsonGetSelfAddress { result } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                register2ty.entry(result.clone()).or_insert(Type::Address);

                let _ = memory_ty2stack_ptr.entry(Type::Address).or_insert_with(|| {
                    *memory_ptr += 1;
                    *memory_ptr
                });
            }
        };
    }
}
