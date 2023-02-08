//! LLTZ IR の定義
use michelson_ast::instruction::Instruction as MInstr;
use michelson_ast::ty::Ty as MTy;
use michelson_ast::val::Val as MVal;

///レジスタ
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

///データ型
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum Type {
    Address,
    Bool,
    Int,
    Mutez,
    Nat,
    Contract(Box<Type>),
    Operation,
    Option(Box<Type>),
    Ptr(Box<Type>),
    Struct { id: String, fields: Vec<Type> },
    Array { size: u32, elementtype: Box<Type> },
}

impl Type {
    ///予約語Typeを受け取り, MichelsonのPairを返す.
    ///Storage, Parameter, PairなどといったMichelsonコードの引数を生成するために使う
    pub fn struct_type2michelson_pair(&self) -> MTy {
        match self {
            Type::Struct { id: _, fields } => {
                if fields.len() >= 2 {
                    let mut es = fields
                        .iter()
                        .map(|field| field.struct_type2michelson_pair())
                        .collect::<Vec<_>>();

                    // CAUTION: ty2, ty1の順番はこのまま．逆にしない．
                    let mut res = MTy::Pair {
                        ty2: Box::new(es.pop().unwrap()),
                        ty1: Box::new(es.pop().unwrap()),
                    };

                    while let Some(e) = es.pop() {
                        res = MTy::Pair {
                            ty1: Box::new(e),
                            ty2: Box::new(res.clone()),
                        }
                    }

                    res
                } else if fields.len() == 1 {
                    fields.iter().nth(0).unwrap().struct_type2michelson_pair()
                } else {
                    MTy::Unit
                }
            }
            Type::Array {
                size: _,
                elementtype: _,
            } => todo!(),
            _ => match self {
                Type::Address => self.to_michelson_ty(),
                Type::Array { .. } => panic!(),
                Type::Bool => self.to_michelson_ty(),
                Type::Mutez => self.to_michelson_ty(),
                Type::Int => self.to_michelson_ty(),
                Type::Nat => self.to_michelson_ty(),
                Type::Struct { .. } => {
                    panic!() //never occur
                }
                Type::Contract(ty) => MTy::Contract {
                    ty: Box::new(ty.struct_type2michelson_pair()),
                },
                Type::Operation => self.to_michelson_ty(),
                Type::Ptr(_) => MTy::Int,
                Type::Option(ty) => MTy::Option {
                    ty: Box::new(ty.struct_type2michelson_pair()),
                },
            },
        }
    }

    pub fn to_michelson_ty(&self) -> MTy {
        match self {
            Type::Address => MTy::Address,
            Type::Bool => MTy::Bool,
            Type::Mutez => MTy::Mutez,
            Type::Int => MTy::Int,
            Type::Nat => MTy::Nat,
            Type::Struct { .. } => {
                panic!("Struct 型に対応する michelson プリミティブはありません")
            }
            Type::Contract(child_ty) => MTy::Contract {
                ty: Box::new(child_ty.to_michelson_ty()),
            },
            Type::Operation => MTy::Operation,
            Type::Ptr(_) => MTy::Int,
            Type::Option(child_ty) => MTy::Option {
                ty: Box::new(child_ty.to_michelson_ty()),
            },
            Type::Array { .. } => {
                panic!("Array 型に対応する michelson プリミティブはありません")
            }
        }
    }

    pub fn get_name(ty: &Type) -> String {
        match ty {
            Type::Address => "address".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Mutez => "mutez".to_string(),
            Type::Int => "int".to_string(),
            Type::Nat => "nat".to_string(),
            Type::Struct { id, fields: _ } => format!("%struct.{id}"),
            Type::Array { size, elementtype } => {
                format!("[{} x {}]", size, Type::get_name(&**elementtype))
            }
            Type::Contract(ty) => {
                let inner = Type::get_name(&*ty);
                format!("(%struct.contract {inner})")
            }
            Type::Operation => String::from("%struct.operation"),
            Type::Ptr(ty) => {
                let inner = Type::get_name(&*ty);
                format!("{inner}*")
            }
            Type::Option(ty) => {
                let inner = Type::get_name(&*ty);
                format!("(option {inner})")
            }
        }
    }

    pub fn deref(ty: &Type) -> Type {
        match ty {
            Type::Ptr(inner) => *(inner.clone()),
            _ => panic!(),
        }
    }
}

/// Michelson のスタック上で構築するレジスタ領域及びメモリ領域で使用する型．
/// LLTZ IR に出てくる型を，初期値などを扱いやすくするために，この型へと変換する必要がある．
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub enum BackendType {
    Address,
    Bool,
    Int,
    Mutez,
    Nat,
    Contract(Box<BackendType>),
    Operation,
    Option(Box<BackendType>),
    Ptr(Box<BackendType>),
    Struct {
        id: String,
        fields: Vec<BackendType>,
    },
    Array {
        size: u32,
        elementtype: Box<BackendType>,
    },
}

impl BackendType {
    pub fn from(ty: &Type) -> BackendType {
        match ty {
            Type::Address => BackendType::Option(Box::new(BackendType::Address)),
            Type::Bool => BackendType::Bool,
            Type::Int => BackendType::Int,
            Type::Mutez => BackendType::Mutez,
            Type::Nat => BackendType::Nat,
            Type::Ptr(inner) => BackendType::Ptr(Box::new(BackendType::from(inner))),
            Type::Struct { id, fields } => BackendType::Struct {
                id: id.clone(),
                fields: fields
                    .iter()
                    .map(|field| BackendType::from(field))
                    .collect::<Vec<BackendType>>(),
            },
            Type::Operation => BackendType::Option(Box::new(BackendType::Operation)),
            Type::Contract(child_ty) => BackendType::Option(Box::new(BackendType::Contract(
                Box::new(BackendType::from(child_ty)),
            ))), //child_tyはStruct型
            Type::Option(child_ty) => BackendType::Option(Box::new(BackendType::from(child_ty))),
            Type::Array { size, elementtype } => BackendType::Array {
                size: *size,
                elementtype: Box::new(BackendType::from(elementtype)),
            },
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            BackendType::Address => self.to_michelson_ty().to_string(),
            BackendType::Bool => self.to_michelson_ty().to_string(),
            BackendType::Mutez => self.to_michelson_ty().to_string(),
            BackendType::Int => self.to_michelson_ty().to_string(),
            BackendType::Nat => self.to_michelson_ty().to_string(),
            BackendType::Struct { .. } => self.struct_type2michelson_pair().to_string(),
            BackendType::Contract(child_ty) => {
                let inner = BackendType::struct_type2michelson_pair(&**child_ty).to_string();
                format!("(contract {inner})")
            }
            BackendType::Operation => self.to_michelson_ty().to_string(),
            BackendType::Ptr(_) => self.to_michelson_ty().to_string(),
            BackendType::Option(child_ty) => {
                let inner = child_ty.to_string();
                format!("(option {inner})")
            }
            BackendType::Array { .. } => panic!(),
        }
    }

    pub fn to_michelson_ty(&self) -> MTy {
        match self {
            BackendType::Address => MTy::Address,
            BackendType::Bool => MTy::Bool,
            BackendType::Mutez => MTy::Mutez,
            BackendType::Int => MTy::Int,
            BackendType::Nat => MTy::Nat,
            BackendType::Struct { .. } => {
                panic!("Struct 型に対応する michelson プリミティブはありません")
            }
            BackendType::Contract(child_ty) => MTy::Contract {
                ty: Box::new(child_ty.to_michelson_ty()),
            },
            BackendType::Operation => MTy::Operation,
            BackendType::Ptr(_) => MTy::Int,
            BackendType::Option(child_ty) => MTy::Option {
                ty: Box::new(child_ty.to_michelson_ty()),
            },
            BackendType::Array { .. } => {
                panic!("Array 型に対応する michelson プリミティブはありません")
            }
        }
    }

    pub fn to_memory_string(&self) -> String {
        match self {
            BackendType::Struct { .. } => String::from("(map int int)"),
            BackendType::Array { .. } => String::from("(map int int)"),
            _ => self.to_string(),
        }
    }

    ///要らない気がする．？
    ///予約語Typeを受け取り, MichelsonのPairを返す.
    ///Storage, Parameter, PairなどといったMichelsonコードの引数を生成するために使う
    fn struct_type2michelson_pair(&self) -> MTy {
        match self {
            BackendType::Struct { id: _, fields } => {
                if fields.len() >= 2 {
                    let mut es = fields
                        .iter()
                        .map(|field| field.struct_type2michelson_pair())
                        .collect::<Vec<_>>();

                    // CAUTION: ty2, ty1の順番はこのまま．逆にしない．
                    let mut res = MTy::Pair {
                        ty2: Box::new(es.pop().unwrap()),
                        ty1: Box::new(es.pop().unwrap()),
                    };

                    while let Some(e) = es.pop() {
                        res = MTy::Pair {
                            ty1: Box::new(e),
                            ty2: Box::new(res.clone()),
                        }
                    }

                    res
                } else if fields.len() == 1 {
                    BackendType::struct_type2michelson_pair(&fields.iter().nth(0).unwrap())
                } else {
                    MTy::Unit
                }
            }
            BackendType::Array {
                size: _,
                elementtype: _,
            } => {
                panic!()
            }
            _ => match self {
                BackendType::Address => self.to_michelson_ty(),
                BackendType::Array { .. } => panic!(),
                BackendType::Bool => self.to_michelson_ty(),
                BackendType::Mutez => self.to_michelson_ty(),
                BackendType::Int => self.to_michelson_ty(),
                BackendType::Nat => self.to_michelson_ty(),
                BackendType::Struct { .. } => {
                    panic!() //never occur
                }
                BackendType::Contract(ty) => MTy::Contract {
                    ty: Box::new(ty.struct_type2michelson_pair()),
                },
                BackendType::Operation => self.to_michelson_ty(),
                BackendType::Ptr(_) => MTy::Int,
                BackendType::Option(ty) => MTy::Option {
                    ty: Box::new(ty.struct_type2michelson_pair()),
                },
            },
        }
    }

    pub fn default_value(ty: &BackendType) -> String {
        let res = match ty {
            BackendType::Address => String::from("NONE address"),
            BackendType::Array {
                size: _,
                elementtype: _,
            } => {
                panic!()
            }
            BackendType::Bool => String::from("False"),
            BackendType::Mutez => String::from("0"),
            BackendType::Int => String::from("0"),
            BackendType::Nat => String::from("0"),
            BackendType::Contract(_) => {
                panic!("BackendType::Contractのdefault_valueはありません.")
            }
            BackendType::Operation => {
                panic!("BackendType::Operationのdefault_valueはありません.")
            }
            BackendType::Struct { .. } => {
                panic!("BackendType::Structのdefault_valueはありません.")
            }
            BackendType::Ptr(_) => String::from("-1"),
            BackendType::Option(child_ty) => {
                let inner = child_ty.clone().to_string();
                format!("NONE {inner}")
            }
        };
        res
    }

    pub fn default_value_instruction(ty: &BackendType) -> MInstr {
        let res = match ty {
            BackendType::Address => MInstr::None {
                ty: Box::new(MTy::Address),
            },
            BackendType::Array {
                size: _,
                elementtype: _,
            } => {
                todo!()
            }
            BackendType::Bool => MInstr::Push {
                ty: MTy::Bool,
                val: MVal::Bool(false),
            },
            BackendType::Mutez => MInstr::Push {
                ty: MTy::Mutez,
                val: MVal::Mutez(0),
            },
            BackendType::Int => MInstr::Push {
                ty: MTy::Int,
                val: MVal::Int(0),
            },
            BackendType::Nat => MInstr::Push {
                ty: MTy::Nat,
                val: MVal::Nat(0),
            },
            BackendType::Contract(_) => {
                todo!()
            }
            BackendType::Operation => {
                todo!()
            }
            BackendType::Struct { .. } => {
                todo!()
            }
            BackendType::Ptr(_) => MInstr::Push {
                ty: MTy::Int,
                val: MVal::Int(-1),
            },
            BackendType::Option(child_ty) => MInstr::None {
                ty: Box::new(child_ty.to_michelson_ty()),
            },
        };
        res
    }

    pub fn get_name(&self) -> String {
        match self {
            BackendType::Address => "address".to_string(),
            BackendType::Array { size, elementtype } => {
                format!("[{} x {}]", size, elementtype.get_name())
            }
            BackendType::Bool => "bool".to_string(),
            BackendType::Mutez => "mutez".to_string(),
            BackendType::Int => "int".to_string(),
            BackendType::Nat => "nat".to_string(),
            BackendType::Struct { id, fields: _ } => format!("%struct.{id}"),
            BackendType::Contract(_) => panic!(),
            BackendType::Operation => panic!(),
            BackendType::Ptr(ty) => {
                let inner = ty.get_name();
                format!("{inner}*")
            }
            BackendType::Option(ty) => match &**ty {
                BackendType::Address => {
                    format!("address")
                }
                BackendType::Contract(child_ty) => {
                    let inner = child_ty.get_name();
                    format!("(%struct.contract {inner})")
                }
                BackendType::Operation => format!("operation"),
                _ => {
                    let inner = ty.get_name();
                    format!("(option {inner})")
                }
            },
        }
    }

    pub fn deref(ty: &BackendType) -> BackendType {
        match ty {
            BackendType::Ptr(inner) => *(inner.clone()),
            _ => panic!(),
        }
    }
}

/// 演算子
pub enum Opcode {
    Add,
    Sub,
    Mul,
}

impl Opcode {
    pub fn to_string(&self) -> String {
        match self {
            Opcode::Add => "add".to_string(),
            Opcode::Sub => "sub".to_string(),
            Opcode::Mul => "mul".to_string(),
        }
    }
}

/// icmp でオペランドに対して適用する条件
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

/// 予約語の構造体
pub enum ReservedStructKind {
    Parameter,
    Storage,
    Operation,
}

/// 関数の引数
pub struct Arg {
    pub ty: Type,
    pub reg: Register,
}

/// LLTZ IR プログラム
pub struct Program {
    pub structure_types: Vec<Type>, //構造体宣言
    pub functions: Vec<Function>,
}

/// 関数
pub struct Function {
    pub function_name: String,
    pub result_type: Type,
    pub argument_list: Vec<Arg>,
    pub instructions: Vec<Instruction>,
}

/// 命令
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
    ///////////////////////////////////////
    ////Michelson Blockcnain Operations////
    ///////////////////////////////////////
    //%n = call i64 @get_amount(), !dbg !43
    //          ↓ + Metadata
    //%n : Mutez = michelsonGetAmount
    MichelsonGetAmount {
        result: Register,
    },
    MichelsonGetBalance {
        result: Register,
    },
    MichelsonGetTotalVotingPower {
        result: Register,
    },
    MichelsonGetLevel {
        result: Register,
    },
    MichelsonGetSender {
        result: Register,
    },
    MichelsonGetSource {
        result: Register,
    },
    MichelsonGetSelfAddress {
        result: Register,
    },
    MichelsonGetSelf {
        result: Register,
    },
    MichelsonContract {
        result: Register,
        ty: Type,
        address: Register,
    },
    MichelsonAssertSome {
        result: Register,
        ty: Type,
        value: Register,
    },
    MichelsonTransferTokens {
        result: Register,
        init: Register,
        tokens: Register,
        contract: Register,
    },
}
