#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct Register {
    id: String,
}

impl Register {
    pub fn get_id(&self) -> String {
        self.id.clone()
    }
    pub fn new(id: &str) -> Self {
        Register {
            id: String::from(id),
        }
    }

    //本来のレジスタではなく、即値の値を入れた仮のレジスタ(const)であるか判定
    pub fn is_const(reg: &Register) -> bool {
        !reg.get_id().contains("%")
    }
}

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Type {
    I1,
    I32,
    Ptr(Box<Type>),
}

impl Type {
    pub fn to_llvm_ty_string(ty: &Type) -> String {
        match ty {
            Type::I1 => "i1".to_string(),
            Type::I32 => "i32".to_string(),
            Type::Ptr(ty) => {
                let inner = Type::to_llvm_ty_string(&*ty);
                format!("{inner}*")
            }
        }
    }
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
    If {
        reg: Register,
        code_block_t: Vec<Instruction>,
        code_block_f: Vec<Instruction>,
    },
    While {
        cond: Register,
        cond_block: Vec<Instruction>,
        loop_block: Vec<Instruction>,
    },
    Op {
        result: Register,
        ty: Type,
        opcode: Opcode,
        op1: Register,
        op2: Register,
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
        value: Register,
    },
}
