use crate::mlir;
pub mod func;
pub mod michelson;

#[derive(Debug, Clone)]
pub enum Operation {
    FuncOp {
        operation: func::ast::Operation,
    },
    MichelsonOp {
        operation: michelson::ast::Operation,
    },
}

impl From<mlir::ast::Operation> for Operation {
    fn from(operation: mlir::ast::Operation) -> Operation {
        match operation.dialect {
            DialectKind::Func => Operation::FuncOp {
                operation: func::ast::Operation::from(operation),
            },
            DialectKind::Michelson => Operation::MichelsonOp {
                operation: michelson::ast::Operation::from(operation),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DialectKind {
    Func,
    Michelson,
}

impl ToString for DialectKind {
    fn to_string(&self) -> String {
        match self {
            DialectKind::Michelson => "michelson".to_owned(),
            DialectKind::Func => "func".to_owned(),
        }
    }
}

impl From<&str> for DialectKind {
    fn from(s: &str) -> DialectKind {
        if s == DialectKind::Func.to_string() {
            DialectKind::Func
        } else if s == DialectKind::Michelson.to_string() {
            DialectKind::Michelson
        } else {
            todo!("{s}: unsupported DialectKind")
        }
    }
}
