use crate::compiler::utils;
use crate::mini_llvm::{Register, Type};
use std::collections::HashMap;

///Struct型の場合は内部にも, メモリの型を 保持している
///（ケースがほとんどである）ので再帰的に調べる
pub fn analyse_memory4alloca(
    ty: Type,
    memory_ty2stack_ptr: &mut HashMap<Type, usize>,
    memory_ptr: &mut usize,
) {
    //既にtyが登録されていたらexit
    match memory_ty2stack_ptr.get(&ty) {
        Some(_) => return,
        _ => {}
    };

    //tyはまずメモリに（無ければ）登録する
    let _ = memory_ty2stack_ptr.entry(ty.clone()).or_insert_with(|| {
        *memory_ptr += 1;
        *memory_ptr
    });

    match ty {
        Type::Struct { id: _, fields } => {
            for field in fields {
                self::analyse_memory4alloca(field, memory_ty2stack_ptr, memory_ptr);
            }
        }
        _ => {}
    }
}

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
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    let memory_ptr = memory_ty2stack_ptr.get(ty).unwrap();

    let michelson_instructions = match ty {
        Type::Struct { id, fields } => {
            exec_struct_alloca(id, fields, ptr, register2stack_ptr, memory_ty2stack_ptr)
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
    utils::format(&michelson_instructions, tab, tab_depth)
}

///Struct型のAllocaを実行する
///Struct { id, fields } 型をallocaし, その領域へのポインタをptrへと格納する命令を生成する
///```llvm
///%ptr = alloca Struct { id, fields };
///```
fn exec_struct_alloca(
    id: &str,
    fields: &Vec<Type>,
    ptr: &Register,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> Vec<String> {
    //Struct { id, fields }型のメモリ領域のスタック上の相対ポインタ
    let memory_ptr = memory_ty2stack_ptr
        .get(&Type::Struct {
            id: id.to_string(),
            fields: fields.clone(),
        })
        .unwrap();
    let mut res = vec![format!("###alloca {{"), format!("EMPTY_MAP int int;")];
    for (idx, field) in fields.iter().enumerate() {
        res.append(&mut vec![format!("###alloca for field No.{idx} {{")]);
        res.append(&mut exec_struct_field_alloca(
            idx,
            field,
            1,
            memory_ptr,
            register2stack_ptr,
            memory_ty2stack_ptr,
        ));
        res.append(&mut vec![format!("###}}")]);
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
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> Vec<String> {
    let field_memory_ptr = memory_ty2stack_ptr.get(field).unwrap();
    match field {
        Type::Struct { id: _, fields } => {
            let mut res = vec![format!("EMPTY_MAP int int;")];
            for (child_field_idx, child_field) in fields.iter().enumerate() {
                res.append(&mut vec![format!(
                    "###alloca for field No.{child_field_idx} {{"
                )]);
                res.append(&mut self::exec_struct_field_alloca(
                    child_field_idx,
                    child_field,
                    depth + 1,
                    memory_ptr,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                ));
                res.append(&mut vec![format!("###}}")]);
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
                format!("PUSH int -1;"),
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
                    format!("DUP {};", register2stack_ptr.len() + memory_ptr + depth - 1),
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
