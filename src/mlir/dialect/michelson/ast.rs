use crate::mlir::{ast, dialect};

use lalrpop_util;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub mlir_parser);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Address,
    Bytes,
    Unit,
    Int,
    Nat,
    Mutez,
    Operation,
    Option { ty: Box<Type> },
    Pair { ty1: Box<Type>, ty2: Box<Type> },
    Contract { ty: Box<Type> },
    List { ty: Box<Type> },
}

impl Type {
    pub fn get_dialect(&self) -> dialect::DialectKind {
        dialect::DialectKind::Michelson
    }
}

impl From<String> for Type {
    fn from(s: String) -> Self {
        mlir_parser::MTypeParser::new().parse(&s).unwrap()
    }
}

pub enum Tok {
    Unit,
    Int,
    Nat,
    Address,
    Bytes,
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
    GetBytesOp {
        result: ast::Result,
        number: ast::Operand,
    },
    Sha256Op {
        result: ast::Result,
        bytes: ast::Operand,
    },
    Sha3Op {
        result: ast::Result,
        bytes: ast::Operand,
    },
    Sha512Op {
        result: ast::Result,
        bytes: ast::Operand,
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
    GetBytesOp,
    Sha256Op,
    Sha3Op,
    Sha512Op,
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
            OperationKind::GetBytesOp => "get_bytes".to_owned(),
            OperationKind::Sha256Op => "sha256".to_owned(),
            OperationKind::Sha3Op => "sha3".to_owned(),
            OperationKind::Sha512Op => "sha512".to_owned(),
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
                } else if operation.get_mnemonic() == OperationKind::GetBytesOp.to_string() {
                    Operation::GetBytesOp {
                        number: operation.operands[0].to_owned(),
                        result: operation.results[0].to_owned(),
                    }
                } else if operation.get_mnemonic() == OperationKind::Sha256Op.to_string() {
                    Operation::Sha256Op {
                        bytes: operation.operands[0].to_owned(),
                        result: operation.results[0].to_owned(),
                    }
                } else if operation.get_mnemonic() == OperationKind::Sha3Op.to_string() {
                    Operation::Sha3Op {
                        bytes: operation.operands[0].to_owned(),
                        result: operation.results[0].to_owned(),
                    }
                } else if operation.get_mnemonic() == OperationKind::Sha512Op.to_string() {
                    Operation::Sha512Op {
                        bytes: operation.operands[0].to_owned(),
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
