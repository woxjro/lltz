use crate::mlir::{ast, dialect};

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

#[derive(Debug, Clone)]
pub enum Operation {
    GetUnitOp {
        result: ast::Result,
    },
    GetAmountOp {
        result: ast::Result,
    },
    MakeListOp {
        result: ast::Result,
    },
    MakePairOp {
        result: ast::Result,
        fst: ast::Operand,
        snd: ast::Operand,
    },
}

enum OperationKind {
    GetUnitOp,
    GetAmountOp,
    MakeListOp,
    MakePairOp,
}

impl ToString for OperationKind {
    fn to_string(&self) -> String {
        match self {
            OperationKind::GetUnitOp => "get_unit".to_owned(),
            OperationKind::GetAmountOp => "get_amount".to_owned(),
            OperationKind::MakeListOp => "make_list".to_owned(),
            OperationKind::MakePairOp => "make_pair".to_owned(),
        }
    }
}

impl From<ast::Operation> for Operation {
    fn from(operation: ast::Operation) -> Operation {
        match operation.dialect {
            dialect::DialectKind::Michelson => {
                if operation.get_mnemonic() == OperationKind::GetAmountOp.to_string() {
                    Operation::GetAmountOp {
                        result: operation.results[0].to_owned(),
                    }
                } else if operation.get_mnemonic() == OperationKind::MakeListOp.to_string() {
                    Operation::MakeListOp {
                        result: operation.results[0].to_owned(),
                    }
                } else if operation.get_mnemonic() == OperationKind::MakePairOp.to_string() {
                    Operation::MakePairOp {
                        result: operation.results[0].to_owned(),
                        fst: operation.operands[0].to_owned(),
                        snd: operation.operands[1].to_owned(),
                    }
                } else if operation.get_mnemonic() == OperationKind::GetUnitOp.to_string() {
                    Operation::GetUnitOp {
                        result: operation.results[0].to_owned(),
                    }
                } else {
                    panic!("unsupported operation")
                }
            }
            _ => panic!(),
        }
    }
}
