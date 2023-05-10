use crate::json::to_mlir::string_to_mlir;
use crate::mlir::dialect::michelson::ast as mlir;
use lalrpop_util;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub mlir_parser);

use serde::{Deserialize, Serialize};
use String as TypeString;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub operations: Vec<Operation>,
    pub arguments: Vec<Argument>,
}

impl Block {
    pub fn to_mlir_block(&self) -> mlir::Block {
        mlir::Block {
            operations: self
                .operations
                .iter()
                .map(|operation| operation.to_mlir_operation())
                .collect::<Vec<_>>(),
            arguments: self
                .arguments
                .iter()
                .map(|argument| argument.to_mlir_argument())
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Operation {
    pub attributes: Vec<Attribute>,
    pub dialect: String,
    pub name: String,
    pub operands: Vec<Operand>,
    pub regions: Vec<Region>,
    pub results: Vec<Result>,
}
impl Operation {
    pub fn to_mlir_operation(&self) -> mlir::Operation {
        mlir::Operation {
            attributes: self
                .attributes
                .iter()
                .map(|attribute| attribute.to_mlir_attribute())
                .collect::<Vec<_>>(),
            dialect: self.dialect.to_owned(),
            name: self.name.to_owned(),
            operands: self
                .operands
                .iter()
                .map(|operand| operand.to_mlir_operand())
                .collect::<Vec<_>>(),
            regions: self
                .regions
                .iter()
                .map(|region| region.to_mlir_region())
                .collect::<Vec<_>>(),
            results: self
                .results
                .iter()
                .map(|result| result.to_mlir_result())
                .collect::<Vec<_>>(),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Argument {
    pub argument: String,
    pub dialect: String,
    pub r#type: TypeString,
}
impl Argument {
    pub fn to_mlir_argument(&self) -> mlir::Argument {
        mlir::Argument {
            argument: self.argument.to_owned(),
            dialect: self.dialect.to_owned(),
            r#type: string_to_mlir(self.r#type.to_owned()),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}
impl Attribute {
    pub fn to_mlir_attribute(&self) -> mlir::Attribute {
        if self.name.contains("function_type") {
            mlir::Attribute {
                name: self.name.to_owned(),
                value: mlir::AttrValue::Type(string_to_mlir(self.value.to_owned())),
            }
        } else {
            mlir::Attribute {
                name: self.name.to_owned(),
                value: mlir::AttrValue::String(self.value.to_owned()),
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Operand {
    pub dialect: String,
    pub operand: String,
    pub r#type: TypeString,
}
impl Operand {
    pub fn to_mlir_operand(&self) -> mlir::Operand {
        mlir::Operand {
            dialect: self.dialect.to_owned(),
            operand: self.operand.to_owned(),
            r#type: string_to_mlir(self.r#type.to_owned()),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Result {
    pub dialect: String,
    pub result: String,
    pub r#type: TypeString,
}
impl Result {
    pub fn to_mlir_result(&self) -> mlir::Result {
        mlir::Result {
            dialect: self.dialect.to_owned(),
            result: self.result.to_owned(),
            r#type: string_to_mlir(self.r#type.to_owned()),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Region {
    pub blocks: Vec<Block>,
}
impl Region {
    pub fn to_mlir_region(&self) -> mlir::Region {
        mlir::Region {
            blocks: self
                .blocks
                .iter()
                .map(|block| block.to_mlir_block())
                .collect::<Vec<_>>(),
        }
    }
}
pub fn get_smart_contract_operation(block: Block) -> std::result::Result<Operation, ()> {
    let ops = &block.operations[0].regions[0].blocks[0].operations;
    let smart_contract = ops.iter().find(|&op| {
        op.attributes
            .iter()
            .any(|attr| attr.name == "sym_name" && attr.value.contains("smart_contract"))
    });
    Ok(smart_contract.unwrap().clone())
}