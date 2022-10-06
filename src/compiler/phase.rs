use crate::mini_llvm::{Condition, Instruction, Opcode, Type};
use std::collections::HashMap;

//まず与えられたLLVM IRの命令列を事前に走査して
//命令列に出現しうる型やレジスタの種類・数などを把握する
pub fn analyse_registers_and_memory(
    register2stack_ptr: &mut HashMap<String, usize>,
    register2ty: &mut HashMap<String, Type>,
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

                //pointer型はi32
                register2ty.entry(ptr.get_id()).or_insert(Type::I32);
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
                //pointer型はi32
                register2ty.entry(ptr.get_id()).or_insert(Type::I32);
                register2ty.entry(value.get_id()).or_insert(ty.clone());
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
                //pointer型はi32
                register2ty.entry(ptr.get_id()).or_insert(Type::I32);
                register2ty.entry(result.get_id()).or_insert(ty.clone());

                let _ = register2stack_ptr.entry(ptr.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
            }
            Instruction::If {
                reg,
                code_block_t,
                code_block_f,
            } => {
                // TODO:  Code Blockの中のレジスタも調べる
                let _ = register2stack_ptr.entry(reg.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                register2ty.entry(reg.get_id()).or_insert(Type::I1);
                analyse_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_types,
                    stack_ptr,
                    memory_ptr,
                    code_block_t,
                );
                analyse_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_types,
                    stack_ptr,
                    memory_ptr,
                    code_block_f,
                );
            }
            Instruction::While {
                cond: _,
                cond_block,
                loop_block,
            } => {
                analyse_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_types,
                    stack_ptr,
                    memory_ptr,
                    cond_block,
                );
                analyse_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_types,
                    stack_ptr,
                    memory_ptr,
                    loop_block,
                );
            }
            Instruction::Op {
                ty,
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
                register2ty.entry(op1.get_id()).or_insert(ty.clone());
                register2ty.entry(op2.get_id()).or_insert(ty.clone());
                register2ty.entry(result.get_id()).or_insert(ty.clone());
            }
            Instruction::Ret { ty, value } => {
                let _ = register2stack_ptr.entry(value.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                register2ty.entry(value.get_id()).or_insert(ty.clone());
            }
            Instruction::Icmp {
                result,
                cond: _,
                ty,
                op1,
                op2,
            } => {
                let _ = register2stack_ptr
                    .entry(result.get_id())
                    .or_insert_with(|| {
                        *stack_ptr += 1;
                        *stack_ptr
                    });
                register2ty.entry(result.get_id()).or_insert(ty.clone());
                let _ = register2stack_ptr.entry(op1.get_id()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                let _ = register2stack_ptr.entry(op2.get_id()).or_insert_with(|| {
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
    register2ty: &mut HashMap<String, Type>,
    memory_types: &mut HashMap<Type, usize>,
) -> String {
    let mut new_michelson_code = format!("{michelson_code}{space}DROP;\n");

    let mut memory_types_sorted = memory_types.iter().collect::<Vec<_>>();
    memory_types_sorted.sort_by(|a, b| (b.1).cmp(a.1));
    for (ty, _v) in memory_types_sorted.iter() {
        let ty_str = match ty {
            Type::I1 => "bool",
            Type::I32 => "int",
        };

        new_michelson_code = format!("{new_michelson_code}{space}PUSH int 0;\n");
        new_michelson_code = format!("{new_michelson_code}{space}EMPTY_BIG_MAP int {ty_str};\n");
        new_michelson_code = format!("{new_michelson_code}{space}PAIR;\n");
    }

    let mut register2stack_ptr_sorted = register2stack_ptr.iter().collect::<Vec<_>>();
    register2stack_ptr_sorted.sort_by(|a, b| (b.1).cmp(a.1));

    for (reg_str, _ptr) in register2stack_ptr_sorted {
        let ty = register2ty.get(reg_str).unwrap();
        let is_const = !reg_str.contains("%");
        let val = match ty {
            Type::I32 => {
                if is_const {
                    //reg_str.parse::<i32>().unwrap()
                    reg_str
                } else {
                    //0
                    "0"
                }
            }
            Type::I1 => "False",
        };
        let michelson_ty = match ty {
            Type::I32 => "int",
            Type::I1 => "bool",
        };
        let comment = if is_const {
            format!("for const {val}")
        } else {
            format!("for reg {reg_str}")
        };
        new_michelson_code =
            format!("{new_michelson_code}{space}PUSH {michelson_ty} {val}; # {comment}\n");
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
                    register2stack_ptr.get(&ptr.get_id()).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}DROP;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.get(&ptr.get_id()).unwrap() - 1
                );
                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
            Instruction::Store { ty, value, ptr } => {
                let memory_ptr = memory_types.get(ty).unwrap();
                michelson_code = format!("{michelson_code}{space}###store {{\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&value.get_id()).unwrap()
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
                    register2stack_ptr.get(&ptr.get_id()).unwrap() + 3
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
                    register2stack_ptr.get(&ptr.get_id()).unwrap() + 1
                );
                michelson_code = format!("{michelson_code}{space}GET;\n");
                michelson_code = format!("{michelson_code}{space}ASSERT_SOME;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DIG {};\n",
                    register2stack_ptr.get(&result.get_id()).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}DROP;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.get(&result.get_id()).unwrap() - 1
                );
                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
            Instruction::If {
                reg,
                code_block_t,
                code_block_f,
            } => {
                michelson_code = format!("{michelson_code}{space}###If {{\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&reg.get_id()).unwrap()
                );
                let michelson_code_block_t = body(
                    String::new(),
                    space,
                    register2stack_ptr,
                    memory_types,
                    code_block_t,
                );
                let michelson_code_block_f = body(
                    String::new(),
                    space,
                    register2stack_ptr,
                    memory_types,
                    code_block_f,
                );

                // TODO, FIXME: 各コードブロックが外のブラケットと揃うようにする
                michelson_code = format!("{michelson_code}{space}IF {{\n");
                michelson_code = format!("{michelson_code}{michelson_code_block_t}");
                michelson_code = format!("{michelson_code}{space}   }}\n");
                michelson_code = format!("{michelson_code}{space}   {{\n");
                michelson_code = format!("{michelson_code}{michelson_code_block_f}");
                michelson_code = format!("{michelson_code}{space}   }};\n");
                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
            Instruction::While {
                cond,
                cond_block,
                loop_block,
            } => {
                let michelson_cond_block = body(
                    String::new(),
                    space,
                    register2stack_ptr,
                    memory_types,
                    cond_block,
                );
                let michelson_loop_block = body(
                    String::new(),
                    space,
                    register2stack_ptr,
                    memory_types,
                    loop_block,
                );

                michelson_code = format!("{michelson_code}{space}###While {{\n");
                michelson_code = format!("{michelson_code}{michelson_cond_block}");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&cond.get_id()).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}LOOP {{\n");
                michelson_code = format!("{michelson_code}{michelson_loop_block}");
                michelson_code = format!("{michelson_code}{michelson_cond_block}");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&cond.get_id()).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}     }};\n");
                michelson_code = format!("{michelson_code}{space}###}}\n");

                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
            Instruction::Op {
                ty: _,
                opcode,
                result,
                op1,
                op2,
            } => {
                //NOTE: 意図的にop2を先にDUPしている(LLVMとの被演算子の順番を揃えるため)
                michelson_code = format!("{michelson_code}{space}###Op {{\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&op2.get_id()).unwrap()
                );
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&op1.get_id()).unwrap() + 1
                );
                let op = match opcode {
                    Opcode::Add => "ADD",
                    Opcode::Sub => "SUB",
                    Opcode::Mul => "MUL",
                };
                michelson_code = format!("{michelson_code}{space}{op};\n");
                michelson_code = format!(
                    "{michelson_code}{space}DIG {};\n",
                    register2stack_ptr.get(&result.get_id()).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}DROP;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.get(&result.get_id()).unwrap() - 1
                );
                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
            Instruction::Ret { ty: _, value: _ } => {}
            Instruction::Icmp {
                result,
                cond,
                ty: _,
                op1,
                op2,
            } => {
                michelson_code = format!("{michelson_code}{space}###Icmp {{\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&op1.get_id()).unwrap()
                );
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&op2.get_id()).unwrap() + 1
                );
                let op = match cond {
                    Condition::Eq => format!("COMPARE;\n{space}EQ"),
                    Condition::Slt => {
                        format!("SUB;\n{space}GT")
                    }
                    _ => format!("COMPARE"),
                };
                michelson_code = format!("{michelson_code}{space}{op};\n");
                //michelson_code = format!("{michelson_code}{space}EQ;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DIG {};\n",
                    register2stack_ptr.get(&result.get_id()).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}DROP;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.get(&result.get_id()).unwrap() - 1
                );
                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
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
