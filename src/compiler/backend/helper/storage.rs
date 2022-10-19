use crate::compiler::utils;
use crate::mini_llvm::{Arg, Register, Type};
use std::collections::HashMap;

///StorageをMichelsonのPairからLLVMのレジスタ・メモリモデルへとデコードする関数
///(parameter, storage)がスタックに積まれている状態で実行
///実行時のMichelsonのStack状態:
/// input: (parameter, storage):[register region]:[memory region]
///output: (parameter, storage):[register region]:[memory region]
pub fn alloca_storage_by_value(
    storage_arg: &Arg,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    let Arg { reg, ty } = storage_arg;
    let mut michelson_instructions = vec![];
    michelson_instructions.push(format!("### alloca storage {{"));

    //Step 0.(parameter, storage)をスタックの一番下に入れる
    michelson_instructions.push(format!(
        "DUG {};",
        register2stack_ptr.len() + memory_ty2stack_ptr.len()
    ));
    //Step 1.普通のallocaをする（Storageの場所を確保するため）
    match Type::deref(ty) {
        Type::Struct { id, fields } => {
            michelson_instructions.append(&mut super::alloca::exec_struct_alloca(
                &id,
                &fields,
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
    michelson_instructions.push(format!(
        "DIG {};",
        register2stack_ptr.len() + memory_ty2stack_ptr.len()
    ));

    //Step 3.LLVMのメモリモデルへとデコードして値を入れていく
    michelson_instructions.append(&mut decode_storage_from_input(
        &reg,
        &ty,
        register2stack_ptr,
        memory_ty2stack_ptr,
    ));

    michelson_instructions.push(format!("###}}"));
    utils::format(&michelson_instructions, tab, tab_depth)
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
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> Vec<String> {
    let mut michelson_instructions = vec![
        format!("DUP;"),
        format!("UNPAIR;"),
        format!("DROP;"), //parameterを破棄
    ];
    match Type::deref(ty) {
        Type::Struct { id: _, fields } => {
            if fields.len() == 0 {
                // unit
                // do nothing
                michelson_instructions.push(format!("DROP;"));
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
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> Vec<String> {
    let mut michelson_instructions = vec![format!("DUP;"), format!("GET {};", get_n_idx)];

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
        }
        _ => {
            /* primitiveの値がスタックの上に乗っているのでそれを使って,Memoryに入れる */
            michelson_instructions.append(&mut vec![
                format!("### PUT {{"), //
                format!("SOME;"),
            ]);
            for (i, (child_idx, child_ty)) in path.iter().enumerate() {
                let memory_ptr = memory_ty2stack_ptr.get(child_ty).unwrap();

                if i == 0 {
                    /* 最初はptrを使う */
                    michelson_instructions.append(&mut vec![
                        format!(
                            "DUP {};",
                            register2stack_ptr.len() + memory_ptr + (depth + 2)
                        ),
                        format!("CAR;"), //bm:some(v):michelson_instructionst
                        format!(
                            "DUP {};",
                            register2stack_ptr.get(ptr).unwrap() + (depth + 2) + 1
                        ), //key:bm:some(v)
                        format!("GET;"),
                        format!("ASSERT_SOME;"), //struct_instance_bm:some(v)
                        format!("PUSH int {child_idx};"),
                        format!("GET;"),
                        format!("ASSERT_SOME;"), //field_ptr:some(v)
                    ]);
                } else {
                    michelson_instructions.append(&mut vec![
                        format!(
                            "DUP {};",
                            register2stack_ptr.len() + memory_ptr + (depth + 2) + 1
                        ),
                        format!("CAR;"),
                        format!("SWAP;"), //field_ptr:mem:some(v)
                        format!("GET;"),
                        format!("ASSERT_SOME;"), //field_instance:some(v)
                        format!("PUSH int {};", *child_idx),
                        format!("GET;"),
                        format!("ASSERT_SOME;"), //field_ptr:some(v)
                    ]);
                }
            }

            let memory_ptr = memory_ty2stack_ptr.get(ty).unwrap();
            michelson_instructions.append(&mut vec![
                format!(
                    "DIG {};",
                    register2stack_ptr.len() + memory_ptr + (depth + 2) //+ 1 - 1
                ), //(mem,cnt):field_ptr:some(v)
                format!("UNPAIR;"), //mem:cnt:field_ptr:some(v)
                format!("DIG 3;"),  //some(v):mem:cnt:field_ptr
                format!("DIG 3;"),  //field_ptr:some(v):mem:cnt
                format!("UPDATE;"), //mem:cnt
                format!("PAIR;"),   //(mem,cnt)
                format!(
                    "DUG {};",
                    register2stack_ptr.len() + (memory_ptr - 1) + (depth + 1)
                ),
            ]);
            //最後の要素だった場合は後処理
            //Struct { .. }から出るときは後処理が必要。入る時にDUPしている為.
            if is_last_field {
                michelson_instructions.push(format!("DROP;"));
            }
            michelson_instructions.push(format!("### }}"));
        }
    }
    michelson_instructions
}
