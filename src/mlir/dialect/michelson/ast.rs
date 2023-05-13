use crate::mlir::dialect;

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
