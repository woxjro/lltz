use crate::compiler::utils;
use crate::mini_llvm::{Register, Type};
use std::collections::HashMap;

///@llvm.memcpyを実行する関数
///structなどへのポインタdestを受け取り, それをsrcポインタ先のstructにコピーする
///tyはdest,srcが指す先の値の型, pathはstructのmemberなどのネストした値をたどるために使う配列
///register2tyはレジスタの型環境
pub fn exec_llvm_memcpy(
    dest: &Register,
    src: &Register,
    ty: &Type,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    register2ty: &HashMap<Register, Type>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    //validation
    match register2ty.get(&dest).unwrap() {
        Type::Ptr(inner) => {
            if inner != &Box::new(ty.clone()) {
                panic!();
            }
        }
        _ => {
            panic!();
        }
    }

    match register2ty.get(&src).unwrap() {
        Type::Ptr(inner) => {
            if inner != &Box::new(ty.clone()) {
                panic!();
            }
        }
        _ => {
            panic!();
        }
    }

    let mut michelson_instructions = vec![format!("### llvm.memcpy {{")];
    match ty {
        Type::Struct { id: _, fields } => {
            let depth = 1;
            let memory_ptr = memory_ty2stack_ptr.get(ty).unwrap();
            michelson_instructions.append(&mut vec![
                format!("DUP {};", register2stack_ptr.len() + memory_ptr),
                format!("CAR;"),
                format!("DUP {};", register2stack_ptr.get(&src).unwrap() + 1),
                format!("GET;"),
                format!("ASSERT_SOME;"), // struct_map_instance:rest
            ]);

            for (idx, field) in fields.iter().enumerate() {
                //DUP big_map struct { id, fields }
                let field_memory_ptr = memory_ty2stack_ptr.get(field).unwrap();
                michelson_instructions.append(&mut vec![
                    format!("### llvm.memcpy GET idx={idx} {{"),
                    format!("DUP;"),
                    format!("PUSH int {idx};"),
                    format!("GET;"),
                    format!("ASSERT_SOME;"), // ptr4field:rest
                    format!("DUP {};", register2stack_ptr.len() + field_memory_ptr + 2),
                    format!("CAR;"),
                    format!("SWAP;"),
                    format!("GET;"),
                    format!("ASSERT_SOME;"), // field_type_value:rest
                ]);

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
                    format!("### }}"),
                    //format!("### llvm.memcpy PUT {idx} {{"),
                ]);
            }
            michelson_instructions.push(format!("DROP;"));
        }
        _ => {
            /*primitive or pointer*/
            panic!("Primitive(Pointer)型に対して@llvm.memcpyは実行出来ません.");
        }
    };
    michelson_instructions.push(format!("### }}"));
    utils::format(&michelson_instructions, tab, tab_depth)
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
    register2ty: &HashMap<Register, Type>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
    dest: &Register,
) -> Vec<String> {
    let mut res = vec![];
    match field {
        Type::Struct { id: _, fields } => {
            for (child_idx, child_field) in fields.iter().enumerate() {
                let memory_ptr = memory_ty2stack_ptr.get(field).unwrap();
                res.append(&mut vec![
                    format!("DUP;"),                  //bm:bm:rest
                    format!("PUSH int {child_idx};"), //idx:bm:bm:rest
                    format!("GET;"),
                    format!("ASSERT_SOME;"), // ptr4field:bm:rest
                    format!("DUP {};", register2stack_ptr.len() + memory_ptr + depth),
                    format!("CAR;"),
                    format!("SWAP;"),
                    format!("GET;"),
                    format!("ASSERT_SOME;"), // field_type_value:bm:rest
                ]);
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
            res.append(&mut vec![format!("DROP;")]);
        }
        _ => {
            /*この関数の役目は終わりPUTの処理へ*/
            res.append(&mut vec![
                format!("### llvm.memcpy PUT {{"),
                //format!("DROP;"),
            ]);
            res.append(&mut self::put_field_element(
                depth,
                field,
                &path,
                register2stack_ptr,
                memory_ty2stack_ptr,
                dest,
            ));
            res.push(format!("### }}"));
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
    memory_ty2stack_ptr: &HashMap<Type, usize>,
    dest: &Register,
) -> Vec<String> {
    let mut res = vec![format!("SOME;")];
    for (i, (child_idx, child_ty)) in path.iter().enumerate() {
        let memory_ptr = memory_ty2stack_ptr.get(child_ty).unwrap();

        if i == 0 {
            /* 最初はdestを使う */
            res.append(&mut vec![
                format!("DUP {};", register2stack_ptr.len() + memory_ptr + depth),
                format!("CAR;"), //bm:some(v):rest
                format!("DUP {};", register2stack_ptr.get(dest).unwrap() + depth + 1), //key:bm:some(v):rest
                format!("GET;"),
                format!("ASSERT_SOME;"), //struct_instance_bm:some(v)
                format!("PUSH int {child_idx};"),
                format!("GET;"),
                format!("ASSERT_SOME;"), //field_ptr:some(v)
            ]);
        } else {
            res.append(&mut vec![
                format!("DUP {};", register2stack_ptr.len() + memory_ptr + depth + 1),
                format!("CAR;"),
                format!("SWAP;"),
                format!("GET;"),
                format!("ASSERT_SOME;"),
                format!("PUSH int {};", *child_idx),
                format!("GET;"),
                format!("ASSERT_SOME;"),
            ]);
        }
    }

    let memory_ptr = memory_ty2stack_ptr.get(primitive_ty).unwrap();
    res.append(&mut vec![
        format!("DIG {};", register2stack_ptr.len() + memory_ptr + depth),
        format!("UNPAIR;"),
        format!("DIG 3;"),
        format!("DIG 3;"),
        format!("UPDATE;"),
        format!("PAIR;"),
        format!(
            "DUG {};",
            register2stack_ptr.len() + memory_ptr + (depth - 1) - 1
        ),
    ]);

    res
}
