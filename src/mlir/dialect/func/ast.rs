use crate::mlir::{ast, dialect};

#[derive(Debug, Clone)]
pub enum Operation {
    //TODO: `func.func`の実装
    //FuncOp { result: ast::Result },
    ReturnOp { operands: Vec<ast::Operand> },
}

enum OperationKind {
    //FuncOp,
    ReturnOp,
}

impl ToString for OperationKind {
    fn to_string(&self) -> String {
        match self {
            //OperationKind::FuncOp => "func".to_owned(),
            OperationKind::ReturnOp => "return".to_owned(),
        }
    }
}

impl From<ast::Operation> for Operation {
    fn from(operation: ast::Operation) -> Operation {
        match operation.dialect {
            dialect::DialectKind::Func => {
                //if operation.get_mnemonic() == OperationKind::FuncOp.to_string() {
                //    Operation::FuncOp {
                //        result: operation.results[0].to_owned(),
                //    }
                //}
                if operation.get_mnemonic() == OperationKind::ReturnOp.to_string() {
                    Operation::ReturnOp {
                        operands: operation.operands,
                    }
                } else {
                    panic!("unsupported operation")
                }
            }
            _ => panic!(),
        }
    }
}
