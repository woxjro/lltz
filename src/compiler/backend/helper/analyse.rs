use crate::mini_llvm::Type;
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

    match ty.clone() {
        Type::Struct { id: _, fields } => {
            //先に子Fieldを登録する
            for field in fields {
                self::analyse_memory4alloca(field, memory_ty2stack_ptr, memory_ptr);
            }
            let _ = memory_ty2stack_ptr.entry(ty.clone()).or_insert_with(|| {
                *memory_ptr += 1;
                *memory_ptr
            });
        }
        _ => {
            let _ = memory_ty2stack_ptr.entry(ty.clone()).or_insert_with(|| {
                *memory_ptr += 1;
                *memory_ptr
            });
        }
    }
}
