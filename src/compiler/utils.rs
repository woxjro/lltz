use crate::mini_llvm::{Condition, Instruction, Opcode, Register, Type};
use std::collections::HashMap;

pub fn format(michelson_instructions: &Vec<String>, tab: &str, tab_depth: usize) -> String {
    let mut indent = String::new();
    for _ in 0..tab_depth {
        indent.push_str(tab);
    }

    michelson_instructions
        .iter()
        .map(|e| format!("{indent}{e}\n"))
        .collect::<String>()
}

/*
 * デバッグ用にMichelsonのスタックの初期状態を出力する
 */
pub fn print_michelson_initial_stack_status(
    register2stack_ptr: &HashMap<Register, usize>,
    register2ty: &HashMap<Register, Type>,
    memory_ty2stack_ptr: &HashMap<Type, usize>,
) -> String {
    let mut rows = vec![];
    let mut register2stack_ptr_sorted = register2stack_ptr.iter().collect::<Vec<_>>();
    register2stack_ptr_sorted.sort_by(|a, b| (a.1).cmp(b.1));

    for (reg, _ptr) in register2stack_ptr_sorted {
        let ty = register2ty.get(reg).unwrap();
        let val = match ty {
            Type::I32 => {
                if Register::is_const(reg) {
                    //reg.parse::<i32>().unwrap()
                    reg.get_id()
                } else {
                    //0
                    "0".to_string()
                }
            }
            Type::I1 => "False".to_string(),
            Type::Ptr(_) => {
                if Register::is_const(reg) {
                    //reg.parse::<i32>().unwrap()
                    reg.get_id()
                } else {
                    //0
                    "0".to_string()
                }
            }
        };
        let michelson_ty = match ty {
            Type::I32 => "int",
            Type::I1 => "bool",
            Type::Ptr(_) => "int",
        };
        let llvm_ty_string = Type::to_llvm_ty_string(ty);

        let comment = if Register::is_const(reg) {
            format!("for const {val} : {llvm_ty_string}")
        } else {
            let id = reg.get_id();
            format!("for reg {id} : {llvm_ty_string}")
        };
        rows.push(format!("{michelson_ty} {val} # {comment}"));
    }

    let mut memory_ty2stack_ptr_sorted = memory_ty2stack_ptr.iter().collect::<Vec<_>>();
    memory_ty2stack_ptr_sorted.sort_by(|a, b| (a.1).cmp(b.1));
    for (ty, _v) in memory_ty2stack_ptr_sorted.iter() {
        let ty_str = match ty {
            Type::I1 => "bool",
            Type::I32 => "int",
            Type::Ptr(_) => "int",
        };

        let llvm_ty_string = Type::to_llvm_ty_string(ty);
        let comment = format!("memory for {llvm_ty_string}");

        rows.push(format!("( (big_map int {ty_str}), 0 ) # {comment}"));
    }

    let mut res = vec![];
    let max_len = rows.iter().map(|row| row.len()).max().unwrap();
    let bar = String::from_utf8(vec![b'-'; max_len + 4]).unwrap();
    let bottom = String::from_utf8(vec![b'_'; max_len + 4]).unwrap();
    let space = String::from_utf8(vec![b' '; max_len + 4]).unwrap();

    for _ in 0..4 {
        res.push(format!("|{space}|\n"));
    }
    for row in rows {
        let left = (max_len - row.len() + 4) / 2;
        let right = left + (max_len - row.len() + 4) % left;
        let left_space = String::from_utf8(vec![b' '; left]).unwrap();
        let right_space = String::from_utf8(vec![b' '; right]).unwrap();

        res.push(format!("|{bar}|\n"));
        res.push(format!("|{left_space}{row}{right_space}|\n"));
    }
    res.push(format!("|{bottom}|\n"));

    res.concat()
}
