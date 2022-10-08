use crate::mini_llvm::{Condition, Instruction, Opcode, Register, Type};
use std::collections::HashMap;

//まず与えられたLLVM IRの命令列を事前に走査して
//命令列に出現しうる型やレジスタの種類・数などを把握する
pub fn analyse_registers_and_memory(
    register2stack_ptr: &mut HashMap<Register, usize>,
    register2ty: &mut HashMap<Register, Type>,
    memory_ty2stack_ptr: &mut HashMap<Type, usize>,
    stack_ptr: &mut usize,
    memory_ptr: &mut usize,
    instructions: &Vec<Instruction>,
) {
    for instruction in instructions {
        match instruction {
            Instruction::Alloca { ptr, ty } => {
                let _ = register2stack_ptr.entry(ptr.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                //NOTE: ptrはType::Ptr(ty)のポインタ型であることに注意
                register2ty
                    .entry(ptr.clone())
                    .or_insert(Type::Ptr(Box::new(ty.clone())));

                let _ = memory_ty2stack_ptr.entry(ty.clone()).or_insert_with(|| {
                    *memory_ptr += 1;
                    *memory_ptr
                });
            }
            Instruction::Store { ty, value, ptr } => {
                let _ = register2stack_ptr.entry(value.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                //即値を仮想レジスタとして扱うのでこの処理は（少なくとも今は）必要
                //NOTE: ptrはType::Ptr(ty)のポインタ型であることに注意
                register2ty
                    .entry(ptr.clone())
                    .or_insert(Type::Ptr(Box::new(ty.clone())));

                //即値を仮想レジスタとして扱うのでこの処理は（少なくとも今は）必要
                register2ty.entry(value.clone()).or_insert(ty.clone());
                let _ = register2stack_ptr.entry(ptr.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
            }
            Instruction::Load { result, ty, ptr } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });

                //NOTE: ptrはType::Ptr(ty)のポインタ型であることに注意
                register2ty
                    .entry(ptr.clone())
                    .or_insert(Type::Ptr(Box::new(ty.clone())));

                register2ty.entry(result.clone()).or_insert(ty.clone());

                let _ = register2stack_ptr.entry(ptr.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
            }
            Instruction::If {
                reg,
                code_block_t,
                code_block_f,
            } => {
                let _ = register2stack_ptr.entry(reg.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                register2ty.entry(reg.clone()).or_insert(Type::I1);
                analyse_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    stack_ptr,
                    memory_ptr,
                    code_block_t,
                );
                analyse_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
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
                    memory_ty2stack_ptr,
                    stack_ptr,
                    memory_ptr,
                    cond_block,
                );
                analyse_registers_and_memory(
                    register2stack_ptr,
                    register2ty,
                    memory_ty2stack_ptr,
                    stack_ptr,
                    memory_ptr,
                    loop_block,
                );
            }
            Instruction::Call { .. } => {
                todo!()
            }
            Instruction::Op {
                ty,
                opcode: _,
                result,
                op1,
                op2,
            } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                let _ = register2stack_ptr.entry(op1.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                let _ = register2stack_ptr.entry(op2.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                register2ty.entry(op1.clone()).or_insert(ty.clone());
                register2ty.entry(op2.clone()).or_insert(ty.clone());
                register2ty.entry(result.clone()).or_insert(ty.clone());
            }
            Instruction::Ret { ty, value } => {
                let _ = register2stack_ptr.entry(value.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                register2ty.entry(value.clone()).or_insert(ty.clone());
            }
            Instruction::Icmp {
                result,
                cond: _,
                ty,
                op1,
                op2,
            } => {
                let _ = register2stack_ptr.entry(result.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                register2ty.entry(result.clone()).or_insert(ty.clone());
                let _ = register2stack_ptr.entry(op1.clone()).or_insert_with(|| {
                    *stack_ptr += 1;
                    *stack_ptr
                });
                let _ = register2stack_ptr.entry(op2.clone()).or_insert_with(|| {
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
    register2stack_ptr: &mut HashMap<Register, usize>,
    register2ty: &mut HashMap<Register, Type>,
    memory_ty2stack_ptr: &mut HashMap<Type, usize>,
) -> String {
    let mut new_michelson_code = format!("{michelson_code}{space}DROP;\n");

    let mut memory_ty2stack_ptr_sorted = memory_ty2stack_ptr.iter().collect::<Vec<_>>();
    memory_ty2stack_ptr_sorted.sort_by(|a, b| (b.1).cmp(a.1));
    for (ty, _v) in memory_ty2stack_ptr_sorted.iter() {
        let ty_str = match ty {
            Type::I1 => "bool",
            Type::I32 => "int",
            Type::Ptr(_) => "int",
        };

        let llvm_ty_string = Type::to_llvm_ty_string(ty);
        let comment = format!("memory for {llvm_ty_string}");

        new_michelson_code = format!("{new_michelson_code}{space}PUSH int 0;\n");
        new_michelson_code =
            format!("{new_michelson_code}{space}EMPTY_BIG_MAP int {ty_str}; # {comment}\n");
        new_michelson_code = format!("{new_michelson_code}{space}PAIR;\n");
    }

    let mut register2stack_ptr_sorted = register2stack_ptr.iter().collect::<Vec<_>>();
    register2stack_ptr_sorted.sort_by(|a, b| (b.1).cmp(a.1));

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
        new_michelson_code =
            format!("{new_michelson_code}{space}PUSH {michelson_ty} {val}; # {comment}\n");
    }
    new_michelson_code
}

pub fn body(
    michelson_code: String,
    space: &str,
    register2stack_ptr: &mut HashMap<Register, usize>,
    memory_ty2stack_ptr: &mut HashMap<Type, usize>,
    instructions: &Vec<Instruction>,
) -> String {
    let mut michelson_code = michelson_code;
    for instruction in instructions {
        match instruction {
            Instruction::Alloca { ptr, ty } => {
                let memory_ptr = memory_ty2stack_ptr.get(ty).unwrap();
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
                    register2stack_ptr.get(&ptr).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}DROP;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.get(&ptr).unwrap() - 1
                );
                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
            Instruction::Store { ty, value, ptr } => {
                let memory_ptr = memory_ty2stack_ptr.get(ty).unwrap();
                michelson_code = format!("{michelson_code}{space}###store {{\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&value).unwrap()
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
                    register2stack_ptr.get(&ptr).unwrap() + 3
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
                let memory_ptr = memory_ty2stack_ptr.get(ty).unwrap();

                michelson_code = format!("{michelson_code}{space}###load {{\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.len() + memory_ptr
                );
                michelson_code = format!("{michelson_code}{space}CAR;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&ptr).unwrap() + 1
                );
                michelson_code = format!("{michelson_code}{space}GET;\n");
                michelson_code = format!("{michelson_code}{space}ASSERT_SOME;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DIG {};\n",
                    register2stack_ptr.get(&result).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}DROP;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.get(&result).unwrap() - 1
                );
                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
            Instruction::If {
                reg,
                code_block_t,
                code_block_f,
            } => {
                let space2 = format!("{space}{space}");
                michelson_code = format!("{michelson_code}{space}###If {{\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&reg).unwrap()
                );
                let michelson_code_block_t = body(
                    String::new(),
                    &space2,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                    code_block_t,
                );
                let michelson_code_block_f = body(
                    String::new(),
                    &space2,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
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
                let space2 = format!("{space}{space}");
                let michelson_cond_block = body(
                    String::new(),
                    space,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                    cond_block,
                );

                // FIXME: インデントを揃えるために上とほぼ同じものを生成している
                let michelson_cond_block_used_in_loop = body(
                    String::new(),
                    &space2,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                    cond_block,
                );

                let michelson_loop_block = body(
                    String::new(),
                    &space2,
                    register2stack_ptr,
                    memory_ty2stack_ptr,
                    loop_block,
                );

                michelson_code = format!("{michelson_code}{space}###While {{\n");
                michelson_code = format!("{michelson_code}{michelson_cond_block}");
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&cond).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}LOOP {{\n");
                michelson_code = format!("{michelson_code}{michelson_loop_block}");
                michelson_code = format!("{michelson_code}{michelson_cond_block_used_in_loop}");
                michelson_code = format!(
                    "{michelson_code}{space2}DUP {};\n",
                    register2stack_ptr.get(&cond).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}     }};\n");

                michelson_code = format!("{michelson_code}{space}###}}\n");
            }
            Instruction::Call { .. } => {
                todo!()
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
                    register2stack_ptr.get(&op2).unwrap()
                );
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&op1).unwrap() + 1
                );
                let op = match opcode {
                    Opcode::Add => "ADD",
                    Opcode::Sub => "SUB",
                    Opcode::Mul => "MUL",
                };
                michelson_code = format!("{michelson_code}{space}{op};\n");
                michelson_code = format!(
                    "{michelson_code}{space}DIG {};\n",
                    register2stack_ptr.get(&result).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}DROP;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.get(&result).unwrap() - 1
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
                    register2stack_ptr.get(&op1).unwrap()
                );
                michelson_code = format!(
                    "{michelson_code}{space}DUP {};\n",
                    register2stack_ptr.get(&op2).unwrap() + 1
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
                    register2stack_ptr.get(&result).unwrap()
                );
                michelson_code = format!("{michelson_code}{space}DROP;\n");
                michelson_code = format!(
                    "{michelson_code}{space}DUG {};\n",
                    register2stack_ptr.get(&result).unwrap() - 1
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
    register2stack_ptr: &mut HashMap<Register, usize>,
    memory_ty2stack_ptr: &mut HashMap<Type, usize>,
) -> String {
    let mut new_michelson_code = michelson_code;
    //後処理:レジスタ領域・メモリ領域をDROPする
    for i in 0..(register2stack_ptr.iter().len() + memory_ty2stack_ptr.iter().len()) {
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
