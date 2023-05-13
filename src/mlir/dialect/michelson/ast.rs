use crate::mlir::dialect;
use michelson_ast::ty::Ty as MTy;

use lalrpop_util;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub mlir_parser);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type {
    Unit,
    Mutez,
    Operation,
    Pair {
        fst: Box<Type>,
        snd: Box<Type>,
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
    pub fn michelify(&self) -> MTy {
        match self {
            Type::Unit => MTy::Unit,
            Type::Mutez => MTy::Mutez,
            Type::Operation => MTy::Operation,
            Type::Pair { fst, snd } => MTy::Pair {
                ty1: Box::new(fst.michelify()),
                ty2: Box::new(snd.michelify()),
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
