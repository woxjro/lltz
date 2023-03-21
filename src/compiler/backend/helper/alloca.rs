use crate::lltz_ir::{BackendType, Register, Type};
use michelson_ast::instruction::Instruction as MInstr;
use michelson_ast::instruction_with_comment::InstructionWithComment as MInstrWrapper;
use michelson_ast::ty::Ty as MTy;
use michelson_ast::val::Val as MVal;
use std::collections::HashMap;

///allocaをMichelsonへとコンパイルする関数
///T(ty)型をallocaし, その領域へのポインタをptrへと格納する命令を生成する
///```llvm
///%ptr = alloca T;
///```
pub fn exec_alloca(
    ptr: &Register,
    ty: &Type,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<BackendType, usize>,
) -> Vec<MInstrWrapper> {
    let memory_ptr = memory_ty2stack_ptr.get(&BackendType::from(ty)).unwrap();

    let instructions = match ty {
        Type::Struct { .. } => {
            exec_aggregate_type_alloca(ty, ptr, register2stack_ptr, memory_ty2stack_ptr)
        }
        Type::Array { size, elementtype } => {
            let mut fields = vec![];
            for _ in 0..*size {
                fields.push(elementtype);
            }
            exec_aggregate_type_alloca(ty, ptr, register2stack_ptr, memory_ty2stack_ptr)
        }
        _ => vec![
            MInstr::Comment(format!(
                "{} = alloca {} {{",
                ptr.get_id(),
                Type::get_name(ty)
            )),
            MInstr::DigN(register2stack_ptr.len() + memory_ptr - 1),
            MInstr::Unpair,
            MInstr::Swap,
            MInstr::Push {
                ty: MTy::Int,
                val: MVal::Int(1),
            },
            MInstr::Add,
            MInstr::Dup,
            MInstr::Dup,
            MInstr::DigN(3),
            MInstr::Swap,
            match BackendType::from(ty) {
                BackendType::Contract(_) => panic!("起こりえない"),
                BackendType::Operation => panic!("起こりえない"),
                _ => BackendType::default_value_instruction(&BackendType::from(ty)),
            },
            MInstr::Some,
            MInstr::Swap,
            MInstr::Update,
            MInstr::Pair,
            MInstr::DugN(register2stack_ptr.len() + memory_ptr),
            MInstr::DigN(*register2stack_ptr.get(&ptr).unwrap()),
            MInstr::Drop,
            MInstr::DugN(register2stack_ptr.get(&ptr).unwrap() - 1),
            MInstr::Comment(format!("}}")),
        ]
        .iter()
        .map(|instr| instr.to_instruction_with_comment())
        .collect::<Vec<_>>(),
    };
    instructions
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
) -> Vec<MInstrWrapper> {
    //Struct { id, fields }型のメモリ領域のスタック上の相対ポインタ

    let memory_ptr = memory_ty2stack_ptr
        .get(&BackendType::from(aggregate_ty))
        .unwrap();
    let mut res = vec![
        MInstr::Comment(format!(
            "{} = alloca {} {{",
            ptr.get_id(),
            Type::get_name(&aggregate_ty)
        )),
        MInstr::EmptyMap {
            kty: MTy::Int,
            vty: MTy::Int,
        },
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
        res.append(&mut vec![MInstr::Comment(format!(
            "{}[{idx}] = alloca {} {{",
            Type::get_name(&aggregate_ty),
            Type::get_name(field),
        ))]);
        res.append(&mut exec_struct_field_alloca(
            idx,
            field,
            1,
            memory_ptr,
            register2stack_ptr,
            memory_ty2stack_ptr,
        ));
        res.append(&mut vec![MInstr::Comment(format!("}}",))]);
    }

    res.append(&mut vec![
        MInstr::Some, //some(map)
        MInstr::DigN(register2stack_ptr.len() + memory_ptr),
        MInstr::Unpair, //bm:ptr:some(map)
        MInstr::Swap,   //ptr:bm:some(map)
        MInstr::Push {
            ty: MTy::Int,
            val: MVal::Int(1),
        },
        MInstr::Add,
        MInstr::Dup,
        MInstr::Dup,     //ptr:ptr:ptr:bm:some(map)
        MInstr::DigN(3), //bm:ptr:ptr:ptr:some(map)
        MInstr::DigN(4), //some(map):bm:ptr:ptr:ptr
        MInstr::DigN(2),
        MInstr::Update, //bm:ptr:ptr
        MInstr::Pair,   //(bm,ptr):ptr
        MInstr::DugN(register2stack_ptr.len() + memory_ptr),
        MInstr::DigN(*register2stack_ptr.get(&ptr).unwrap()),
        MInstr::Drop,
        MInstr::DugN(register2stack_ptr.get(&ptr).unwrap() - 1),
        MInstr::Comment(format!("}}")),
    ]);

    res.iter()
        .map(|instr| instr.to_instruction_with_comment())
        .collect::<Vec<_>>()
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
) -> Vec<MInstr> {
    let field_memory_ptr = memory_ty2stack_ptr.get(&BackendType::from(field)).unwrap();
    match field {
        Type::Struct { id: _, fields } => {
            let mut res = vec![MInstr::EmptyMap {
                kty: MTy::Int,
                vty: MTy::Int,
            }];
            for (child_field_idx, child_field) in fields.iter().enumerate() {
                res.append(&mut vec![MInstr::Comment(format!(
                    "alloca for field No.{child_field_idx} {{"
                ))]);
                res.append(&mut self::exec_struct_field_alloca(
                    child_field_idx,
                    child_field,
                    depth + 1,
                    memory_ptr,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                ));
                res.append(&mut vec![MInstr::Comment(format!("}}"))]);
            }
            //TODO: MAP int int をUPDATEでどっかに入れる必要がある
            //child_map:parent_map
            //があったとして、child_mapをchild_mapのbig_mapにいれて返ってきた
            //ptrをparent_mapにkey:idx, value:ptrとして入れる
            res.append(&mut vec![
                MInstr::Some, //some(map)
                MInstr::DigN(register2stack_ptr.len() + field_memory_ptr + depth),
                MInstr::Unpair, //bm:ptr:child_map
                MInstr::Swap,
                MInstr::Push {
                    ty: MTy::Int,
                    val: MVal::Int(1),
                },
                MInstr::Add,
                MInstr::Dup,
                MInstr::Dup,
                MInstr::DigN(3), //bm:ptr:ptr:ptr:some(child_map)
                MInstr::Swap,    //ptr:bm:ptr:ptr:some(child_map)
                MInstr::DigN(4), //some(child_map):ptr:bm:ptr:ptr
                MInstr::Swap,    //ptr:some(child_map):bm:ptr:ptr
                MInstr::Update,  //bm:ptr:ptr:parent_map
                MInstr::Pair,    //(bm,ptr):ptr:parent_map
                MInstr::DugN(register2stack_ptr.len() + field_memory_ptr + depth), //ptr:parent_map
                MInstr::Some,
                MInstr::Push {
                    ty: MTy::Int,
                    val: MVal::Int(idx.try_into().unwrap()),
                },
                MInstr::Update,
            ]);
            res
        }
        Type::Array { size, elementtype } => {
            let mut fields = vec![];
            for _ in 0..*size {
                fields.push(*elementtype.clone());
            }
            let fields = fields;

            let mut res = vec![MInstr::EmptyMap {
                kty: MTy::Int,
                vty: MTy::Int,
            }];
            for (child_field_idx, child_field) in fields.iter().enumerate() {
                res.append(&mut vec![MInstr::Comment(format!(
                    "alloca for field No.{child_field_idx} {{"
                ))]);
                res.append(&mut self::exec_struct_field_alloca(
                    child_field_idx,
                    child_field,
                    depth + 1,
                    memory_ptr,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                ));
                res.append(&mut vec![MInstr::Comment(format!("}}"))]);
            }
            //TODO: MAP int int をUPDATEでどっかに入れる必要がある
            //child_map:parent_map
            //があったとして、child_mapをchild_mapのbig_mapにいれて返ってきた
            //ptrをparent_mapにkey:idx, value:ptrとして入れる
            res.append(&mut vec![
                MInstr::Some, //some(map)
                MInstr::DigN(register2stack_ptr.len() + field_memory_ptr + depth),
                MInstr::Unpair, //bm:ptr:child_map
                MInstr::Swap,
                MInstr::Push {
                    ty: MTy::Int,
                    val: MVal::Int(1),
                },
                MInstr::Add,
                MInstr::Dup,     //ptr:ptr:bm:some(child_map)
                MInstr::Dup,     //ptr:ptr:ptr:bm:some(child_map)
                MInstr::DigN(3), //bm:ptr:ptr:ptr:some(child_map)
                MInstr::Swap,    //ptr:bm:ptr:ptr:some(child_map)
                MInstr::DigN(4), //some(child_map):ptr:bm:ptr:ptr
                MInstr::Swap,    //ptr:some(child_map):bm:ptr:ptr
                MInstr::Update,  //bm:ptr:ptr:parent_map
                MInstr::Pair,    //(bm,ptr):ptr:parent_map
                MInstr::DugN(register2stack_ptr.len() + field_memory_ptr + depth), //ptr:parent_map
                MInstr::Some,
                MInstr::Push {
                    ty: MTy::Int,
                    val: MVal::Int(idx.try_into().unwrap()),
                },
                MInstr::Update,
            ]);
            res
        }
        _ => {
            //TODO: 多分プリミティブの型に応じてdefault valueを変える必要が出てくる
            //今はIntとPtr型でどっちもintなのでとりあえずOK
            vec![
                //field tyのallocaみたいな事をする
                MInstr::DigN(register2stack_ptr.len() + field_memory_ptr + depth - 1),
                MInstr::Unpair, //bm:ptr:map
                MInstr::Swap,
                MInstr::Push {
                    ty: MTy::Int,
                    val: MVal::Int(1),
                },
                MInstr::Add,
                MInstr::Dup,
                MInstr::Dup,     //ptr:ptr:ptr:bm:map
                MInstr::DigN(3), //bm:ptr:ptr:ptr:map
                MInstr::Swap,    //ptr:bm:ptr:ptr:map
                match BackendType::from(field) {
                    BackendType::Contract(_) => panic!("起こりえない"),
                    BackendType::Operation => panic!("起こりえない"),
                    _ => BackendType::default_value_instruction(&BackendType::from(field)),
                },
                MInstr::Some,
                MInstr::Swap,   //ptr:some(-1):bm:ptr:ptr:map
                MInstr::Update, //bm:ptr:ptr:map
                MInstr::Pair,   //(bm, ptr):ptr:map
                MInstr::DugN(register2stack_ptr.len() + field_memory_ptr + depth),
                MInstr::Some,
                MInstr::Push {
                    ty: MTy::Int,
                    val: MVal::Int(idx.try_into().unwrap()),
                }, //idx:some(ptr):map
                MInstr::Update, //map
            ]
        }
    }
}
