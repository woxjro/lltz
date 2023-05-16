use crate::mlir::{ast, dialect};

use lalrpop_util;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub mlir_parser);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Address,
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
    Contract {
        ty: Box<Type>,
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

    /// 型がスタックに積める値を持つか否かを判定する関数
    /// 例えば
    /// - Type::Mutez は 0 をというスタックに積める値をもつため true
    /// - Type::Address は 初期値に相当する値は無いため false
    pub fn has_initial_value(&self) -> bool {
        match self {
            Type::Pair { .. } => false,
            _ => true,
        }
    }
}

impl From<String> for Type {
    fn from(s: String) -> Self {
        mlir_parser::TypeParser::new().parse(&s).unwrap()
    }
}

pub enum Tok {
    Unit,
    Address,
    Contract,
    Option,
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
    GetSourceOp {
        result: ast::Result,
    },
    GetContractOp {
        address: ast::Operand,
        result: ast::Result,
    },
    AssertSomeOp {
        operand: ast::Operand,
        result: ast::Result,
    },
    ConsOp {
        list: ast::Operand,
        element: ast::Operand,
        result: ast::Result,
    },
    TransferTokensOp {
        parameter: ast::Operand,
        amount: ast::Operand,
        contract: ast::Operand,
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
    AssertSomeOp,
    TransferTokensOp,
    ConsOp,
    GetUnitOp,
    GetAmountOp,
    GetSourceOp,
    GetContractOp,
    MakeListOp,
    MakePairOp,
}

impl ToString for OperationKind {
    fn to_string(&self) -> String {
        match self {
            OperationKind::AssertSomeOp => "assert_some".to_owned(),
            OperationKind::TransferTokensOp => "transfer_tokens".to_owned(),
            OperationKind::ConsOp => "cons".to_owned(),
            OperationKind::GetUnitOp => "get_unit".to_owned(),
            OperationKind::GetAmountOp => "get_amount".to_owned(),
            OperationKind::GetSourceOp => "get_source".to_owned(),
            OperationKind::GetContractOp => "get_contract".to_owned(),
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
                } else if operation.get_mnemonic() == OperationKind::GetSourceOp.to_string() {
                    Operation::GetSourceOp {
                        result: operation.results[0].to_owned(),
                    }
                } else if operation.get_mnemonic() == OperationKind::AssertSomeOp.to_string() {
                    Operation::AssertSomeOp {
                        operand: operation.operands[0].to_owned(),
                        result: operation.results[0].to_owned(),
                    }
                } else if operation.get_mnemonic() == OperationKind::TransferTokensOp.to_string() {
                    Operation::TransferTokensOp {
                        parameter: operation.operands[0].to_owned(),
                        amount: operation.operands[1].to_owned(),
                        contract: operation.operands[2].to_owned(),
                        result: operation.results[0].to_owned(),
                    }
                } else if operation.get_mnemonic() == OperationKind::ConsOp.to_string() {
                    Operation::ConsOp {
                        list: operation.operands[0].to_owned(),
                        element: operation.operands[1].to_owned(),
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
                } else if operation.get_mnemonic() == OperationKind::GetContractOp.to_string() {
                    Operation::GetContractOp {
                        address: operation.operands[0].to_owned(),
                        result: operation.results[0].to_owned(),
                    }
                } else {
                    todo!("'{}' is an unsupported operation", operation.get_mnemonic())
                }
            }
            _ => panic!(),
        }
    }
}
