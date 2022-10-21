mod helper;
use crate::compiler::utils;
use crate::mini_llvm::{
    reserved_type2michelson_pair, Arg, Condition, Function, Instruction, Opcode, Register, Type,
};
use std::collections::HashMap;

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
        };
    }
}

///ここが終わった段階ではMichelson StackのTopに(Parameter, Storage)が乗っている
pub fn prepare_storage(
    smart_contract_function: &Function,
    michelson_code: String,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    let storage_arg = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(ty) {
            Type::Struct { id, fields: _ } => id == String::from("Storage"),
            _ => false,
        })
        .unwrap();
    format!(
        "{michelson_code}{}",
        helper::storage::alloca_storage_by_value(
            storage_arg,
            tab,
            tab_depth,
            &register2stack_ptr,
            &memory_ty2stack_ptr,
        )
    )
}

///ここが終わった段階では(Parameter, Strorage)はもう要らないのでDROP.
pub fn prepare_parameter(
    smart_contract_function: &Function,
    michelson_code: String,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    let parameter_arg = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(ty) {
            Type::Struct { id, fields: _ } => id == String::from("Parameter"),
            _ => false,
        })
        .unwrap();
    format!(
        "{michelson_code}{}",
        helper::parameter::alloca_parameter_by_value(
            parameter_arg,
            tab,
            tab_depth,
            &register2stack_ptr,
            &memory_ty2stack_ptr,
        )
    )
}

///スマートコントラクトの返り値として使うPairをAllocaする関数
///（ここでAllocaしたPairをエンコードしてコントラクトの返り値とする）
pub fn prepare_pair(
    smart_contract_function: &Function,
    michelson_code: String,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    let pair_arg = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(ty) {
            Type::Struct { id, fields: _ } => id == String::from("Pair"),
            _ => false,
        })
        .unwrap();
    format!(
        "{michelson_code}{}",
        helper::alloca::exec_alloca(
            &pair_arg.reg,
            &Type::deref(&pair_arg.ty),
            tab,
            tab_depth,
            &register2stack_ptr,
            &memory_ty2stack_ptr,
        )
    )
}

///Step.1
///ここではmichelson_codeを受け取り、実際にMichelsonの命令を追加していく.
///レジスタ型環境（register2ty, register2stack_ptr）とメモリ型環境（memory_ty2stack_ptr）
///を受け取り,それらに相当するMichelson命令をスタックにPUSHする
pub fn prepare(
    michelson_code: String,
    space: &str,
    register2stack_ptr: &HashMap<Register, usize>,
    register2ty: &HashMap<Register, Type>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    let mut new_michelson_code = michelson_code;

    let mut memory_ty2stack_ptr_sorted = memory_ty2stack_ptr.iter().collect::<Vec<_>>();
    memory_ty2stack_ptr_sorted.sort_by(|a, b| (b.1).cmp(a.1));
    for (ty, _v) in memory_ty2stack_ptr_sorted.iter() {
        let ty_str = Type::to_michelson_ty_string(&ty);

        let llvm_ty_string = Type::to_llvm_ty_string(ty);
        let comment = format!("memory for {llvm_ty_string}");

        new_michelson_code = format!("{new_michelson_code}{space}PUSH int 0;\n");
        new_michelson_code =
            format!("{new_michelson_code}{space}EMPTY_MAP int {ty_str}; # {comment}\n");
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
    //(param, storage)を一番上に持ってくる
    new_michelson_code = format!(
        "{new_michelson_code}{space}DIG {};\n",
        register2stack_ptr.len() + memory_ty2stack_ptr.len()
    );
    //new_michelson_code = format!("{new_michelson_code}{space}DROP;\n");
    new_michelson_code
}

///Step.2
///LLVMの命令列instructionsを実際にコンパイルしていく関数
///レジスタ型環境（register2ty（これは今回は無し）, register2stack_ptr）と
///メモリ型環境（memory_ty2stack_ptr）を参考にコンパイルしていく.
///tab,tab_depthはMichelsonコードのフォーマットのために使う
pub fn body(
    michelson_code: String,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    register2ty: &HashMap<Register, Type>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
    instructions: &Vec<Instruction>,
) -> String {
    let mut michelson_code = michelson_code;
    for instruction in instructions {
        match instruction {
            Instruction::Alloca { ptr, ty } => {
                michelson_code = format!(
                    "{michelson_code}{}",
                    helper::alloca::exec_alloca(
                        ptr,
                        ty,
                        tab,
                        tab_depth,
                        register2stack_ptr,
                        memory_ty2stack_ptr
                    )
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
                    register2ty,
                    memory_ty2stack_ptr,
                    code_block_t,
                );
                let michelson_code_block_f = body(
                    String::new(),
                    tab,
                    tab_depth + 1,
                    register2stack_ptr,
                    register2ty,
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
                    register2ty,
                    memory_ty2stack_ptr,
                    cond_block,
                );

                // FIXME: インデントを揃えるために上とほぼ同じものを生成している
                let michelson_cond_block_used_in_loop = body(
                    String::new(),
                    tab,
                    tab_depth + 1,
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    cond_block,
                );

                let michelson_loop_block = body(
                    String::new(),
                    tab,
                    tab_depth + 1,
                    register2stack_ptr,
                    register2ty,
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
            Instruction::LlvmMemcpy { dest, src, ty } => {
                michelson_code = format!(
                    "{michelson_code}{}",
                    helper::llvm_memcpy::exec_llvm_memcpy(
                        dest,
                        src,
                        ty,
                        tab,
                        tab_depth,
                        register2stack_ptr,
                        register2ty,
                        memory_ty2stack_ptr
                    )
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

/// input:                 [register]:[memory]
///output: encoded_storage:[register]:[memory]
pub fn retrieve_storage_from_memory(
    smart_contract_function: &Function,
    michelson_code: String,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
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
    let storage_memory_ptr = memory_ty2stack_ptr.get(&storage_ty).unwrap();
    let pair_memory_ptr = memory_ty2stack_ptr.get(&Type::deref(pair_ty_ptr)).unwrap();
    let mut michelson_instructions = vec![];
    michelson_instructions.append(&mut vec![
        format!("### encode Storage {{"),
        format!("DUP {};", register2stack_ptr.len() + pair_memory_ptr),
        format!("CAR;"),
        format!("PUSH int {};", register2stack_ptr.get(reg).unwrap()),
        format!("GET;"),
        format!("ASSERT_SOME; # {}", "Pair MAP Instance"),
        format!("PUSH int {};", 1), // StorageのIndex(=1)
        format!("GET;"),
        format!("ASSERT_SOME; # {}", "Storage Ptr"),
        format!("DUP {};", register2stack_ptr.len() + storage_memory_ptr + 1),
        format!("CAR;"),
        format!("SWAP;"),
        format!("GET;"),
        format!("ASSERT_SOME; # {}", "Storage MAP Instance"),
    ]);

    match storage_ty {
        Type::Struct { id, fields } => {
            if fields.len() >= 2 {
                //逆順にスタックにencodeしたものを積んでいき、最後にPAIR nまとめる.
                for (field_idx, field) in fields.iter().enumerate().rev() {
                    michelson_instructions.append(&mut retrieve_storage_field_from_memory(
                        field_idx,
                        field,
                        fields.len() - field_idx,
                        register2stack_ptr,
                        memory_ty2stack_ptr,
                    ));
                }
                michelson_instructions
                    .push(format!("PAIR {}; # PACK Struct {{ {id} }}", fields.len()));
            } else if fields.len() == 1 {
                todo!()
            } else {
                michelson_instructions.push(format!("DROP;"));
                michelson_instructions.push(format!("UNIT;"));
            }
        }
        _ => {
            panic!("StorageがStruct型ではなくPrimitive型になっています.")
        }
    }
    michelson_instructions.push(format!("### }}"));

    format!(
        "{michelson_code}{}",
        utils::format(&michelson_instructions, tab, tab_depth)
    )
}

///FIXME:
/// input:          [es_idx+1]:..:[es_n]:[storage map instance]:[register]:[memory]
///output: [es_idx]:[es_idx+1]:..:[es_n]:[register]:[memory]
fn retrieve_storage_field_from_memory(
    field_idx: usize,
    field: &Type,
    depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> Vec<String> {
    match field {
        Type::Struct {
            id: _,
            fields: child_fields,
        } => {
            let mut michelson_instructions = vec![];
            for (child_field_idx, child_field) in child_fields.iter().enumerate().rev() {
                michelson_instructions.append(&mut retrieve_storage_field_from_memory(
                    child_field_idx,
                    child_field,
                    depth + child_fields.len() - child_field_idx - 1,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                ));
            }
            michelson_instructions
        }
        _ => {
            let memory_ptr = memory_ty2stack_ptr.get(&field).unwrap();
            vec![
                format!("DUP {} ;", depth),
                format!("PUSH int {};", field_idx),
                format!("GET;"),
                format!("ASSERT_SOME;"),
                format!(
                    "DUP {} # memory: {};",
                    register2stack_ptr.len() + memory_ptr + depth,
                    Type::to_llvm_ty_string(field)
                ),
                format!("CAR;"),
                format!("SWAP;"),
                format!("GET;"),
                format!("ASSERT_SOME;"),
            ]
        }
    }
}

///Step.3(将来的にはこの関数はなくなるかもしれない)
///レジスタ型環境（register2ty（これは今回は無し）, register2stack_ptr）と
///メモリ型環境（memory_ty2stack_ptr）に相当するMichelsonスタックをDROPする
pub fn exit(
    michelson_code: String,
    space: &str,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
    structure_types: &Vec<Type>,
) -> String {
    let mut new_michelson_code = michelson_code;
    new_michelson_code = format!(
        "{new_michelson_code}{space}DUG {}; # {}\n",
        register2stack_ptr.len() + memory_ty2stack_ptr.len(),
        "move a storage to the stack bottom"
    );
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

    //TODO: operationがハードコードされている。ここを直したい
    new_michelson_code = format!("{new_michelson_code}{space}NIL operation; PAIR;");

    let parameter_michelson_ty = reserved_type2michelson_pair(
        structure_types
            .iter()
            .find(|ty| match ty {
                Type::Struct { id, fields: _ } => id == &String::from("Parameter"),
                _ => false,
            })
            .expect("Parameter型が宣言されていません.")
            .clone(),
    );
    let storage_michelson_ty = reserved_type2michelson_pair(
        structure_types
            .iter()
            .find(|ty| match ty {
                Type::Struct { id, fields: _ } => id == &String::from("Storage"),
                _ => false,
            })
            .expect("Storage型が宣言されていません.")
            .clone(),
    );

    format!("parameter {parameter_michelson_ty};\nstorage {storage_michelson_ty};\ncode {{\n{new_michelson_code} }}")
}
