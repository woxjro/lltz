//! LLTZ IR の定義

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
    pub fn struct_type2michelson_pair(ty: Type) -> String {
        match ty {
            Type::Struct { id: _, fields } => {
                let mut res = String::new();
                if fields.len() >= 2 {
                    for (i, field) in fields.iter().enumerate() {
                        if i == 0 {
                            res = Type::struct_type2michelson_pair(field.clone())
                        } else {
                            res = format!(
                                "{res} {}",
                                Type::struct_type2michelson_pair(field.clone())
                            );
                        }
                    }
                    format!("(pair {res})")
                } else if fields.len() == 1 {
                    Type::struct_type2michelson_pair(fields.iter().nth(0).unwrap().clone())
                } else {
                    format!("unit")
                }
            }
            Type::Array {
                size: _,
                elementtype: _,
            } => todo!(),
            _ => match ty {
                Type::Address => String::from("address"),
                Type::Bool => String::from("bool"),
                Type::Mutez => String::from("mutez"),
                Type::Int => String::from("int"),
                Type::Nat => String::from("nat"),
                Type::Struct { .. } => {
                    panic!() //never occur
                }
                Type::Array { .. } => {
                    panic!() //never occur
                }
                Type::Contract(ty) => {
                    let inner = Type::struct_type2michelson_pair(*ty);
                    format!("(contract {inner})")
                }
                Type::Operation => String::from("operation"),
                Type::Ptr(_) => String::from("int"),
                Type::Option(ty) => {
                    let inner = Type::struct_type2michelson_pair(*ty);
                    format!("(option {inner})")
                }
            },
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
            BackendType::Address => String::from("address"),
            BackendType::Bool => String::from("bool"),
            BackendType::Mutez => String::from("mutez"),
            BackendType::Int => String::from("int"),
            BackendType::Nat => String::from("nat"),
            BackendType::Struct { .. } => BackendType::struct_type2michelson_pair(self),
            BackendType::Contract(child_ty) => {
                let inner = BackendType::struct_type2michelson_pair(&**child_ty);
                format!("(contract {inner})")
            }
            BackendType::Operation => String::from("operation"),
            BackendType::Ptr(_) => String::from("int"),
            BackendType::Option(child_ty) => {
                let inner = child_ty.to_string();
                format!("(option {inner})")
            }
            BackendType::Array { .. } => panic!(),
        }
    }

    pub fn to_memory_string(&self) -> String {
        match self {
            BackendType::Struct { .. } => String::from("(map int int)"),
            BackendType::Array { .. } => String::from("(map int int)"),
            _ => self.to_string(),
        }
    }

    ///予約語Typeを受け取り, MichelsonのPairを返す.
    ///Storage, Parameter, PairなどといったMichelsonコードの引数を生成するために使う
    pub fn struct_type2michelson_pair(&self) -> String {
        match self {
            BackendType::Struct { id: _, fields } => {
                let mut res = String::new();
                if fields.len() >= 2 {
                    for (i, field) in fields.iter().enumerate() {
                        if i == 0 {
                            res = BackendType::struct_type2michelson_pair(&field)
                        } else {
                            res = format!(
                                "{res} {}",
                                BackendType::struct_type2michelson_pair(&field)
                            );
                        }
                    }
                    format!("(pair {res})")
                } else if fields.len() == 1 {
                    BackendType::struct_type2michelson_pair(&fields.iter().nth(0).unwrap())
                } else {
                    format!("unit")
                }
            }
            BackendType::Array {
                size: _,
                elementtype: _,
            } => {
                panic!()
            }
            _ => match self {
                BackendType::Address => String::from("address"),
                BackendType::Array { .. } => panic!(),
                BackendType::Bool => String::from("bool"),
                BackendType::Mutez => String::from("mutez"),
                BackendType::Int => String::from("int"),
                BackendType::Nat => String::from("nat"),
                BackendType::Struct { .. } => {
                    panic!() //never occur
                }
                BackendType::Contract(ty) => {
                    let inner = BackendType::struct_type2michelson_pair(&ty);
                    format!("(contract {inner})")
                }
                BackendType::Operation => String::from("operation"),
                BackendType::Ptr(_) => String::from("int"),
                BackendType::Option(ty) => {
                    let inner = BackendType::struct_type2michelson_pair(&ty);
                    format!("(option {inner})")
                }
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
pub struct LltzIr {
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
