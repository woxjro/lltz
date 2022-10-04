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
