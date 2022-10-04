use crate::mini_llvm::{Instruction, Opcode, Type};
use std::collections::HashMap;

//まず与えられたLLVM IRの命令列を事前に走査して
//命令列に出現しうる型やレジスタの種類・数などを把握する
pub fn analyse_registers_and_memory(
    register2stack_ptr: &mut HashMap<String, usize>,
    memory_types: &mut HashMap<Type, usize>,
    stack_ptr: &mut usize,
    memory_ptr: &mut usize,
    instructions: &Vec<Instruction>,
) {
    for instruction in instructions {
        match instruction {
            Instruction::Alloca { ptr, ty } => {
                let _ = register2stack_ptr.entry(ptr.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                let _ = memory_types.entry(ty.clone()).or_insert_with(|| {
                    *memory_ptr += 1;
                    *memory_ptr
                });
            }
            Instruction::Store { ty, value, ptr } => {
                let _ = register2stack_ptr.entry(value.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                let _ = register2stack_ptr.entry(ptr.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
            }
            Instruction::Load { result, ty, ptr } => {
                let _ = register2stack_ptr
                    .entry(result.get_id())
                    .or_insert_with(|| {
                        *stack_ptr += 1;
                        *stack_ptr
                    });
                let _ = register2stack_ptr.entry(ptr.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
            }
            Instruction::Ifz {
                reg,
                code_block_t,
                code_block_f,
            } => {
                // TODO:  Code Blockの中のレジスタも調べる
                let _ = register2stack_ptr.entry(reg.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                analyse_registers_and_memory(
                    register2stack_ptr,
                    memory_types,
                    stack_ptr,
                    memory_ptr,
                    code_block_t,
                );
                analyse_registers_and_memory(
                    register2stack_ptr,
                    memory_types,
                    stack_ptr,
                    memory_ptr,
                    code_block_f,
                );
            }
            Instruction::Whilez { reg, code_block } => {
                let _ = register2stack_ptr.entry(reg.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                analyse_registers_and_memory(
                    register2stack_ptr,
                    memory_types,
                    stack_ptr,
                    memory_ptr,
                    code_block,
                );
            }
            Instruction::Op {
                ty: _,
                opcode: _,
                result,
                op1,
                op2,
            } => {
                let _ = register2stack_ptr
                    .entry(result.get_id())
                    .or_insert_with(|| {
                        *stack_ptr += 1;
                        *stack_ptr
                    });
                let _ = register2stack_ptr.entry(op1.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                let _ = register2stack_ptr.entry(op2.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
            }
            Instruction::Ret { ty: _, value } => {
                let _ = register2stack_ptr.entry(value.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
            }
        };
    }
}

//レジスタ・メモリ領域などに相当するMichelsonコードをスタックにPUSHする
pub fn prepare(
    michelson_code: String,
    space: &str,
    register2stack_ptr: &mut HashMap<String, usize>,
    memory_types: &mut HashMap<Type, usize>,
) -> String {
    let mut new_michelson_code = format!("{michelson_code}{space}DROP;\n");

    let mut memory_types_sorted = memory_types.iter().collect::<Vec<_>>();
    memory_types_sorted.sort_by(|a, b| (b.1).cmp(a.1));
    for (ty, _v) in memory_types_sorted.iter() {
        let ty_str = match ty {
            Type::I32 => "int",
        };

        new_michelson_code = format!("{new_michelson_code}{space}PUSH int 0;\n");
        new_michelson_code = format!("{new_michelson_code}{space}EMPTY_BIG_MAP int {ty_str};\n");
        new_michelson_code = format!("{new_michelson_code}{space}PAIR;\n");
    }

    let mut register2stack_ptr_sorted = register2stack_ptr.iter().collect::<Vec<_>>();
    register2stack_ptr_sorted.sort_by(|a, b| (b.1).cmp(a.1));

    for (reg_str, _ptr) in register2stack_ptr_sorted {
        let is_const = !reg_str.contains("%");
        let val = if is_const {
            reg_str.parse::<i32>().unwrap()
        } else {
            0
        };
        let comment = if is_const {
            format!("for const {val}")
        } else {
            format!("for reg {reg_str}")
        };
        new_michelson_code = format!("{new_michelson_code}{space}PUSH int {val}; # {comment}\n");
    }
    new_michelson_code
}

pub fn body(
    michelson_code: String,
    space: &str,
    register2stack_ptr: &mut HashMap<String, usize>,
    memory_types: &mut HashMap<Type, usize>,
    instructions: &Vec<Instruction>,
) -> String {
    let mut michelson_code = michelson_code;
    for instruction in instructions {
        match instruction {
            Instruction::Alloca { ptr, ty } => {
                let memory_ptr = memory_types.get(ty).unwrap();
                michelson_code = format!("{michelson_code}{space}###alloca {{\n");
                michelson_code = format!(
                    "{michelson_code}{space}DIG {};\n",
                    register2stack_ptr.len() + memory_ptr - 1
                );
                michelson_code = format!("{michelson_code}{space}UNPAIR;\n");
                michelson_code = format!("{michelson_code}{space}SWAP;\n");
                michelson_code = format!("{michelson_code}{space}PUSH int 1;\n");
                michelson_code = format!("{michelson_code}{space}ADD;\n");
                michelson_code = format!("{michelson_code}{space}DUP;\n");
                michelson_code = format!("{michelson_code}{space}DUP;\n");
                michelson_code = format!("{michelson_code}{space}DIG 3;\n");
                michelson_code = format!("{michelson_code}{space}SWAP;\n");
                michelson_code = format!("{michelson_code}{space}PUSH int -1; # default value\n");
                michelson_code = format!("{michelson_code}{space}SOME;\n");
                michelson_code = format!("{michelson_code}{space}SWAP;\n");
                michelson_code = format!("{michelson_code}{space}UPDATE;\n");
                michelson_code = format!("{michelson_code}{space}PAIR;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.len() + memory_ptr
                );
                michelson_code = format!(
                    "{michelson_code}{space}DIG {};\n",
                    register2stack_ptr.get(&ptr.id).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}DROP;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.get(&ptr.id).unwrap() - 1
                );
                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
            Instruction::Store { ty, value, ptr } => {
                let memory_ptr = memory_types.get(ty).unwrap();
                michelson_code = format!("{michelson_code}{space}###store {{\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&value.id).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}SOME;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DIG {};\n",
                    register2stack_ptr.len() + memory_ptr
                );
                michelson_code = format!("{michelson_code}{space}UNPAIR;\n");
                michelson_code = format!("{michelson_code}{space}DIG 2;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&ptr.id).unwrap() + 3
                );
                michelson_code = format!("{michelson_code}{space}UPDATE;\n");
                michelson_code = format!("{michelson_code}{space}PAIR;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.len() + memory_ptr - 1
                );
                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
            Instruction::Load { result, ty, ptr } => {
                let memory_ptr = memory_types.get(ty).unwrap();

                michelson_code = format!("{michelson_code}{space}###load {{\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.len() + memory_ptr
                );
                michelson_code = format!("{michelson_code}{space}CAR;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&ptr.id).unwrap() + 1
                );
                michelson_code = format!("{michelson_code}{space}GET;\n");
                michelson_code = format!("{michelson_code}{space}ASSERT_SOME;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DIG {};\n",
                    register2stack_ptr.get(&result.id).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}DROP;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.get(&result.id).unwrap() - 1
                );
                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
            Instruction::Ifz {
                reg: _,
                code_block_t: _,
                code_block_f: _,
            } => {}
            Instruction::Whilez {
                reg: _,
                code_block: _,
            } => {}
            Instruction::Op {
                ty: _,
                opcode,
                result,
                op1,
                op2,
            } => {
                michelson_code = format!("{michelson_code}{space}###Op {{\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&op1.id).unwrap()
                );
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&op2.id).unwrap() + 1
                );
                let op = match opcode {
                    Opcode::Add => "ADD",
                    Opcode::Sub => "SUB",
                    Opcode::Mul => "MUL",
                };
                michelson_code = format!("{michelson_code}{space}{op};\n");
                michelson_code = format!(
                    "{michelson_code}{space}DIG {};\n",
                    register2stack_ptr.get(&result.id).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}DROP;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.get(&result.id).unwrap() - 1
                );
                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
            Instruction::Ret { ty: _, value: _ } => {}
        };
    }
    michelson_code
}

pub fn exit(
    michelson_code: String,
    space: &str,
    register2stack_ptr: &mut HashMap<String, usize>,
    memory_types: &mut HashMap<Type, usize>,
) -> String {
    let mut new_michelson_code = michelson_code;
    //後処理:レジスタ領域・メモリ領域をDROPする
    for i in 0..(register2stack_ptr.iter().len() + memory_types.iter().len()) {
        if i % 5 == 0 {
            new_michelson_code = format!("{new_michelson_code}{space}DROP;");
        } else if i % 5 == 4 {
            new_michelson_code = format!("{new_michelson_code}DROP;\n");
        } else {
            new_michelson_code = format!("{new_michelson_code}DROP;");
        }
    }
    new_michelson_code = format!("{new_michelson_code}\n");
    new_michelson_code = format!("{new_michelson_code}{space}UNIT; NIL operation; PAIR;");

    format!("parameter unit;\nstorage unit;\ncode {{\n{new_michelson_code} }}")
}
