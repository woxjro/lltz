use crate::compiler::utils;
use crate::mini_llvm::{BackendType, Register, Type};
use std::collections::HashMap;

///allocaをMichelsonへとコンパイルする関数
///T(ty)型をallocaし, その領域へのポインタをptrへと格納する命令を生成する
///```llvm
///%ptr = alloca T;
///```
///tab,tab_depthは生成するMichelsonコードをformatする際に使用する
pub fn exec_alloca(
    ptr: &Register,
    ty: &Type,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
) -> String {
    let memory_ptr = memory_ty2stack_ptr
        .get(&BackendType::from(ty.clone()))
        .unwrap();

    let michelson_instructions = match ty {
        Type::Struct { .. } => {
            exec_aggregate_type_alloca(ty, ptr, register2stack_ptr, memory_ty2stack_ptr)
        }
        Type::Array { size, elementtype } => {
            let mut fields = vec![];
            for _ in 0..*size {
                fields.push(*elementtype.clone());
            }
            exec_aggregate_type_alloca(ty, ptr, register2stack_ptr, memory_ty2stack_ptr)
        }
        _ => {
            vec![
                format!("### {} = alloca {} {{", ptr.get_id(), Type::to_llvm_ty(ty),),
                format!("DIG {};", register2stack_ptr.len() + memory_ptr - 1),
                format!("UNPAIR;"),
                format!("SWAP;"),
                format!("PUSH int 1;"),
                format!("ADD;"),
                format!("DUP;"),
                format!("DUP;"),
                format!("DIG 3;"),
                format!("SWAP;"),
                match BackendType::from(ty.clone()) {
                    BackendType::Option(_) => {
                        format!(
                            "{}; # default value",
                            BackendType::default_value(&BackendType::from(ty.clone()))
                        )
                    }
                    BackendType::Contract(_) => panic!(),
                    BackendType::Operation => panic!(),
                    _ => {
                        format!(
                            "PUSH {} {}; # default value",
                            BackendType::from(ty.clone()).to_string(),
                            BackendType::default_value(&BackendType::from(ty.clone()))
                        )
                    }
                },
                format!("SOME;"),
                format!("SWAP;"),
                format!("UPDATE;"),
                format!("PAIR;"),
                format!("DUG {};", register2stack_ptr.len() + memory_ptr),
                format!("DIG {};", register2stack_ptr.get(&ptr).unwrap()),
                format!("DROP;"),
                format!("DUG {};", register2stack_ptr.get(&ptr).unwrap() - 1),
                format!("### }}"),
            ]
        }
    };
    utils::format(&michelson_instructions, tab, tab_depth)
}

///Struct型のAllocaを実行する
///Struct { id, fields } 型をallocaし, その領域へのポインタをptrへと格納する命令を生成する
///```llvm
///%ptr = alloca Struct { id, fields };
///```
pub fn exec_aggregate_type_alloca(
    aggregate_ty: &Type,
    ptr: &Register,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
) -> Vec<String> {
    //Struct { id, fields }型のメモリ領域のスタック上の相対ポインタ

    let memory_ptr = memory_ty2stack_ptr
        .get(&BackendType::from(aggregate_ty.clone()))
        .unwrap();
    let mut res = vec![
        format!(
            "### {} = alloca {} {{",
            ptr.get_id(),
            Type::to_llvm_ty(&aggregate_ty)
        ),
        format!("EMPTY_MAP int int;"),
    ];

    let fields = match aggregate_ty {
        Type::Struct { id: _, fields } => fields.clone(),
        Type::Array { size, elementtype } => {
            let mut fields = vec![];
            for _ in 0..*size {
                fields.push(*elementtype.clone());
            }
            fields
        }
        _ => panic!(),
    };

    for (idx, field) in fields.iter().enumerate() {
        res.append(&mut vec![format!(
            "### alloca field idx={idx} : {} {{",
            Type::to_llvm_ty(field)
        )]);
        res.append(&mut exec_struct_field_alloca(
            idx,
            field,
            1,
            memory_ptr,
            register2stack_ptr,
            memory_ty2stack_ptr,
        ));
        res.append(&mut vec![format!("### }}")]);
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
        format!("### }}"),
    ]);

    res
}

///Struct型のメンバー型も再帰的にAllocaする
///呼び出し元のStruct型のメモリ領域をmap_0とするとこの関数が呼び出された際の
///Michelsonのスタックの状態は
///```michelson_stack
///map_0:map_1:...:map_{depth-1}:register_region:memory_region
///```
///となっており、トップにStructのmapが積まれている事に注意
fn exec_struct_field_alloca(
    idx: usize,
    field: &Type,
    depth: usize,
    memory_ptr: &usize,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
) -> Vec<String> {
    let field_memory_ptr = memory_ty2stack_ptr
        .get(&BackendType::from(field.clone()))
        .unwrap();
    match field {
        Type::Struct { id: _, fields } => {
            let mut res = vec![format!("EMPTY_MAP int int;")];
            for (child_field_idx, child_field) in fields.iter().enumerate() {
                res.append(&mut vec![format!(
                    "### alloca for field No.{child_field_idx} {{"
                )]);
                res.append(&mut self::exec_struct_field_alloca(
                    child_field_idx,
                    child_field,
                    depth + 1,
                    memory_ptr,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                ));
                res.append(&mut vec![format!("### }}")]);
            }
            //TODO: MAP int int をUPDATEでどっかに入れる必要がある
            //child_map:parent_map
            //があったとして、child_mapをchild_mapのbig_mapにいれて返ってきた
            //ptrをparent_mapにkey:idx, value:ptrとして入れる
            res.append(&mut vec![
                format!("SOME;"), //some(map)
                format!(
                    "DIG {};",
                    register2stack_ptr.len() + field_memory_ptr + depth
                ),
                format!("UNPAIR;"), //bm:ptr:child_map
                format!("SWAP;"),
                format!("PUSH int 1;"),
                format!("ADD;"),
                format!("DUP;"),    //ptr:ptr:bm:some(child_map)
                format!("DUP;"),    //ptr:ptr:ptr:bm:some(child_map)
                format!("DIG 3;"),  //bm:ptr:ptr:ptr:some(child_map)
                format!("SWAP;"),   //ptr:bm:ptr:ptr:some(child_map)
                format!("DIG 4;"),  //some(child_map):ptr:bm:ptr:ptr
                format!("SWAP;"),   //ptr:some(child_map):bm:ptr:ptr
                format!("UPDATE;"), //bm:ptr:ptr:parent_map
                format!("PAIR;"),   //(bm,ptr):ptr:parent_map
                format!(
                    "DUG {};",
                    register2stack_ptr.len() + field_memory_ptr + depth
                ), //ptr:parent_map
                format!("SOME;"),
                format!("PUSH int {idx};"),
                format!("UPDATE;"),
            ]);
            res
        }
        Type::Array { size, elementtype } => {
            let mut fields = vec![];
            for _ in 0..*size {
                fields.push(*elementtype.clone());
            }
            let fields = fields;

            let mut res = vec![format!("EMPTY_MAP int int;")];
            for (child_field_idx, child_field) in fields.iter().enumerate() {
                res.append(&mut vec![format!(
                    "### alloca for field No.{child_field_idx} {{"
                )]);
                res.append(&mut self::exec_struct_field_alloca(
                    child_field_idx,
                    child_field,
                    depth + 1,
                    memory_ptr,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                ));
                res.append(&mut vec![format!("### }}")]);
            }
            //TODO: MAP int int をUPDATEでどっかに入れる必要がある
            //child_map:parent_map
            //があったとして、child_mapをchild_mapのbig_mapにいれて返ってきた
            //ptrをparent_mapにkey:idx, value:ptrとして入れる
            res.append(&mut vec![
                format!("SOME;"), //some(map)
                format!(
                    "DIG {};",
                    register2stack_ptr.len() + field_memory_ptr + depth
                ),
                format!("UNPAIR;"), //bm:ptr:child_map
                format!("SWAP;"),
                format!("PUSH int 1;"),
                format!("ADD;"),
                format!("DUP;"),    //ptr:ptr:bm:some(child_map)
                format!("DUP;"),    //ptr:ptr:ptr:bm:some(child_map)
                format!("DIG 3;"),  //bm:ptr:ptr:ptr:some(child_map)
                format!("SWAP;"),   //ptr:bm:ptr:ptr:some(child_map)
                format!("DIG 4;"),  //some(child_map):ptr:bm:ptr:ptr
                format!("SWAP;"),   //ptr:some(child_map):bm:ptr:ptr
                format!("UPDATE;"), //bm:ptr:ptr:parent_map
                format!("PAIR;"),   //(bm,ptr):ptr:parent_map
                format!(
                    "DUG {};",
                    register2stack_ptr.len() + field_memory_ptr + depth
                ), //ptr:parent_map
                format!("SOME;"),
                format!("PUSH int {idx};"),
                format!("UPDATE;"),
            ]);
            res
        }
        _ => {
            //TODO: 多分プリミティブの型に応じてdefault valueを変える必要が出てくる
            //今はIntとPtr型でどっちもintなのでとりあえずOK
            vec![
                //field tyのallocaみたいな事をする
                format!(
                    "DIG {};",
                    register2stack_ptr.len() + field_memory_ptr + depth - 1
                ),
                format!("UNPAIR;"), //bm:ptr:map
                format!("SWAP;"),
                format!("PUSH int 1;"),
                format!("ADD;"),
                format!("DUP;"),
                format!("DUP;"),   //ptr:ptr:ptr:bm:map
                format!("DIG 3;"), //bm:ptr:ptr:ptr:map
                format!("SWAP;"),  //ptr:bm:ptr:ptr:map
                match BackendType::from(field.clone()) {
                    BackendType::Option(_) => {
                        format!(
                            "{}; # default value",
                            BackendType::default_value(&BackendType::from(field.clone()))
                        )
                    }
                    BackendType::Contract(_) => panic!(),
                    BackendType::Operation => panic!(),
                    _ => {
                        format!(
                            "PUSH {} {}; # default value",
                            BackendType::from(field.clone()).to_string(),
                            BackendType::default_value(&BackendType::from(field.clone()))
                        )
                    }
                },
                format!("SOME;"),
                format!("SWAP;"),   //ptr:some(-1):bm:ptr:ptr:map
                format!("UPDATE;"), //bm:ptr:ptr:map
                format!("PAIR;"),   //(bm, ptr):ptr:map
                format!(
                    "DUG {};",
                    register2stack_ptr.len() + field_memory_ptr + depth
                ),
                format!("SOME;"),           //some(ptr):map
                format!("PUSH int {idx};"), //idx:some(ptr):map
                format!("UPDATE;"),         //map
            ]
        }
    }
}
