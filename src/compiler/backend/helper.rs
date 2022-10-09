use crate::mini_llvm::{Instruction, Register, Type};
use std::collections::HashMap;

//Struct型の場合は内部にも, メモリの型を 保持している
//（ケースがほとんどである）ので再帰的に調べる
pub fn alloca_memory(
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
                self::alloca_memory(field, memory_ty2stack_ptr, memory_ptr);
            }
        }
        _ => {}
    }
}
