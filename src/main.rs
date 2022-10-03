use std::collections::{HashMap, HashSet};
struct Register {
    id: String,
}

#[derive(Hash, Eq, PartialEq, Debug)]
enum Type {
    I32,
}

enum Opcode {
    Add,
    Sub,
    Mul,
}

enum Instruction {
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

fn compile(instructions: Vec<Instruction>) -> String {
    //レジスタの下処理
    let mut register2stack_ptr = HashMap::new();
    let mut memory_types = HashSet::new();
    let mut stack_ptr = 0;
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
                code_block_t,
                code_block_f,
            } => {
                // TODO:  Code Blockの中のレジスタも調べる
                let _ = register2stack_ptr.entry(&reg.id).or_insert_with(|| {
                    stack_ptr += 1;
                    stack_ptr
                });
            }
            Instruction::Whilez { reg, code_block } => {
                // TODO:  Code Blockの中のレジスタも調べる
                let _ = register2stack_ptr.entry(&reg.id).or_insert_with(|| {
                    stack_ptr += 1;
                    stack_ptr
                });
            }
            Instruction::Op {
                ty,
                opcode,
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
            Instruction::Ret { ty, reg } => {
                let _ = register2stack_ptr.entry(&reg.id).or_insert_with(|| {
                    stack_ptr += 1;
                    stack_ptr
                });
            }
        };
    }

    //dbg!(&register2stack_ptr);
    //dbg!(&memory_types);

    let mut michelson_code = String::new();
    let space = "      ";

    michelson_code = format!("{michelson_code}{space}DROP;\n");

    for ty in memory_types.iter() {
        let ty_str = match ty {
            I32 => "int",
        };

        michelson_code = format!("{michelson_code}{space}EMPTY_BIG_MAP string {ty_str};\n");
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

    format!("parameter unit;\nstorage unit;\ncode {{\n{michelson_code}}}")
}

fn main() {
    //define dso_local i32 @main() #0 {
    //  %1 = alloca i32, align 4
    //  %2 = alloca i32, align 4
    //  %3 = alloca i32, align 4
    //  %4 = alloca i32, align 4
    //  store i32 0, i32* %1, align 4
    //  store i32 10, i32* %2, align 4
    //  store i32 20, i32* %3, align 4
    //  %5 = load i32, i32* %2, align 4
    //  %6 = load i32, i32* %3, align 4
    //  %7 = add nsw i32 %5, %6
    //  store i32 %7, i32* %4, align 4
    //  ret i32 0
    //}

    //{{
    let instr1 = Instruction::Alloca {
        reg: Register {
            id: "%1".to_string(),
        },
        ty: Type::I32,
    };

    let instr2 = Instruction::Alloca {
        reg: Register {
            id: "%2".to_string(),
        },
        ty: Type::I32,
    };
    let instr3 = Instruction::Alloca {
        reg: Register {
            id: "%3".to_string(),
        },
        ty: Type::I32,
    };
    let instr4 = Instruction::Alloca {
        reg: Register {
            id: "%4".to_string(),
        },
        ty: Type::I32,
    };

    let instr5 = Instruction::Store {
        src: Register {
            id: "0".to_string(),
        },
        ptr: Register {
            id: "%1".to_string(),
        },
    };

    let instr6 = Instruction::Store {
        src: Register {
            id: "10".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };
    let instr7 = Instruction::Store {
        src: Register {
            id: "20".to_string(),
        },
        ptr: Register {
            id: "%3".to_string(),
        },
    };

    let instr8 = Instruction::Load {
        dst: Register {
            id: "%5".to_string(),
        },
        ptr: Register {
            id: "%2".to_string(),
        },
    };
    let instr9 = Instruction::Load {
        dst: Register {
            id: "%6".to_string(),
        },
        ptr: Register {
            id: "%3".to_string(),
        },
    };
    let instr10 = Instruction::Op {
        ty: Type::I32,
        opcode: Opcode::Add,
        dst: Register {
            id: "%7".to_string(),
        },
        reg1: Register {
            id: "%5".to_string(),
        },
        reg2: Register {
            id: "%6".to_string(),
        },
    };
    let instr11 = Instruction::Store {
        src: Register {
            id: "%7".to_string(),
        },
        ptr: Register {
            id: "%4".to_string(),
        },
    };

    let instr12 = Instruction::Ret {
        ty: Type::I32,
        reg: Register {
            id: "0".to_string(),
        },
    };
    //}}

    let instructions = vec![
        instr1, instr2, instr3, instr4, instr5, instr6, instr7, instr8, instr9, instr10, instr11,
        instr12,
    ];

    let michelson_code = compile(instructions);
    println!("{}", michelson_code);
}
