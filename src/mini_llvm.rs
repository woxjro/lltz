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
    Struct { id: String, fields: Vec<Type> },
    Ptr(Box<Type>),
}

impl Type {
    pub fn to_llvm_ty_string(ty: &Type) -> String {
        match ty {
            Type::I1 => "i1".to_string(),
            Type::I32 => "i32".to_string(),
            Type::Struct { id, fields: _ } => format!("%struct.{id}"),
            Type::Ptr(ty) => {
                let inner = Type::to_llvm_ty_string(&*ty);
                format!("{inner}*")
            }
        }
    }

    pub fn to_michelson_ty_string(ty: &Type) -> String {
        let res = match ty {
            Type::I1 => String::from("bool"),
            Type::I32 => String::from("int"),
            Type::Struct { .. } => {
                String::from("(map int int)")
                //map (struct.index, ptr)
            }
            Type::Ptr(_) => String::from("int"),
        };
        res
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

pub enum ReservedStructKind {
    Parameter,
    Storage,
    Operation,
}

#[allow(dead_code)]
pub struct Arg {
    ty: Type,
    reg: Register,
}

pub struct MiniLlvm {
    pub structure_types: Vec<Type>, //構造体宣言
    pub functions: Vec<Function>,
}

///define [linkage] [PreemptionSpecifier] [visibility] [DLLStorageClass]
///       [cconv] [ret attrs]
///       <ResultType> @<FunctionName> ([argument list])
///       [(unnamed_addr|local_unnamed_addr)] [AddrSpace] [fn Attrs]
///       [section "name"] [partition "name"] [comdat [($name)]] [align N]
///       [gc] [prefix Constant] [prologue Constant] [personality Constant]
///       (!name !N)* { ... }
pub struct Function {
    pub function_name: String,
    pub result_type: Type,
    pub argument_list: Vec<Arg>,
    pub instructions: Vec<Instruction>,
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
    GetElementPtr {
        result: Register,
        //If the inbounds keyword is present, the result value of
        //the getelementptr is a poison value
        //inboudns
        ty: Type,
        ptrval: Register,
        subsequent: Vec<(Type, Register)>,
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
    // TODO:  その他の任意フィールドの実装
    //<result> = [tail | musttail | notail ] call [fast-math flags] [cconv] [ret attrs]
    //           [addrspace(<num>)] <ty>|<fnty> <fnptrval>(<function args>) [fn attrs]
    //           [ operand bundles ]
    Call {
        result: Register,
        fnty: Type,
        fnptrval: Register,
        function_args: Vec<Arg>,
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
    //Structなどのポインタを受け取り, srcからdstへと再帰的にコピーする
    //%5 = bitcast %struct.Storage* %4 to i8*
    //%6 = bitcast %struct.Storage* %2 to i8*
    //call void @llvm.memcpy.p0i8.p0i8.i64(
    //     i8* align 4 %5, i8* align 8 %6, i64 36, i1 false)
    LlvmMemcpy {
        dest: Register,
        src: Register,
        ty: Type, //ポインタdest,srcが指す中身の型
    },
    Ret {
        ty: Type,
        value: Register,
    },
}
