use crate::mlir::dialect::michelson::ast as michelson_dialect;
use michelson_ast::instruction::Instruction as MichelsonInstruction;
use michelson_ast::ty::Ty as MichelsonType;
use michelson_ast::val::Val as MichelsonVal;

/// Michelson の Stack 上に確保される値の型
/// 例：
/// MLIR の Operation 型は Michelson の Stack 上では Option Operation 型となる
#[derive(Debug, Clone)]
pub enum StackType {
    Unit,
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
            michelson_dialect::Type::List { ty } => StackType::Option {
                ty: Box::new(StackType::List {
                    ty: Box::new(stupidly_from(ty.as_ref().to_owned())),
                }),
            },
            michelson_dialect::Type::SmartContract { .. } => panic!(),
            ty => stupidly_from(ty.to_owned()),
        }
    }
}

impl StackType {
    pub fn default_value_instruction(&self) -> MichelsonInstruction {
        match self {
            StackType::Unit => MichelsonInstruction::Unit,
            StackType::Mutez => MichelsonInstruction::Push {
                ty: MichelsonType::Mutez,
                val: MichelsonVal::Mutez(0),
            },
            StackType::Option { ty } => MichelsonInstruction::None {
                ty: MichelsonType::from(ty.as_ref().to_owned()),
            },
            StackType::List { ty } => MichelsonInstruction::Nil {
                ty: MichelsonType::from(ty.as_ref().to_owned()),
            },
            _ => todo!(),
        }
    }
}

fn stupidly_from(ty: michelson_dialect::Type) -> StackType {
    match ty {
        michelson_dialect::Type::Unit => StackType::Unit,
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
        _ => todo!(),
    }
}

/// michelson_ast::ty::Ty を拡張
impl From<StackType> for MichelsonType {
    fn from(stack_type: StackType) -> MichelsonType {
        match stack_type {
            StackType::Unit => MichelsonType::Unit,
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
        }
    }
}
