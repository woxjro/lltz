pub mod mini_llvm {
    pub struct Register {
        pub id: String,
    }

    #[derive(Hash, Eq, PartialEq, Debug)]
    pub enum Type {
        I32,
    }

    pub enum Opcode {
        Add,
        Sub,
        Mul,
    }

    pub enum Instruction {
        Alloca {
            reg: Register,
            ty: Type,
        },
        Store {
            src: Register,
            ptr: Register,
        },
        Load {
            dst: Register,
            ptr: Register,
        },
        Ifz {
            reg: Register,
            code_block_t: Vec<Instruction>,
            code_block_f: Vec<Instruction>,
        },
        Whilez {
            reg: Register,
            code_block: Vec<Instruction>,
        },
        Op {
            ty: Type,
            opcode: Opcode,
            dst: Register,
            reg1: Register,
            reg2: Register,
        },
        Ret {
            ty: Type,
            reg: Register,
        },
    }
} /* mini_llvm */

pub mod compiler {
    use super::mini_llvm::{Instruction, Opcode, Type};
    use std::collections::{HashMap, HashSet};
    pub fn compile(instructions: Vec<Instruction>) -> String {
        //レジスタの下処理
        let mut register2stack_ptr = HashMap::new();
        //let mut ty2memory_ptr = HashMap::new();
        //let mut memory_ptr = 0;
        let mut memory_types = HashSet::new();
        let mut stack_ptr = 0;

        // TODO: 今の所単相のコンパイルのみ考えているため
        // bit_map int intへのポインタは定数として扱う
        let BM_I32 = 1; //bit_map int int へのポインタ
        for instruction in &instructions {
            match instruction {
                Instruction::Alloca { reg, ty } => {
                    let _ = register2stack_ptr.entry(&reg.id).or_insert_with(|| {
                        stack_ptr += 1;
                        stack_ptr
                    });
                    memory_types.insert(ty);
                }
                Instruction::Store { src, ptr } => {
                    let _ = register2stack_ptr.entry(&src.id).or_insert_with(|| {
                        stack_ptr += 1;
                        stack_ptr
                    });
                    let _ = register2stack_ptr.entry(&ptr.id).or_insert_with(|| {
                        stack_ptr += 1;
                        stack_ptr
                    });
                }
                Instruction::Load { dst, ptr } => {
                    let _ = register2stack_ptr.entry(&dst.id).or_insert_with(|| {
                        stack_ptr += 1;
                        stack_ptr
                    });
                    let _ = register2stack_ptr.entry(&ptr.id).or_insert_with(|| {
                        stack_ptr += 1;
                        stack_ptr
                    });
                }
                Instruction::Ifz {
                    reg,
                    code_block_t: _,
                    code_block_f: _,
                } => {
                    // TODO:  Code Blockの中のレジスタも調べる
                    let _ = register2stack_ptr.entry(&reg.id).or_insert_with(|| {
                        stack_ptr += 1;
                        stack_ptr
                    });
                }
                Instruction::Whilez { reg, code_block: _ } => {
                    // TODO:  Code Blockの中のレジスタも調べる
                    let _ = register2stack_ptr.entry(&reg.id).or_insert_with(|| {
                        stack_ptr += 1;
                        stack_ptr
                    });
                }
                Instruction::Op {
                    ty: _,
                    opcode: _,
                    dst,
                    reg1,
                    reg2,
                } => {
                    let _ = register2stack_ptr.entry(&dst.id).or_insert_with(|| {
                        stack_ptr += 1;
                        stack_ptr
                    });
                    let _ = register2stack_ptr.entry(&reg1.id).or_insert_with(|| {
                        stack_ptr += 1;
                        stack_ptr
                    });
                    let _ = register2stack_ptr.entry(&reg2.id).or_insert_with(|| {
                        stack_ptr += 1;
                        stack_ptr
                    });
                }
                Instruction::Ret { ty: _, reg } => {
                    let _ = register2stack_ptr.entry(&reg.id).or_insert_with(|| {
                        stack_ptr += 1;
                        stack_ptr
                    });
                }
            };
        }

        dbg!(&register2stack_ptr);
        //dbg!(&memory_types);

        let mut michelson_code = String::new();
        let space = "       ";

        michelson_code = format!("{michelson_code}{space}DROP;\n");

        for ty in memory_types.iter() {
            let ty_str = match ty {
                Type::I32 => "int",
            };

            michelson_code = format!("{michelson_code}{space}PUSH int 0;\n");
            michelson_code = format!("{michelson_code}{space}EMPTY_BIG_MAP int {ty_str};\n");
            michelson_code = format!("{michelson_code}{space}PAIR;\n");
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
            michelson_code = format!("{michelson_code}{space}PUSH int {val}; # {comment}\n");
        }

        /* Body */
        for instruction in &instructions {
            match instruction {
                Instruction::Alloca { reg, ty: _ } => {
                    michelson_code = format!("{michelson_code}{space}###alloca {{\n");
                    michelson_code = format!(
                        "{michelson_code}{space}DIG {};\n",
                        register2stack_ptr.len() + BM_I32 - 1
                    );
                    michelson_code = format!("{michelson_code}{space}UNPAIR;\n");
                    michelson_code = format!("{michelson_code}{space}SWAP;\n");
                    michelson_code = format!("{michelson_code}{space}PUSH int 1;\n");
                    michelson_code = format!("{michelson_code}{space}ADD;\n");
                    michelson_code = format!("{michelson_code}{space}DUP;\n");
                    michelson_code = format!("{michelson_code}{space}DUP;\n");
                    michelson_code = format!("{michelson_code}{space}DIG 3;\n");
                    michelson_code = format!("{michelson_code}{space}SWAP;\n");
                    michelson_code =
                        format!("{michelson_code}{space}PUSH int -1; # default value\n");
                    michelson_code = format!("{michelson_code}{space}SOME;\n");
                    michelson_code = format!("{michelson_code}{space}SWAP;\n");
                    michelson_code = format!("{michelson_code}{space}UPDATE;\n");
                    michelson_code = format!("{michelson_code}{space}PAIR;\n");
                    michelson_code = format!(
                        "{michelson_code}{space}DUG {};\n",
                        register2stack_ptr.len() + BM_I32
                    );
                    michelson_code = format!(
                        "{michelson_code}{space}DIG {};\n",
                        register2stack_ptr.get(&reg.id).unwrap()
                    );
                    michelson_code = format!("{michelson_code}{space}DROP;\n");
                    michelson_code = format!(
                        "{michelson_code}{space}DUG {};\n",
                        register2stack_ptr.get(&reg.id).unwrap() - 1
                    );
                    michelson_code = format!("{michelson_code}{space}###}}\n");
                }
                Instruction::Store { src, ptr } => {
                    michelson_code = format!("{michelson_code}{space}###store {{\n");
                    michelson_code = format!(
                        "{michelson_code}{space}DUP {};\n",
                        register2stack_ptr.get(&src.id).unwrap()
                    );
                    michelson_code = format!("{michelson_code}{space}SOME;\n");
                    michelson_code = format!(
                        "{michelson_code}{space}DIG {};\n",
                        register2stack_ptr.len() + BM_I32
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
                        register2stack_ptr.len() + BM_I32 - 1
                    );
                    michelson_code = format!("{michelson_code}{space}###}}\n");
                }
                Instruction::Load { dst, ptr } => {
                    michelson_code = format!("{michelson_code}{space}###load {{\n");
                    michelson_code = format!(
                        "{michelson_code}{space}DUP {};\n",
                        register2stack_ptr.len() + BM_I32
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
                        register2stack_ptr.get(&dst.id).unwrap()
                    );
                    michelson_code = format!("{michelson_code}{space}DROP;\n");
                    michelson_code = format!(
                        "{michelson_code}{space}DUG {};\n",
                        register2stack_ptr.get(&dst.id).unwrap() - 1
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
                    dst,
                    reg1,
                    reg2,
                } => {
                    michelson_code = format!("{michelson_code}{space}###Op {{\n");
                    michelson_code = format!(
                        "{michelson_code}{space}DUP {};\n",
                        register2stack_ptr.get(&reg1.id).unwrap()
                    );
                    michelson_code = format!(
                        "{michelson_code}{space}DUP {};\n",
                        register2stack_ptr.get(&reg2.id).unwrap() + 1
                    );
                    let op = match opcode {
                        Opcode::Add => "ADD",
                        Opcode::Sub => "SUB",
                        Opcode::Mul => "MUL",
                    };
                    michelson_code = format!("{michelson_code}{space}{op};\n");
                    michelson_code = format!(
                        "{michelson_code}{space}DIG {};\n",
                        register2stack_ptr.get(&dst.id).unwrap()
                    );
                    michelson_code = format!("{michelson_code}{space}DROP;\n");
                    michelson_code = format!(
                        "{michelson_code}{space}DUG {};\n",
                        register2stack_ptr.get(&dst.id).unwrap() - 1
                    );
                    michelson_code = format!("{michelson_code}{space}###}}\n");
                }
                Instruction::Ret { ty: _, reg: _ } => {}
            };
        }

        //後処理:レジスタ領域・メモリ領域をDROPする
        for i in 0..(register2stack_ptr.iter().len() + memory_types.iter().len()) {
            if i % 5 == 0 {
                michelson_code = format!("{michelson_code}{space}DROP;");
            } else if i % 5 == 4 {
                michelson_code = format!("{michelson_code}DROP;\n");
            } else {
                michelson_code = format!("{michelson_code}DROP;");
            }
        }
        michelson_code = format!("{michelson_code}\n");
        michelson_code = format!("{michelson_code}{space}UNIT; NIL operation; PAIR;");

        format!("parameter unit;\nstorage unit;\ncode {{\n{michelson_code} }}")
    }
} /* compiler */
