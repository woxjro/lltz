use super::helper;
use crate::lltz_ir::{Arg, Function, InnerType, Register, Type};
use michelson_ast::wrapped_instruction::WrappedInstruction as MWrappedInstr;
use std::collections::HashMap;
///ここが終わった段階ではMichelson StackのTopに(Parameter, Storage)が乗っている
pub fn inject_storage(
    smart_contract_function: &Function,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
) -> Vec<MWrappedInstr> {
    let storage_arg = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(&ty) {
            Type::Struct { id, fields: _ } => id == String::from("Storage"),
            _ => false,
        })
        .unwrap();
    helper::storage::alloca_storage_by_value(storage_arg, &register2stack_ptr, &memory_ty2stack_ptr)
}

///ここが終わった段階では(Parameter, Strorage)はもう要らないのでDROP.
pub fn inject_parameter(
    smart_contract_function: &Function,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
) -> Vec<MWrappedInstr> {
    let parameter_arg = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(&ty) {
            Type::Struct { id, fields: _ } => id == String::from("Parameter"),
            _ => false,
        })
        .unwrap();
    helper::parameter::alloca_parameter_by_value(
        parameter_arg,
        &register2stack_ptr,
        &memory_ty2stack_ptr,
    )
}

///スマートコントラクトの返り値として使うPairをAllocaする関数
///（ここでAllocaしたPairをエンコードしてコントラクトの返り値とする）
pub fn inject_pair(
    smart_contract_function: &Function,
    register2stack_ptr: &HashMap<Register, usize>,
    memory_ty2stack_ptr: &HashMap<InnerType, usize>,
) -> Vec<MWrappedInstr> {
    let pair_arg = smart_contract_function
        .argument_list
        .iter()
        .find(|Arg { reg: _, ty }| match Type::deref(&ty) {
            Type::Struct { id, fields: _ } => id == String::from("Pair"),
            _ => false,
        })
        .unwrap();
    helper::alloca::exec_alloca(
        &pair_arg.reg,
        &Type::deref(&pair_arg.ty),
        &register2stack_ptr,
        &memory_ty2stack_ptr,
    )
}
