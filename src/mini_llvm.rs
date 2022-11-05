//! LLVM IR'に出てくるもの（命令, 型定義, 関数...）を定義しているモジュール

///LLVM IR'のレジスタ
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

///LLVM IR'に出てくるデータ型
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
            _ => match ty {
                Type::Address => String::from("address"),
                Type::Bool => String::from("bool"),
                Type::Mutez => String::from("mutez"),
                Type::Int => String::from("int"),
                Type::Nat => String::from("nat"),
                Type::Struct { .. } => {
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

    pub fn to_llvm_ty(ty: &Type) -> String {
        match ty {
            Type::Address => "address".to_string(),
            Type::Bool => "i1 for bool".to_string(),
            Type::Mutez => "(i64 for mutez)".to_string(),
            Type::Int => "(i64 for int)".to_string(),
            Type::Nat => "(i64 for nat)".to_string(),
            Type::Struct { id, fields: _ } => format!("%struct.{id}"),
            Type::Contract(ty) => {
                let inner = Type::to_llvm_ty(&*ty);
                format!("(%struct.contract {inner})")
            }
            Type::Operation => String::from("%struct.operation"),
            Type::Ptr(ty) => {
                let inner = Type::to_llvm_ty(&*ty);
                format!("{inner}*")
            }
            Type::Option(ty) => {
                let inner = Type::to_llvm_ty(&*ty);
                format!("(option {inner})")
            }
        }
    }

    //FIXME: 多分どこかバグっている {{
    //       (フロントで)option operationをしようと思うとバグる気がする
    fn to_michelson_ty(ty: &Type) -> String {
        match ty {
            Type::Address => String::from("address"),
            Type::Bool => String::from("bool"),
            Type::Mutez => String::from("mutez"),
            Type::Int => String::from("int"),
            Type::Nat => String::from("nat"),
            Type::Struct { .. } => {
                String::from("(map int int)")
                //map (struct.index, ptr)
            }
            Type::Contract(_) => panic!(),
            Type::Operation => panic!(),
            Type::Ptr(_) => String::from("int"),
            Type::Option(ty) => {
                let inner = Type::to_michelson_ty(&*ty);
                format!("(option {inner})")
            }
        }
    }

    pub fn to_michelson_backend_ty(ty: &Type) -> String {
        match ty {
            Type::Operation => {
                format!("(option {})", Type::to_michelson_ty2(&ty))
            }
            Type::Contract(_) => {
                format!("(option {})", Type::to_michelson_ty2(&ty))
            }
            _ => Type::to_michelson_ty(&ty),
        }
    }

    pub fn to_michelson_ty2(ty: &Type) -> String {
        match ty {
            Type::Address => String::from("address"),
            Type::Bool => String::from("bool"),
            Type::Mutez => String::from("mutez"),
            Type::Int => String::from("int"),
            Type::Nat => String::from("nat"),
            Type::Struct { .. } => Type::struct_type2michelson_pair(ty.clone()),
            Type::Contract(child_ty) => {
                let inner = Type::to_michelson_ty2(&*child_ty);
                format!("(contract {inner})")
            }
            Type::Operation => String::from("operation"),
            Type::Ptr(_) => String::from("int"),
            Type::Option(child_ty) => {
                let inner = Type::to_michelson_ty2(&*child_ty);
                format!("(option {inner})")
            }
        }
    }

    pub fn default_value(ty: &Type) -> String {
        let res = match ty {
            Type::Address => String::from("\"KT1PGQFmnGyZMeuHzssNxqx9tYfDvX5JMN3W\""),
            Type::Bool => String::from("False"),
            Type::Mutez => String::from("0"),
            Type::Int => String::from("0"),
            Type::Nat => String::from("0"),
            Type::Contract(ty) => {
                let inner = Type::to_michelson_ty2(&*ty);
                //NOTE: contractはbig_mapに入らないので、内部的にはoption型に包む
                format!("NONE (contract {inner})")
            }
            Type::Operation => {
                //NOTE: operationはbig_mapに入らないので、内部的にはoption型に包む
                String::from("NONE operation")
            }
            Type::Struct { .. } => {
                panic!("Struct型にはDefaultの値はありません.")
            }
            Type::Ptr(_) => String::from("-1"),
            Type::Option(ty) => {
                let inner = Type::to_michelson_backend_ty(&*ty);
                format!("(None {inner})")
            }
        };
        res
    }
    // }}

    pub fn deref(ty: &Type) -> Type {
        match ty {
            Type::Ptr(inner) => *(inner.clone()),
            _ => panic!(),
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

pub enum ReservedStructKind {
    Parameter,
    Storage,
    Operation,
}

#[allow(dead_code)]
pub struct Arg {
    pub ty: Type,
    pub reg: Register,
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
}
