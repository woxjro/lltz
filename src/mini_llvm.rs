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
    I1,
}

pub enum Opcode {
    Add,
    Sub,
    Mul,
}

pub enum Condition {
    Eq,  //equal
    Ne,  //not equal
    Ugt, //unsigned greater than
    Uge, //unsigned greater or equal
    Ult, //unsigned less than
    Ule, //unsigned less or equal
    Sgt, //signed greater than
    Sge, //signed greater or equal
    Slt, //signed less than
    Sle, //signed less or equal
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
    Icmp {
        result: Register,
        cond: Condition,
        ty: Type,
        op1: Register,
        op2: Register,
    },
    Ret {
        ty: Type,
        reg: Register,
    },
}
