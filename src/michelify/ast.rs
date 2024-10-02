use crate::mlir::ast::Type;
use crate::mlir::dialect::michelson::ast as michelson_dialect;
use michelson_ast::instruction::Instruction as MichelsonInstruction;
use michelson_ast::ty::Ty as MichelsonType;
use michelson_ast::val::Val as MichelsonVal;

/// Michelson の Stack 上に確保される値の型
/// 例：
/// MLIR の Operation 型は Michelson の Stack 上では Option Operation 型となる
#[derive(Debug, Clone)]
pub enum StackType {
    Address,
    Bytes,
    Unit,
    Int,
    Nat,
    Mutez,
    Operation,
    Option {
        ty: Box<StackType>,
    },
    Pair {
        ty1: Box<StackType>,
        ty2: Box<StackType>,
    },
    List {
        ty: Box<StackType>,
    },
    Contract {
        ty: Box<StackType>,
    },
}

impl From<michelson_dialect::Type> for StackType {
    fn from(ty: michelson_dialect::Type) -> StackType {
        match ty {
            michelson_dialect::Type::Operation => StackType::Option {
                ty: Box::new(StackType::Operation),
            },
            michelson_dialect::Type::Pair { ty1, ty2 } => StackType::Option {
                ty: Box::new(StackType::Pair {
                    ty1: Box::new(stupidly_from(ty1.as_ref().to_owned())),
                    ty2: Box::new(stupidly_from(ty2.as_ref().to_owned())),
                }),
            },
            michelson_dialect::Type::Contract { ty } => StackType::Option {
                ty: Box::new(StackType::Contract {
                    ty: Box::new(stupidly_from(ty.as_ref().to_owned())),
                }),
            },
            ty => stupidly_from(ty.to_owned()),
        }
    }
}

impl From<Type> for StackType {
    fn from(ty: Type) -> StackType {
        match ty {
            Type::Michelson(ty) => StackType::from(ty),
            Type::Func(_) => panic!(),
        }
    }
}

impl StackType {
    pub fn default_value_instruction(&self) -> Vec<MichelsonInstruction> {
        match self {
            StackType::Unit => vec![MichelsonInstruction::Unit],
            StackType::Address => vec![MichelsonInstruction::Source],
            StackType::Bytes => vec![
                MichelsonInstruction::Push {
                    ty: MichelsonType::Int,
                    val: MichelsonVal::Int(0),
                },
                MichelsonInstruction::Bytes,
            ],
            StackType::Int => vec![MichelsonInstruction::Push {
                ty: MichelsonType::Int,
                val: MichelsonVal::Int(0),
            }],
            StackType::Nat => vec![MichelsonInstruction::Push {
                ty: MichelsonType::Nat,
                val: MichelsonVal::Nat(0),
            }],
            StackType::Mutez => vec![MichelsonInstruction::Push {
                ty: MichelsonType::Mutez,
                val: MichelsonVal::Mutez(0),
            }],
            StackType::Option { ty } => vec![MichelsonInstruction::None {
                ty: MichelsonType::from(ty.as_ref().to_owned()),
            }],
            StackType::List { ty } => vec![MichelsonInstruction::Nil {
                ty: MichelsonType::from(ty.as_ref().to_owned()),
            }],
            ty => todo!("{:?}", ty),
        }
    }
}

fn stupidly_from(ty: michelson_dialect::Type) -> StackType {
    match ty {
        michelson_dialect::Type::Unit => StackType::Unit,
        michelson_dialect::Type::Address => StackType::Address,
        michelson_dialect::Type::Mutez => StackType::Mutez,
        michelson_dialect::Type::Operation => StackType::Operation,
        michelson_dialect::Type::Option { ty } => StackType::Option {
            ty: Box::new(stupidly_from(ty.as_ref().to_owned())),
        },
        michelson_dialect::Type::List { ty } => StackType::List {
            ty: Box::new(stupidly_from(ty.as_ref().to_owned())),
        },
        michelson_dialect::Type::Pair { ty1, ty2 } => StackType::Pair {
            ty1: Box::new(stupidly_from(ty1.as_ref().to_owned())),
            ty2: Box::new(stupidly_from(ty2.as_ref().to_owned())),
        },
        michelson_dialect::Type::Contract { ty } => StackType::Contract {
            ty: Box::new(stupidly_from(ty.as_ref().to_owned())),
        },
        michelson_dialect::Type::Bytes => StackType::Bytes,
        michelson_dialect::Type::Int => StackType::Int,
        michelson_dialect::Type::Nat => StackType::Nat,
    }
}

/// michelson_ast::ty::Ty を拡張
impl From<StackType> for MichelsonType {
    fn from(stack_type: StackType) -> MichelsonType {
        match stack_type {
            StackType::Address => MichelsonType::Address,
            StackType::Bytes => MichelsonType::Bytes,
            StackType::Unit => MichelsonType::Unit,
            StackType::Int => MichelsonType::Int,
            StackType::Nat => MichelsonType::Nat,
            StackType::Mutez => MichelsonType::Mutez,
            StackType::Operation => MichelsonType::Operation,
            StackType::Option { ty } => MichelsonType::Option {
                ty: Box::new(MichelsonType::from(*ty)),
            },
            StackType::List { ty } => MichelsonType::List {
                ty: Box::new(MichelsonType::from(*ty)),
            },
            StackType::Pair { ty1, ty2 } => MichelsonType::Pair {
                ty1: Box::new(MichelsonType::from(*ty1)),
                ty2: Box::new(MichelsonType::from(*ty2)),
            },
            StackType::Contract { ty } => MichelsonType::Contract {
                ty: Box::new(MichelsonType::from(*ty)),
            },
        }
    }
}

/// michelson_ast::ty::Ty を拡張
impl From<michelson_dialect::Type> for MichelsonType {
    fn from(ty: michelson_dialect::Type) -> MichelsonType {
        match ty {
            michelson_dialect::Type::Unit => MichelsonType::Unit,
            michelson_dialect::Type::Mutez => MichelsonType::Mutez,
            michelson_dialect::Type::Operation => MichelsonType::Operation,
            michelson_dialect::Type::Option { ty } => MichelsonType::Option {
                ty: Box::new(MichelsonType::from(*ty)),
            },
            michelson_dialect::Type::Pair { ty1, ty2 } => MichelsonType::Pair {
                ty1: Box::new(MichelsonType::from(*ty1)),
                ty2: Box::new(MichelsonType::from(*ty2)),
            },
            michelson_dialect::Type::List { ty } => MichelsonType::List {
                ty: Box::new(MichelsonType::from(*ty)),
            },
            michelson_dialect::Type::Contract { ty } => MichelsonType::Contract {
                ty: Box::new(MichelsonType::from(*ty)),
            },
            michelson_dialect::Type::Address => MichelsonType::Address,
            michelson_dialect::Type::Bytes => MichelsonType::Bytes,
            michelson_dialect::Type::Int => MichelsonType::Int,
            michelson_dialect::Type::Nat => MichelsonType::Nat,
        }
    }
}

/// michelson_ast::ty::Ty を拡張
impl From<Type> for MichelsonType {
    fn from(ty: Type) -> MichelsonType {
        match ty {
            Type::Michelson(ty) => MichelsonType::from(ty),
            Type::Func(_) => panic!(),
        }
    }
}
