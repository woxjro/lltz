mod helper;
use crate::compiler::utils;
use crate::mini_llvm::{Condition, Instruction, Opcode, Register, Type};
use std::collections::HashMap;
//まず与えられたLLVM IRの命令列を事前に走査して
//命令列に出現しうる型やレジスタの種類・数などを把握する
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
                helper::alloca_memory(ty.clone(), memory_ty2stack_ptr, memory_ptr);
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
                register2ty
                    .entry(result.clone())
                    .or_insert(Type::Ptr(Box::new(ty.clone())));

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
                register2ty.entry(reg.clone()).or_insert(Type::I1);
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
        };
    }
}

//レジスタ・メモリ領域などに相当するMichelsonコードをスタックにPUSHする
pub fn prepare(
    michelson_code: String,
    space: &str,
    register2stack_ptr: &mut HashMap<Register, usize>,
    register2ty: &mut HashMap<Register, Type>,
    memory_ty2stack_ptr: &mut HashMap<Type, usize>,
) -> String {
    let mut new_michelson_code = format!("{michelson_code}{space}DROP;\n");

    let mut memory_ty2stack_ptr_sorted = memory_ty2stack_ptr.iter().collect::<Vec<_>>();
    memory_ty2stack_ptr_sorted.sort_by(|a, b| (b.1).cmp(a.1));
    for (ty, _v) in memory_ty2stack_ptr_sorted.iter() {
        let ty_str = Type::to_michelson_ty_string(&ty);

        let llvm_ty_string = Type::to_llvm_ty_string(ty);
        let comment = format!("memory for {llvm_ty_string}");

        new_michelson_code = format!("{new_michelson_code}{space}PUSH int 0;\n");
        new_michelson_code =
            format!("{new_michelson_code}{space}EMPTY_BIG_MAP int {ty_str}; # {comment}\n");
        new_michelson_code = format!("{new_michelson_code}{space}PAIR;\n");
    }

    let mut register2stack_ptr_sorted = register2stack_ptr.iter().collect::<Vec<_>>();
    register2stack_ptr_sorted.sort_by(|a, b| (b.1).cmp(a.1));

    for (reg, _ptr) in register2stack_ptr_sorted {
        let ty = register2ty.get(reg).unwrap();
        let val = match ty {
            Type::I32 => {
                if Register::is_const(reg) {
                    //reg.parse::<i32>().unwrap()
                    reg.get_id()
                } else {
                    //0
                    "0".to_string()
                }
            }
            Type::I1 => "False".to_string(),
            // TODO FIXME: llvm struct to michelson type
            // 多分ここは実行されることはない.?? = レジスタにStructが入ることはない.?
            Type::Struct { id: _, fields: _ } => "0".to_string(),
            Type::Ptr(_) => {
                if Register::is_const(reg) {
                    //即値をレジスタとして扱っているのでidは数値となる
                    //reg.parse::<i32>().unwrap()
                    reg.get_id()
                } else {
                    //0
                    "0".to_string()
                }
            }
        };
        let michelson_ty = Type::to_michelson_ty_string(&ty);
        let llvm_ty_string = Type::to_llvm_ty_string(ty);

        let comment = if Register::is_const(reg) {
            format!("for const {val} : {llvm_ty_string}")
        } else {
            let id = reg.get_id();
            format!("for reg {id} : {llvm_ty_string}")
        };
        new_michelson_code =
            format!("{new_michelson_code}{space}PUSH {michelson_ty} {val}; # {comment}\n");
    }
    new_michelson_code
}

pub fn body(
    michelson_code: String,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &mut HashMap<Register, usize>,
    memory_ty2stack_ptr: &mut HashMap<Type, usize>,
    instructions: &Vec<Instruction>,
) -> String {
    let mut michelson_code = michelson_code;
    for instruction in instructions {
        match instruction {
            Instruction::Alloca { ptr, ty } => {
                let memory_ptr = memory_ty2stack_ptr.get(ty).unwrap();

                let michelson_instructions = match ty {
                    Type::Struct { id: _, fields } => {
                        let mut res = vec![format!("###alloca {{"), format!("EMPTY_MAP int int;")];
                        for (i, field) in fields.iter().enumerate() {
                            // FIXME TODO: structの中にstructや配列などといったものが
                            // 入れ子になっている際の処理の場合分け.
                            // とりあえずStruct型の中がプリミティブ型のみであると仮定して実装
                            let field_memory_ptr = memory_ty2stack_ptr.get(field).unwrap();
                            let mut instructions = vec![
                                //field tyのallocaみたいな事をする
                                format!("DIG {};", register2stack_ptr.len() + field_memory_ptr),
                                format!("UNPAIR;"), //bm:ptr:map
                                format!("SWAP;"),
                                format!("PUSH int 1;"),
                                format!("ADD;"),
                                format!("DUP;"),
                                format!("DUP;"),   //ptr:ptr:ptr:bm:map
                                format!("DIG 3;"), //bm:ptr:ptr:ptr:map
                                format!("SWAP;"),  //ptr:bm:ptr:ptr:map
                                format!("PUSH int -1;"),
                                format!("SOME;"),
                                format!("SWAP;"),   //ptr:some(-1):bm:ptr:ptr:map
                                format!("UPDATE;"), //bm:ptr:ptr:map
                                format!("PAIR;"),   //(bm, ptr):ptr:map
                                format!("DUG {};", register2stack_ptr.len() + field_memory_ptr + 1),
                                format!("SOME;"),         //some(ptr):map
                                format!("PUSH int {i};"), //idx:some(ptr):map
                                format!("UPDATE;"),       //map
                            ];

                            res.append(&mut instructions);
                        }

                        res.append(&mut vec![
                            format!("SOME;"), //some(map)
                            format!("DIG {};", register2stack_ptr.len() + memory_ptr),
                            format!("UNPAIR;"), //bm:ptr:some(map)
                            format!("SWAP;"),   //ptr:bm:some(map)
                            format!("PUSH int 1;"),
                            format!("ADD;"),
                            format!("DUP;"),
                            format!("DUP;"),   //ptr:ptr:ptr:bm:some(map)
                            format!("DIG 3;"), //bm:ptr:ptr:ptr:some(map)
                            format!("DIG 4;"), //some(map):bm:ptr:ptr:ptr
                            format!("DIG 2;"),
                            format!("UPDATE;"), //bm:ptr:ptr
                            format!("PAIR;"),   //(bm,ptr):ptr
                            format!("DUG {};", register2stack_ptr.len() + memory_ptr),
                            format!("DIG {};", register2stack_ptr.get(&ptr).unwrap()),
                            format!("DROP;"),
                            format!("DUG {};", register2stack_ptr.get(&ptr).unwrap() - 1),
                            format!("###}}"),
                        ]);

                        res
                    }
                    _ => {
                        vec![
                            format!("###alloca {{"),
                            format!("DIG {};", register2stack_ptr.len() + memory_ptr - 1),
                            format!("UNPAIR;"),
                            format!("SWAP;"),
                            format!("PUSH int 1;"),
                            format!("ADD;"),
                            format!("DUP;"),
                            format!("DUP;"),
                            format!("DIG 3;"),
                            format!("SWAP;"),
                            format!("PUSH int -1; # default value"),
                            format!("SOME;"),
                            format!("SWAP;"),
                            format!("UPDATE;"),
                            format!("PAIR;"),
                            format!("DUG {};", register2stack_ptr.len() + memory_ptr),
                            format!("DIG {};", register2stack_ptr.get(&ptr).unwrap()),
                            format!("DROP;"),
                            format!("DUG {};", register2stack_ptr.get(&ptr).unwrap() - 1),
                            format!("###}}"),
                        ]
                    }
                };
                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(&michelson_instructions, tab, tab_depth)
                );
            }
            Instruction::Store { ty, value, ptr } => {
                let memory_ptr = memory_ty2stack_ptr.get(ty).unwrap();

                let michelson_instructions = vec![
                    format!("###store {{"),
                    format!("DUP {};", register2stack_ptr.get(&value).unwrap()),
                    format!("SOME;"),
                    format!("DIG {};", register2stack_ptr.len() + memory_ptr),
                    format!("UNPAIR;"),
                    format!("DIG 2;"),
                    format!("DUP {};", register2stack_ptr.get(&ptr).unwrap() + 3),
                    format!("UPDATE;"),
                    format!("PAIR;"),
                    format!("DUG {};", register2stack_ptr.len() + memory_ptr - 1),
                    format!("###}}"),
                ];
                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(&michelson_instructions, tab, tab_depth)
                );
            }
            Instruction::Load { result, ty, ptr } => {
                let memory_ptr = memory_ty2stack_ptr.get(ty).unwrap();

                let michelson_instructions = vec![
                    format!("###load {{"),
                    format!("DUP {};", register2stack_ptr.len() + memory_ptr),
                    format!("CAR;"),
                    format!("DUP {};", register2stack_ptr.get(&ptr).unwrap() + 1),
                    format!("GET;"),
                    format!("ASSERT_SOME;"),
                    format!("DIG {};", register2stack_ptr.get(&result).unwrap()),
                    format!("DROP;"),
                    format!("DUG {};", register2stack_ptr.get(&result).unwrap() - 1),
                    format!("###}}"),
                ];
                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(&michelson_instructions, tab, tab_depth)
                );
            }
            Instruction::GetElementPtr {
                result,
                ty,
                ptrval,
                subsequent,
            } => {
                let memory_ptr = memory_ty2stack_ptr.get(ty).unwrap();
                // FIXME TODO: subsequent[1]で決め打ちで取得しているので直したい.
                //              (...が, これ以外無い気がする)
                let (_, reg) = &subsequent[1];
                let michelson_instructions = vec![
                    format!("###getElementPtr {{"),
                    format!("DUP {};", register2stack_ptr.len() + memory_ptr),
                    format!("CAR;"), //bm
                    format!("DUP {};", register2stack_ptr.get(&ptrval).unwrap() + 1),
                    format!("GET;"),         //some(map)
                    format!("ASSERT_SOME;"), //map
                    format!("DUP {};", register2stack_ptr.get(&reg).unwrap() + 1), //int:map
                    format!("GET;"),
                    format!("ASSERT_SOME;"), //ptr
                    format!("DIG {};", register2stack_ptr.get(&result).unwrap()),
                    format!("DROP;"),
                    format!("DUG {};", register2stack_ptr.get(&result).unwrap() - 1),
                    format!("###}}"),
                ];
                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(&michelson_instructions, tab, tab_depth)
                );
            }
            Instruction::If {
                reg,
                code_block_t,
                code_block_f,
            } => {
                michelson_code = format!("{michelson_code}{tab}###If {{\n");
                michelson_code = format!(
                    "{michelson_code}{tab}DUP {};\n",
                    register2stack_ptr.get(&reg).unwrap()
                );
                let michelson_code_block_t = body(
                    String::new(),
                    tab,
                    tab_depth + 1,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                    code_block_t,
                );
                let michelson_code_block_f = body(
                    String::new(),
                    tab,
                    tab_depth + 1,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                    code_block_f,
                );

                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(&vec![format!("IF {{"),], tab, tab_depth)
                );

                michelson_code = format!("{michelson_code}{michelson_code_block_t}");
                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(&vec![format!("   }}"), format!("   {{"),], tab, tab_depth),
                );

                michelson_code = format!("{michelson_code}{michelson_code_block_f}");
                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(&vec![format!("   }};"), format!("###}}"),], tab, tab_depth),
                );
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
                let michelson_cond_block = body(
                    String::new(),
                    tab,
                    tab_depth,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                    cond_block,
                );

                // FIXME: インデントを揃えるために上とほぼ同じものを生成している
                let michelson_cond_block_used_in_loop = body(
                    String::new(),
                    tab,
                    tab_depth + 1,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                    cond_block,
                );

                let michelson_loop_block = body(
                    String::new(),
                    tab,
                    tab_depth + 1,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                    loop_block,
                );

                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(&vec![format!("###While {{")], tab, tab_depth)
                );
                michelson_code = format!("{michelson_code}{}", michelson_cond_block);

                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(
                        &vec![
                            format!("DUP {};", register2stack_ptr.get(&cond).unwrap()),
                            format!("LOOP {{"),
                        ],
                        tab,
                        tab_depth
                    )
                );

                michelson_code = format!("{michelson_code}{}", michelson_loop_block);
                michelson_code = format!("{michelson_code}{}", michelson_cond_block_used_in_loop);

                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(
                        &vec![format!("DUP {};", register2stack_ptr.get(&cond).unwrap())],
                        tab,
                        tab_depth + 1
                    )
                );

                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(&vec![format!("     }};"), format!("###}}")], tab, tab_depth)
                );

                //];
            }
            Instruction::Call { .. } => {
                todo!()
            }
            Instruction::Op {
                ty: _,
                opcode,
                result,
                op1,
                op2,
            } => {
                let op = match opcode {
                    Opcode::Add => "ADD",
                    Opcode::Sub => "SUB",
                    Opcode::Mul => "MUL",
                };

                let michelson_instructions = vec![
                    //NOTE: 意図的にop2を先にDUPしている(LLVMとの被演算子の順番を揃えるため)
                    format!("###Op {{"),
                    format!("DUP {};", register2stack_ptr.get(&op2).unwrap()),
                    format!("DUP {};", register2stack_ptr.get(&op1).unwrap() + 1),
                    format!("{op};"),
                    format!("DIG {};", register2stack_ptr.get(&result).unwrap()),
                    format!("DROP;"),
                    format!("DUG {};", register2stack_ptr.get(&result).unwrap() - 1),
                    format!("###}}"),
                ];
                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(&michelson_instructions, tab, tab_depth)
                );
            }
            Instruction::Ret { ty: _, value: _ } => {}
            Instruction::Icmp {
                result,
                cond,
                ty: _,
                op1,
                op2,
            } => {
                let mut michelson_instructions = vec![
                    format!("###Icmp {{"),
                    format!("DUP {};", register2stack_ptr.get(&op1).unwrap()),
                    format!("DUP {};", register2stack_ptr.get(&op2).unwrap() + 1),
                ];

                let mut op = vec![];
                // TODO: 他のConditionについても実装
                match cond {
                    Condition::Eq => {
                        op.push(format!("COMPARE;"));
                        op.push(format!("EQ;"));
                    }
                    Condition::Slt => {
                        op.push(format!("SUB;"));
                        op.push(format!("GT;"));
                    }
                    _ => {
                        op.push(format!("COMPARE;"));
                    }
                };

                let mut rest = vec![
                    format!("DIG {};", register2stack_ptr.get(&result).unwrap()),
                    format!("DROP;"),
                    format!("DUG {};", register2stack_ptr.get(&result).unwrap() - 1),
                    format!("###}}"),
                ];

                michelson_instructions.append(&mut op);
                michelson_instructions.append(&mut rest);

                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(&michelson_instructions, tab, tab_depth)
                );
            }
        };
    }
    michelson_code
}

pub fn exit(
    michelson_code: String,
    space: &str,
    register2stack_ptr: &mut HashMap<Register, usize>,
    memory_ty2stack_ptr: &mut HashMap<Type, usize>,
) -> String {
    let mut new_michelson_code = michelson_code;
    //後処理:レジスタ領域・メモリ領域をDROPする
    for i in 0..(register2stack_ptr.iter().len() + memory_ty2stack_ptr.iter().len()) {
        if i % 5 == 0 {
            new_michelson_code = format!("{new_michelson_code}{space}DROP;");
        } else if i % 5 == 4 {
            new_michelson_code = format!("{new_michelson_code}DROP;\n");
        } else {
            new_michelson_code = format!("{new_michelson_code}DROP;");
        }
    }
    new_michelson_code = format!("{new_michelson_code}\n");
    new_michelson_code = format!("{new_michelson_code}{space}UNIT; NIL operation; PAIR;");

    format!("parameter unit;\nstorage unit;\ncode {{\n{new_michelson_code} }}")
}
