use crate::lltz_ir::{InnerType, Register, Type};
use michelson_ast::instruction::Instruction as MInstr;
use michelson_ast::instruction_with_comment::InstructionWithComment as MInstrWrapper;
use michelson_ast::ty::Ty as MTy;
use michelson_ast::val::Val as MVal;
use std::collections::HashMap;

///@llvm.memcpyを実行する関数
///structなどへのポインタdestを受け取り, それをsrcポインタ先のstructにコピーする
///tyはdest,srcが指す先の値の型, pathはstructのmemberなどのネストした値をたどるために使う配列
///register2tyはレジスタの型環境
pub fn exec_llvm_memcpy(
    dest: &Register,
    src: &Register,
    ty: &Type,
    register2stack_ptr: &HashMap<Register, usize>,
    register2ty: &HashMap<Register, InnerType>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
) -> Vec<MInstrWrapper> {
    //validation
    match register2ty.get(&dest).unwrap() {
        InnerType::Ptr(inner) => {
            if **inner != InnerType::from(ty) {
                panic!(
                    "@llvm.memcpyでdestの指す先の型:{}がty:{}と一致していません.",
                    inner.get_name(),
                    InnerType::from(ty).get_name()
                );
            }
        }
        _ => {
            panic!("@llvm.memcpyのdestがポインタ型になっていません.");
        }
    }

    match register2ty.get(&src).unwrap() {
        InnerType::Ptr(inner) => {
            if **inner != InnerType::from(ty) {
                panic!(
                    "@llvm.memcpyでsrcの指す先の型:{}がty:{}と一致していません.",
                    inner.get_name(),
                    InnerType::from(ty).get_name()
                );
            }
        }
        _ => {
            panic!("@llvm.memcpyのsrcがポインタ型になっていません.");
        }
    }

    let mut michelson_instructions =
        vec![MInstr::Comment(format!("@llvm.memcpy {{",)).to_instruction_with_comment()];
    match ty {
        Type::Struct { id: _, fields } => {
            let depth = 1;
            let memory_ptr = memory_ty2stack_ptr.get(&InnerType::from(ty)).unwrap();
            michelson_instructions.append(
                &mut vec![
                    MInstr::DupN(register2stack_ptr.len() + memory_ptr),
                    MInstr::Car,
                    MInstr::DupN(register2stack_ptr.get(&src).unwrap() + 1),
                    MInstr::Get,
                    MInstr::AssertSome,
                ]
                .iter()
                .map(|instr| instr.to_instruction_with_comment())
                .collect::<Vec<_>>(),
            );

            for (idx, field) in fields.iter().enumerate() {
                //DUP big_map struct { id, fields }
                let field_memory_ptr = memory_ty2stack_ptr.get(&InnerType::from(field)).unwrap();
                michelson_instructions.append(
                    &mut vec![
                        vec![MInstr::Comment(format!(
                            "### llvm.memcpy GET {}[{idx}] {{",
                            Type::get_name(&ty)
                        ))
                        .to_instruction_with_comment()],
                        vec![
                            MInstr::Dup,
                            MInstr::Push {
                                ty: MTy::Int,
                                val: MVal::Int(idx.try_into().unwrap()),
                            },
                            MInstr::Get,
                            MInstr::AssertSome, // ptr4field:rest
                            MInstr::DupN(register2stack_ptr.len() + field_memory_ptr + 2),
                            MInstr::Car,
                            MInstr::Swap,
                            MInstr::Get,
                            MInstr::AssertSome, // field_type_value:rest
                        ]
                        .iter()
                        .map(|instr| instr.to_instruction_with_comment())
                        .collect::<Vec<_>>(),
                    ]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>(),
                );

                let mut path = vec![(idx, ty.clone())];
                //GET
                michelson_instructions.append(&mut self::get_field_element(
                    depth + 1,
                    field,
                    &mut path,
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    dest,
                ));

                michelson_instructions.append(&mut vec![
                    MInstr::Comment(format!("}}")).to_instruction_with_comment()
                ]);
            }
            michelson_instructions.push(MInstr::Drop.to_instruction_with_comment());
        }
        _ => {
            /*primitive or pointer*/
            panic!("Primitive(Pointer)型に対して@llvm.memcpyは実行出来ません.");
        }
    };
    michelson_instructions.push(MInstr::Comment(format!("}}")).to_instruction_with_comment());
    michelson_instructions
}

///@llvm.memcpyをサポートする関数
///DFSでStruct型の中を再帰的に探索し, Primitive型に到達したら、その値で
///destの先のStruct型に値をコピー（UPDATE)する. このUPDATEする操作はput_field_elementが担当
///pathにPrimitive型に辿り着く際にたどってきたパスを保存しておき、put_field_elementに渡す
fn get_field_element(
    depth: usize,
    field: &Type,
    path: &mut Vec<(usize, Type)>,
    register2stack_ptr: &HashMap<Register, usize>,
    register2ty: &HashMap<Register, InnerType>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
    dest: &Register,
) -> Vec<MInstrWrapper> {
    let mut res = vec![];
    match field {
        Type::Struct { id: _, fields } => {
            for (child_idx, child_field) in fields.iter().enumerate() {
                let memory_ptr = memory_ty2stack_ptr.get(&InnerType::from(field)).unwrap();
                res.append(
                    &mut vec![
                        MInstr::Dup, //bm:bm:rest
                        MInstr::Push {
                            ty: MTy::Int,
                            val: MVal::Int(child_idx.try_into().unwrap()),
                        }, //idx:bm:bm:rest
                        MInstr::Get,
                        MInstr::AssertSome, // ptr4field:bm:rest
                        MInstr::DupN(register2stack_ptr.len() + memory_ptr + depth),
                        MInstr::Car,
                        MInstr::Swap,
                        MInstr::Get,
                        MInstr::AssertSome,
                    ]
                    .iter()
                    .map(|instr| instr.to_instruction_with_comment())
                    .collect::<Vec<_>>(),
                );
                let mut new_path = path.clone();
                new_path.push((child_idx, field.clone()));
                res.append(&mut self::get_field_element(
                    depth + 1,
                    child_field,
                    &mut new_path,
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    dest,
                ));
            }
            res.append(&mut vec![MInstr::Drop.to_instruction_with_comment()]);
        }
        _ => {
            /*この関数の役目は終わりPUTの処理へ*/
            res.append(&mut vec![
                MInstr::Comment(format!("@llvm.memcpy PUT {{")).to_instruction_with_comment()
            ]);
            res.append(&mut self::put_field_element(
                depth,
                field,
                &path,
                register2stack_ptr,
                memory_ty2stack_ptr,
                dest,
            ));
            res.push(MInstr::Comment(format!("}}")).to_instruction_with_comment());
        }
    }

    res
}

///@llvm.memcpyをサポートする関数
///DFSでStruct型を辿ってきたpathを受け取り、destポインタとそのpathを元にして
///srcの値をdestへとコピーする関数
fn put_field_element(
    depth: usize,
    primitive_ty: &Type,
    path: &Vec<(usize, Type)>,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
    dest: &Register,
) -> Vec<MInstrWrapper> {
    let mut res = vec![MInstr::Some.to_instruction_with_comment()];
    for (i, (child_idx, child_ty)) in path.iter().enumerate() {
        let memory_ptr = memory_ty2stack_ptr.get(&InnerType::from(child_ty)).unwrap();

        if i == 0 {
            /* 最初はdestを使う */
            res.append(
                &mut vec![
                    MInstr::DupN(register2stack_ptr.len() + memory_ptr + depth),
                    MInstr::Car, //bm:some(v):rest
                    MInstr::DupN(register2stack_ptr.get(dest).unwrap() + depth + 1), //key:bm:some(v):rest
                    MInstr::Get,
                    MInstr::AssertSome, //struct_instance_bm:some(v)
                    MInstr::Push {
                        ty: MTy::Int,
                        val: MVal::Int((*child_idx).try_into().unwrap()),
                    },
                    MInstr::Get,
                    MInstr::AssertSome, //field_ptr:some(v)
                ]
                .iter()
                .map(|instr| instr.to_instruction_with_comment())
                .collect::<Vec<_>>(),
            );
        } else {
            res.append(
                &mut vec![
                    MInstr::DupN(register2stack_ptr.len() + memory_ptr + depth + 1),
                    MInstr::Car,
                    MInstr::Swap,
                    MInstr::Get,
                    MInstr::AssertSome,
                    MInstr::Push {
                        ty: MTy::Int,
                        val: MVal::Int((*child_idx).try_into().unwrap()),
                    },
                    MInstr::Get,
                    MInstr::AssertSome,
                ]
                .iter()
                .map(|instr| instr.to_instruction_with_comment())
                .collect::<Vec<_>>(),
            );
        }
    }

    let memory_ptr = memory_ty2stack_ptr
        .get(&InnerType::from(primitive_ty))
        .unwrap();
    res.append(
        &mut vec![
            MInstr::DigN(register2stack_ptr.len() + memory_ptr + depth),
            MInstr::Unpair,
            MInstr::DigN(3),
            MInstr::DigN(3),
            MInstr::Update,
            MInstr::Pair,
            MInstr::DugN(register2stack_ptr.len() + memory_ptr + (depth - 1) - 1),
        ]
        .iter()
        .map(|instr| instr.to_instruction_with_comment())
        .collect::<Vec<_>>(),
    );

    res
}
