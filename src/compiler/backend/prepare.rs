use super::helper;
use crate::mini_llvm::{Arg, Function, Register, Type};
use std::collections::HashMap;

///ここが終わった段階ではMichelson StackのTopに(Parameter, Storage)が乗っている
pub fn prepare_storage(
    smart_contract_function: &Function,
    michelson_code: String,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    let storage_arg = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(ty) {
            Type::Struct { id, fields: _ } => id == String::from("Storage"),
            _ => false,
        })
        .unwrap();
    format!(
        "{michelson_code}{}",
        helper::storage::alloca_storage_by_value(
            storage_arg,
            tab,
            tab_depth,
            &register2stack_ptr,
            &memory_ty2stack_ptr,
        )
    )
}

///ここが終わった段階では(Parameter, Strorage)はもう要らないのでDROP.
pub fn prepare_parameter(
    smart_contract_function: &Function,
    michelson_code: String,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    let parameter_arg = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(ty) {
            Type::Struct { id, fields: _ } => id == String::from("Parameter"),
            _ => false,
        })
        .unwrap();
    format!(
        "{michelson_code}{}",
        helper::parameter::alloca_parameter_by_value(
            parameter_arg,
            tab,
            tab_depth,
            &register2stack_ptr,
            &memory_ty2stack_ptr,
        )
    )
}

///スマートコントラクトの返り値として使うPairをAllocaする関数
///（ここでAllocaしたPairをエンコードしてコントラクトの返り値とする）
pub fn prepare_pair(
    smart_contract_function: &Function,
    michelson_code: String,
    tab: &str,
    tab_depth: usize,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    let pair_arg = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(ty) {
            Type::Struct { id, fields: _ } => id == String::from("Pair"),
            _ => false,
        })
        .unwrap();
    format!(
        "{michelson_code}{}",
        helper::alloca::exec_alloca(
            &pair_arg.reg,
            &Type::deref(&pair_arg.ty),
            tab,
            tab_depth,
            &register2stack_ptr,
            &memory_ty2stack_ptr,
        )
    )
}
