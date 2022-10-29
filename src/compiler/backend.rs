//! 以下のコンパイルフローにおけるLLVM IR'からMichelsonへのコンパイル（Backend）を担当するモジュール
//! LLVM IR ---> LLVM IR' ---> Michelson

mod analyse;
mod helper;
mod prepare;
use crate::compiler::utils;
use crate::mini_llvm::{
    reserved_type2michelson_pair, Arg, Condition, Function, Instruction, Opcode, Register, Type,
};
use std::collections::HashMap;

///MiniLlvmの構造体宣言,引数リスト,命令列を受け取り,それらに現れるレジスタ、メモリや型
///などを調べる.
pub fn analyse(
    structure_types: &Vec<Type>,
    argument_list: &Vec<Arg>,
    instructions: &Vec<Instruction>,
    stack_ptr: &mut usize,
    register2stack_ptr: &mut HashMap<Register, usize>,
    memory_ptr: &mut usize,
    memory_ty2stack_ptr: &mut HashMap<Type, usize>,
    register2ty: &mut HashMap<Register, Type>,
) {
    analyse::analyse_structure_types(memory_ty2stack_ptr, memory_ptr, &structure_types);

    analyse::analyse_argument_list(register2stack_ptr, register2ty, stack_ptr, &argument_list);

    analyse::analyse_registers_and_memory(
        register2stack_ptr,
        register2ty,
        memory_ty2stack_ptr,
        stack_ptr,
        memory_ptr,
        &instructions,
    );
}

///（主に）MiniLlvmの`smart_contract_function`を受け取りそのargument_listである引数について
///Allocaに相当する事をしたり, Michelson引数であるStorage, Parameterなどの値を挿入したりする
pub fn prepare_from_argument_list(
    smart_contract_function: &Function,
    michelson_code: String,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    let mut michelson_code = michelson_code;
    michelson_code = prepare::prepare_storage(
        smart_contract_function,
        michelson_code,
        tab,
        tab_depth,
        register2stack_ptr,
        memory_ty2stack_ptr,
    );

    michelson_code = prepare::prepare_parameter(
        smart_contract_function,
        michelson_code,
        tab,
        tab_depth,
        register2stack_ptr,
        memory_ty2stack_ptr,
    );

    michelson_code = prepare::prepare_pair(
        smart_contract_function,
        michelson_code,
        tab,
        tab_depth,
        register2stack_ptr,
        memory_ty2stack_ptr,
    );

    michelson_code
}

///ここではmichelson_codeを受け取り、実際にMichelsonの命令を追加していく.
///レジスタ型環境（register2ty, register2stack_ptr）とメモリ型環境（memory_ty2stack_ptr）
///を受け取り,それらに相当するMichelson命令をスタックにPUSHする
pub fn prepare(
    michelson_code: String,
    tab: &str,
    register2stack_ptr: &HashMap<Register, usize>,
    register2ty: &HashMap<Register, Type>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    let mut michelson_instructions = vec![];
    let mut memory_ty2stack_ptr_sorted = memory_ty2stack_ptr.iter().collect::<Vec<_>>();
    memory_ty2stack_ptr_sorted.sort_by(|a, b| (b.1).cmp(a.1));
    for (ty, _v) in memory_ty2stack_ptr_sorted.iter() {
        let ty_str = Type::to_michelson_ty_string(&ty);

        let llvm_ty_string = Type::to_llvm_ty_string(ty);
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
            Type::default_value(&ty)
        };
        let michelson_ty = Type::to_michelson_ty_string(&ty);
        let llvm_ty_string = Type::to_llvm_ty_string(ty);

        let comment = if Register::is_const(reg) {
            format!("for const {val} : {llvm_ty_string}")
        } else {
            let id = reg.get_id();
            format!("for reg {id} : {llvm_ty_string}")
        };
        michelson_instructions.push(format!("PUSH {michelson_ty} {val}; # {comment}"));
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
                    format!(
                        "### store {} {}, {}* {} {{",
                        Type::to_llvm_ty_string(ty),
                        value.get_id(),
                        Type::to_llvm_ty_string(ty),
                        ptr.get_id()
                    ),
                    format!("DUP {};", register2stack_ptr.get(&value).unwrap()),
                    format!("SOME;"),
                    format!("DIG {};", register2stack_ptr.len() + memory_ptr),
                    format!("UNPAIR;"),
                    format!("DIG 2;"),
                    format!("DUP {};", register2stack_ptr.get(&ptr).unwrap() + 3),
                    format!("UPDATE;"),
                    format!("PAIR;"),
                    format!("DUG {};", register2stack_ptr.len() + memory_ptr - 1),
                    format!("### }}"),
                ];
                michelson_code = format!(
                    "{michelson_code}{}",
                    utils::format(&michelson_instructions, tab, tab_depth)
                );
            }
            Instruction::Load { result, ty, ptr } => {
                let memory_ptr = memory_ty2stack_ptr.get(ty).unwrap();

                let michelson_instructions = vec![
                    format!(
                        "### {} = load {}, {}* {} {{",
                        result.get_id(),
                        Type::to_llvm_ty_string(ty),
                        Type::to_llvm_ty_string(ty),
                        ptr.get_id()
                    ),
                    format!("DUP {};", register2stack_ptr.len() + memory_ptr),
                    format!("CAR;"),
                    format!("DUP {};", register2stack_ptr.get(&ptr).unwrap() + 1),
                    format!("GET;"),
                    format!("ASSERT_SOME;"),
                    format!("DIG {};", register2stack_ptr.get(&result).unwrap()),
                    format!("DROP;"),
                    format!("DUG {};", register2stack_ptr.get(&result).unwrap() - 1),
                    format!("### }}"),
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
                    format!(
                        "### {} = getElementPtr {}, {}*, {} {{",
                        result.get_id(),
                        Type::to_llvm_ty_string(ty),
                        Type::to_llvm_ty_string(ty),
                        ptrval.get_id()
                    ),
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
                    format!("### }}"),
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
                michelson_code = format!("{michelson_code}{tab}### If {{\n");
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
                    utils::format(&vec![format!("   }};"), format!("### }}"),], tab, tab_depth),
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
                    utils::format(&vec![format!("### While {{")], tab, tab_depth)
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
                    utils::format(
                        &vec![format!("     }};"), format!("### }}")],
                        tab,
                        tab_depth
                    )
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
                    format!("### Op {{"),
                    format!("DUP {};", register2stack_ptr.get(&op2).unwrap()),
                    format!("DUP {};", register2stack_ptr.get(&op1).unwrap() + 1),
                    format!("{op};"),
                    format!("DIG {};", register2stack_ptr.get(&result).unwrap()),
                    format!("DROP;"),
                    format!("DUG {};", register2stack_ptr.get(&result).unwrap() - 1),
                    format!("### }}"),
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
                    format!("### Icmp {{"),
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
                    format!("### }}"),
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

///Michelsonコントラクトとして最後の返り値の準備をする段階において、
///返り値となるStorageをメモリ領域から回収し, Michelsonの入力Storageの
///型に合わせた状態でスタックのトップに持ってくる関数
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
                    format!("PAIR {}; # PACK Struct {{ {id} }}", fields.len()),
                    format!("SWAP;"),
                    format!("DROP; # Storage MAP Instance"),
                ]);
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

///FIXME: 方針はあっているが少しややこしい.
fn retrieve_storage_field_from_memory(
    field_idx: usize,
    field: &Type,
    path: Vec<usize>,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> Vec<String> {
    let memory_ptr = memory_ty2stack_ptr.get(&field).unwrap();
    match field {
        Type::Struct {
            id: child_id,
            fields: child_fields,
        } => {
            //TODO: child_fields.len() > 2, == 1, == 0で場合分け
            let mut michelson_instructions = vec![
                format!("### {{"),
                format!("DUP {}; # MAP instance", path[path.len() - 1]),
                format!("PUSH int {};", field_idx),
                format!("GET;"),
                format!("ASSERT_SOME;"),
                format!(
                    "DUP {}; # memory: {}",
                    register2stack_ptr.len() + memory_ptr + path.iter().sum::<usize>() + 1,
                    Type::to_llvm_ty_string(field)
                ),
                format!("CAR;"),
                format!("SWAP;"),
                format!("GET;"),
                format!("ASSERT_SOME;"),
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
                format!(
                    "PAIR {}; # PACK Struct {{ {child_id} }}",
                    child_fields.len()
                ),
                format!("SWAP;"),
                format!("DROP; # child field MAP Instance"),
                format!("### }}"),
            ]);

            michelson_instructions
        }
        _ => {
            vec![
                format!("### {{"),
                format!("DUP {}; # MAP instance", path[path.len() - 1]),
                format!("PUSH int {};", field_idx),
                format!("GET;"),
                format!("ASSERT_SOME;"),
                format!(
                    "DUP {}; # memory: {}",
                    register2stack_ptr.len() + memory_ptr + path.iter().sum::<usize>() + 1,
                    Type::to_llvm_ty_string(field)
                ),
                format!("CAR;"),
                format!("SWAP;"),
                format!("GET;"),
                format!("ASSERT_SOME;"),
                format!("### }}"),
            ]
        }
    }
}

///(将来的にはこの関数はなくなるかもしれない)
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
