pub struct Register {
    pub id: String,
}

impl Register {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
}

#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug)]
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
        ptr: Register,
        ty: Type,
    },
    Load {
        result: Register,
        ty: Type,
        ptr: Register,
    },
    Store {
        ty: Type,
        value: Register,
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
        result: Register,
        ty: Type,
        opcode: Opcode,
        op1: Register,
        op2: Register,
    },
    Ret {
        ty: Type,
        value: Register,
    },
}
