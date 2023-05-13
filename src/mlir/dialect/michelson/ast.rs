use crate::mlir::dialect;
use michelson_ast::ty::Ty as MTy;

use lalrpop_util;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub mlir_parser);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Unit,
    Mutez,
    Operation,
    Option {
        ty: Box<Type>,
    },
    Pair {
        ty1: Box<Type>,
        ty2: Box<Type>,
    },
    List {
        ty: Box<Type>,
    },
    SmartContract {
        param: Box<Type>,
        storage: Box<Type>,
        res: Box<Type>,
    },
}

impl Type {
    //FIXME: 削除したい．impl From<Type> for MTy としたい
    pub fn michelify(&self) -> MTy {
        match self {
            Type::Unit => MTy::Unit,
            Type::Mutez => MTy::Mutez,
            Type::Operation => MTy::Operation,
            Type::Pair { ty1, ty2 } => MTy::Pair {
                ty1: Box::new(ty1.michelify()),
                ty2: Box::new(ty2.michelify()),
            },
            Type::List { ty } => MTy::List {
                ty: Box::new(ty.michelify()),
            },
            _ => todo!(),
        }
    }

    pub fn get_dialect(&self) -> dialect::DialectKind {
        dialect::DialectKind::Michelson
    }
}

impl From<String> for Type {
    fn from(s: String) -> Self {
        mlir_parser::TypeParser::new().parse(&s).unwrap()
    }
}

pub enum Tok {
    Unit,
    Mutez,
    Operation,
    Pair,
    List,
}
