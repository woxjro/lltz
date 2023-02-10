use crate::lltz_ir::{Arg, BackendType, Register, Type};
use michelson_ast::instruction::Instruction as MInstr;
use michelson_ast::instruction_wrapper::InstructionWrapper as MInstrWrapper;
use michelson_ast::ty::Ty as MTy;
use michelson_ast::val::Val as MVal;
use std::collections::HashMap;

///StorageをMichelsonのPairからLLVMのレジスタ・メモリモデルへとデコードする関数
///(parameter, storage)がスタックに積まれている状態で実行
///実行時のMichelsonのStack状態:
/// input: (parameter, storage):[register region]:[memory region]
///output: (parameter, storage):[register region]:[memory region]
pub fn alloca_storage_by_value(
    storage_arg: &Arg,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
) -> Vec<MInstrWrapper> {
    let Arg { reg, ty } = storage_arg;
    let mut michelson_instructions = vec![];
    michelson_instructions.push(MInstrWrapper::Comment(format!("alloca storage {{")));

    //Step 0.(parameter, storage)をスタックの一番下に入れる
    michelson_instructions.push(
        MInstr::DugN(register2stack_ptr.len() + memory_ty2stack_ptr.len()).to_instruction_wrapper(),
    );
    //Step 1.普通のallocaをする（Storageの場所を確保するため）
    match Type::deref(ty) {
        Type::Struct { .. } => {
            michelson_instructions.append(&mut super::alloca::exec_aggregate_type_alloca(
                &Type::deref(ty),
                reg,
                register2stack_ptr,
                memory_ty2stack_ptr,
            ));
        }
        _ => {
            panic!("引数tyがStruct型ではありません.")
        }
    }

    //Step 2.(parameter, storage)を上に持ってきた後,
    michelson_instructions.push(
        MInstr::DigN(register2stack_ptr.len() + memory_ty2stack_ptr.len()).to_instruction_wrapper(),
    );

    //Step 3.LLVMのメモリモデルへとデコードして値を入れていく
    michelson_instructions.append(&mut decode_storage_from_input(
        &reg,
        &ty,
        register2stack_ptr,
        memory_ty2stack_ptr,
    ));

    michelson_instructions.push(MInstrWrapper::Comment(format!("}}")));
    michelson_instructions
}

/// Michelson引数のStorageをLLVMのメモリモデルへとデコードする関数
/// ptrをデコードした値を入れるStorage型のメモリアドレスとし,tyをptrの型とする
/// 実行時のMichelsonのStack状態:
/// input: (parameter, storage):[register region]:[memory region]
///output: (parameter, storage):[register region]:[memory region]
fn decode_storage_from_input(
    ptr: &Register,
    ty: &Type,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
) -> Vec<MInstrWrapper> {
    let mut michelson_instructions = vec![MInstr::Dup, MInstr::Unpair, MInstr::Drop]
        .iter()
        .map(|instr| instr.to_instruction_wrapper())
        .collect::<Vec<_>>();

    match Type::deref(ty) {
        Type::Struct { id: _, fields } => {
            if fields.len() == 0 {
                // unit
                // do nothing
                michelson_instructions.push(MInstr::Drop.to_instruction_wrapper());
            } else if fields.len() == 1 {
                // ty
                todo!()
            } else {
                // pair ty0 ty1 ty2 ty3 ..
                for (field_idx, field) in fields.iter().enumerate() {
                    let get_n_idx = if field_idx + 1 == fields.len() {
                        field_idx * 2
                    } else {
                        field_idx * 2 + 1
                    };
                    let is_last_field = field_idx + 1 == fields.len();
                    michelson_instructions.append(&mut decode_storage_field_from_input(
                        ptr,
                        get_n_idx,
                        field,
                        1,
                        is_last_field,
                        vec![(field_idx, Type::deref(ty))],
                        register2stack_ptr,
                        memory_ty2stack_ptr,
                    ));
                }
            }
        }
        _ => {
            panic!("StorageがStruct型ではなくPrimitive型になっています.")
        }
    }

    michelson_instructions
}

/// 実行時のMichelsonのStack状態:
/// input: [pair_0..pair_n]:storage:(parameter, storage):[register region]:[memory region]
///output: [pair_0..pair_n]:storage:(parameter, storage):[register region]:[memory region]
fn decode_storage_field_from_input(
    ptr: &Register,
    get_n_idx: usize,
    ty: &Type,
    depth: usize,
    is_last_field: bool,
    path: Vec<(usize, Type)>,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
) -> Vec<MInstrWrapper> {
    let mut michelson_instructions = vec![MInstr::Dup, MInstr::GetN(get_n_idx)]
        .iter()
        .map(|instr| instr.to_instruction_wrapper())
        .collect::<Vec<_>>();

    match ty {
        Type::Struct {
            id: _child_id,
            fields: child_fields,
        } => {
            for (child_field_idx, child_field) in child_fields.iter().enumerate() {
                let get_n_idx = if child_field_idx + 1 == child_fields.len() {
                    child_field_idx * 2
                } else {
                    child_field_idx * 2 + 1
                };
                let is_last_child_field = child_field_idx + 1 == child_fields.len();
                //NOTE: DROP;child_tyを入れるのではない.
                let new_path = vec![path.clone(), vec![(child_field_idx, ty.clone())]].concat();
                michelson_instructions.append(&mut decode_storage_field_from_input(
                    ptr,
                    get_n_idx,
                    child_field,
                    depth + 1,
                    is_last_child_field,
                    new_path.clone(),
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                ));
            }
            //最後の要素だった場合は後処理
            //Struct { .. }から出るときは後処理が必要。入る時にDUPしている為.
            if is_last_field {
                michelson_instructions.push(MInstr::Drop.to_instruction_wrapper());
            }
        }
        _ => {
            /* primitiveの値がスタックの上に乗っているのでそれを使って,Memoryに入れる */
            michelson_instructions.append(&mut vec![
                MInstrWrapper::Comment(format!("PUT {{")),
                MInstr::Some.to_instruction_wrapper(),
            ]);
            for (i, (child_idx, child_ty)) in path.iter().enumerate() {
                let memory_ptr = memory_ty2stack_ptr
                    .get(&BackendType::from(child_ty))
                    .unwrap();

                if i == 0 {
                    /* 最初はptrを使う */
                    michelson_instructions.append(
                        &mut vec![
                            MInstr::DupN(register2stack_ptr.len() + memory_ptr + (depth + 2)),
                            MInstr::Car, //bm:some(v):michelson_instructionst
                            MInstr::DupN(register2stack_ptr.get(ptr).unwrap() + (depth + 2) + 1), //key:bm:some(v)
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
                        .map(|instr| instr.to_instruction_wrapper())
                        .collect::<Vec<_>>(),
                    );
                } else {
                    michelson_instructions.append(
                        &mut vec![
                            MInstr::DupN(register2stack_ptr.len() + memory_ptr + (depth + 2) + 1),
                            MInstr::Car,
                            MInstr::Swap, //field_ptr:mem:some(v)
                            MInstr::Get,
                            MInstr::AssertSome, //field_instance:some(v)
                            MInstr::Push {
                                ty: MTy::Int,
                                val: MVal::Int((*child_idx).try_into().unwrap()),
                            },
                            MInstr::Get,
                            MInstr::AssertSome, //field_ptr:some(v)
                        ]
                        .iter()
                        .map(|instr| instr.to_instruction_wrapper())
                        .collect::<Vec<_>>(),
                    );
                }
            }

            let memory_ptr = memory_ty2stack_ptr.get(&BackendType::from(ty)).unwrap();
            michelson_instructions.append(
                &mut vec![
                    MInstr::DigN(
                        register2stack_ptr.len() + memory_ptr + (depth + 2), /* + 1 - 1 */
                    ), //(mem,cnt):field_ptr:some(v)
                    MInstr::Unpair,  //mem:cnt:field_ptr:some(v)
                    MInstr::DigN(3), //some(v):mem:cnt:field_ptr
                    MInstr::DigN(3), //field_ptr:some(v):mem:cnt
                    MInstr::Update,  //mem:cnt
                    MInstr::Pair,    //(mem,cnt)
                    MInstr::DugN(register2stack_ptr.len() + (memory_ptr - 1) + (depth + 1)),
                ]
                .iter()
                .map(|instr| instr.to_instruction_wrapper())
                .collect::<Vec<_>>(),
            );
            //最後の要素だった場合は後処理
            //Struct { .. }から出るときは後処理が必要。入る時にDUPしている為.
            if is_last_field {
                michelson_instructions.push(MInstr::Drop.to_instruction_wrapper())
            }
            michelson_instructions.push(MInstrWrapper::Comment(format!("}}")));
        }
    }
    michelson_instructions
}
