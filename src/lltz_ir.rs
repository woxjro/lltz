//! LLTZ IR の定義
use michelson_ast::instruction::Instruction as MInstr;
use michelson_ast::ty::Ty as MTy;
use michelson_ast::val::Val as MVal;

pub enum Value {
    Register(Register),
    Const(Const),
}

impl Value {
    pub fn to_string(&self) -> String {
        match self {
            Value::Register(register) => register.get_id(),
            Value::Const(cnst) => cnst.to_string(),
        }
    }
}

#[derive(Clone)]
pub enum Const {
    Address(String),
    //BigMap { kty: Box<Ty>, vty: Box<Ty> },
    //Bls12_381_fr,
    //Bls12_381_g1,
    //Bls12_381_g2,
    Bool(bool),
    //Bytes,
    //Chain_id,
    //Contract { ty: Ty },
    Int(i128),
    //Key,
    //Key_hash,
    //Lambda { ty1: Ty, ty2: Ty },
    //List { ty: Type },
    //Map { kty: Ty, vty: Ty },
    Mutez(i128),
    Nat(u128),
    //Never,
    //Operation,
    //Option { ty: Ty },
    //Or{ ty1, ty2},
    //Pair {ty1, ty2},
    //Sapling_state {n},
    //Sapling_transaction {n},
    //Set cty,
    //Signature,
    String(String),
    //Ticket cty,
    //Timepstamp,
    //Unit,
}

impl Const {
    pub fn to_string(&self) -> String {
        match self {
            Const::Address(addr) => addr.clone(),
            Const::Bool(b) => format!("{b}"),
            Const::Int(i) => i.to_string(),
            Const::Mutez(m) => m.to_string(),
            Const::Nat(n) => n.to_string(),
            Const::String(s) => s.to_string(),
        }
    }

    pub fn get_push_instruction(&self) -> MInstr {
        match self {
            Const::Address(addr) => MInstr::Push {
                ty: MTy::Address,
                val: MVal::Address(addr.clone()),
            },
            Const::Bool(b) => MInstr::Push {
                ty: MTy::Bool,
                val: MVal::Bool(*b),
            },
            Const::Int(i) => MInstr::Push {
                ty: MTy::Int,
                val: MVal::Int(*i),
            },
            Const::Mutez(m) => MInstr::Push {
                ty: MTy::Mutez,
                val: MVal::Mutez(*m),
            },
            Const::Nat(n) => MInstr::Push {
                ty: MTy::Nat,
                val: MVal::Nat(*n),
            },
            Const::String(s) => MInstr::Push {
                ty: MTy::String,
                val: MVal::String(s.clone()),
            },
        }
    }

    pub fn has_default_value(&self) -> bool {
        match self {
            Const::Address(_) => false,
            Const::Bool(_) => true,
            Const::Int(_) => true,
            Const::Mutez(_) => true,
            Const::Nat(_) => true,
            Const::String(_) => true,
        }
    }
}

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
    pub fn to_entrypoint_ty(&self) -> MTy {
        match self {
            Type::Struct { id: _, fields } => {
                if fields.len() >= 2 {
                    let mut es = fields
                        .iter()
                        .map(|field| field.to_entrypoint_ty())
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
                    fields.iter().nth(0).unwrap().to_entrypoint_ty()
                } else {
                    MTy::Unit
                }
            }
            Type::Array {
                size: _,
                elementtype: _,
            } => todo!(),
            Type::Contract(ty) => MTy::Contract {
                ty: Box::new(ty.to_entrypoint_ty()),
            },
            Type::Ptr(_) => MTy::Int,
            Type::Option(ty) => MTy::Option {
                ty: Box::new(ty.to_entrypoint_ty()),
            },
            _ => self.to_michelson_ty(),
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
pub enum InnerType {
    Address,
    Bool,
    Int,
    Mutez,
    Nat,
    Contract(Box<InnerType>),
    Operation,
    Option(Box<InnerType>),
    Ptr(Box<InnerType>),
    Struct {
        id: String,
        fields: Vec<InnerType>,
    },
    Array {
        size: u32,
        elementtype: Box<InnerType>,
    },
}

impl InnerType {
    pub fn from(ty: &Type) -> InnerType {
        match ty {
            Type::Address => InnerType::Option(Box::new(InnerType::Address)),
            Type::Bool => InnerType::Bool,
            Type::Int => InnerType::Int,
            Type::Mutez => InnerType::Mutez,
            Type::Nat => InnerType::Nat,
            Type::Ptr(inner) => InnerType::Ptr(Box::new(InnerType::from(inner))),
            Type::Struct { id, fields } => InnerType::Struct {
                id: id.clone(),
                fields: fields
                    .iter()
                    .map(|field| InnerType::from(field))
                    .collect::<Vec<InnerType>>(),
            },
            Type::Operation => InnerType::Option(Box::new(InnerType::Operation)),
            Type::Contract(child_ty) => InnerType::Option(Box::new(InnerType::Contract(Box::new(
                InnerType::from(child_ty),
            )))), //child_tyはStruct型
            Type::Option(child_ty) => InnerType::Option(Box::new(InnerType::from(child_ty))),
            Type::Array { size, elementtype } => InnerType::Array {
                size: *size,
                elementtype: Box::new(InnerType::from(elementtype)),
            },
        }
    }

    pub fn to_memory_ty(&self) -> MTy {
        match self {
            InnerType::Struct { .. } => MTy::Map {
                kty: Box::new(MTy::Int),
                vty: Box::new(MTy::Int),
            },
            InnerType::Array { .. } => MTy::Map {
                kty: Box::new(MTy::Int),
                vty: Box::new(MTy::Int),
            },
            _ => self.to_michelson_ty(),
        }
    }

    ///要らない気がする．？
    ///予約語Typeを受け取り, MichelsonのPairを返す.
    ///Storage, Parameter, PairなどといったMichelsonコードの引数を生成するために使う
    fn to_entrypoint_ty(&self) -> MTy {
        match self {
            InnerType::Struct { id: _, fields } => {
                if fields.len() >= 2 {
                    let mut es = fields
                        .iter()
                        .map(|field| field.to_entrypoint_ty())
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
                    InnerType::to_entrypoint_ty(&fields.iter().nth(0).unwrap())
                } else {
                    MTy::Unit
                }
            }
            InnerType::Array { .. } => {
                panic!()
            }
            InnerType::Contract(ty) => MTy::Contract {
                ty: Box::new(ty.to_entrypoint_ty()),
            },
            InnerType::Ptr(_) => MTy::Int,
            InnerType::Option(ty) => MTy::Option {
                ty: Box::new(ty.to_entrypoint_ty()),
            },
            _ => self.to_michelson_ty(),
        }
    }

    pub fn to_michelson_ty(&self) -> MTy {
        match self {
            InnerType::Address => MTy::Address,
            InnerType::Bool => MTy::Bool,
            InnerType::Mutez => MTy::Mutez,
            InnerType::Int => MTy::Int,
            InnerType::Nat => MTy::Nat,
            InnerType::Struct { .. } => self.to_entrypoint_ty(),
            InnerType::Contract(child_ty) => MTy::Contract {
                ty: Box::new(child_ty.to_michelson_ty()),
            },
            InnerType::Operation => MTy::Operation,
            InnerType::Ptr(_) => MTy::Int,
            InnerType::Option(child_ty) => MTy::Option {
                ty: Box::new(child_ty.to_michelson_ty()),
            },
            InnerType::Array { .. } => {
                panic!("Array 型に対応する michelson プリミティブはありません")
            }
        }
    }

    pub fn default_value(ty: &InnerType) -> String {
        let res = match ty {
            InnerType::Address => String::from("NONE address"),
            InnerType::Array {
                size: _,
                elementtype: _,
            } => {
                panic!()
            }
            InnerType::Bool => String::from("False"),
            InnerType::Mutez => String::from("0"),
            InnerType::Int => String::from("0"),
            InnerType::Nat => String::from("0"),
            InnerType::Contract(_) => {
                panic!("InnerType::Contractのdefault_valueはありません.")
            }
            InnerType::Operation => {
                panic!("InnerType::Operationのdefault_valueはありません.")
            }
            InnerType::Struct { .. } => {
                panic!("InnerType::Structのdefault_valueはありません.")
            }
            InnerType::Ptr(_) => String::from("-1"),
            InnerType::Option(child_ty) => {
                let inner = child_ty.to_michelson_ty().to_string();
                format!("NONE {inner}")
            }
        };
        res
    }

    pub fn default_value_instruction(ty: &InnerType) -> MInstr {
        let res = match ty {
            InnerType::Address => MInstr::None { ty: MTy::Address },
            InnerType::Array {
                size: _,
                elementtype: _,
            } => {
                todo!()
            }
            InnerType::Bool => MInstr::Push {
                ty: MTy::Bool,
                val: MVal::Bool(false),
            },
            InnerType::Mutez => MInstr::Push {
                ty: MTy::Mutez,
                val: MVal::Mutez(0),
            },
            InnerType::Int => MInstr::Push {
                ty: MTy::Int,
                val: MVal::Int(0),
            },
            InnerType::Nat => MInstr::Push {
                ty: MTy::Nat,
                val: MVal::Nat(0),
            },
            InnerType::Contract(_) => {
                todo!()
            }
            InnerType::Operation => {
                todo!()
            }
            InnerType::Struct { .. } => {
                todo!()
            }
            InnerType::Ptr(_) => MInstr::Push {
                ty: MTy::Int,
                val: MVal::Int(-1),
            },
            InnerType::Option(child_ty) => MInstr::None {
                ty: child_ty.to_michelson_ty(),
            },
        };
        res
    }

    pub fn get_name(&self) -> String {
        match self {
            InnerType::Address => "address".to_string(),
            InnerType::Array { size, elementtype } => {
                format!("[{} x {}]", size, elementtype.get_name())
            }
            InnerType::Bool => "bool".to_string(),
            InnerType::Mutez => "mutez".to_string(),
            InnerType::Int => "int".to_string(),
            InnerType::Nat => "nat".to_string(),
            InnerType::Struct { id, fields: _ } => format!("%struct.{id}"),
            InnerType::Contract(_) => panic!(),
            InnerType::Operation => panic!(),
            InnerType::Ptr(ty) => {
                let inner = ty.get_name();
                format!("{inner}*")
            }
            InnerType::Option(ty) => match &**ty {
                InnerType::Address => {
                    format!("address")
                }
                InnerType::Contract(child_ty) => {
                    let inner = child_ty.get_name();
                    format!("(%struct.contract {inner})")
                }
                InnerType::Operation => format!("operation"),
                _ => {
                    let inner = ty.get_name();
                    format!("(option {inner})")
                }
            },
        }
    }

    pub fn deref(ty: &InnerType) -> InnerType {
        match ty {
            InnerType::Ptr(inner) => *(inner.clone()),
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
        value: Value,
        ptr: Register,
    },
    GetElementPtr {
        result: Register,
        //If the inbounds keyword is present, the result value of
        //the getelementptr is a poison value
        //inboudns
        ty: Type,
        ptrval: Register,
        subsequent: Vec<(Type, Value)>,
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
        op1: Value,
        op2: Value,
    },
    Icmp {
        result: Register,
        cond: Condition,
        ty: Type,
        op1: Value,
        op2: Value,
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
        tokens: Value,
        contract: Register,
    },
}
