use crate::json::mlir::error;

use lalrpop_util;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub mlir_parser);

use serde::{Deserialize, Serialize};
type TypeString = String;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub operations: Vec<Operation>,
    pub arguments: Vec<Argument>,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Argument {
    pub argument: String,
    pub dialect: String,
    pub r#type: TypeString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Operand {
    pub dialect: String,
    pub operand: String,
    pub r#type: TypeString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Result {
    pub dialect: String,
    pub result: String,
    pub r#type: TypeString,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Region {
    pub blocks: Vec<Block>,
}

pub fn get_smart_contract_operation(
    block: Block,
) -> std::result::Result<Operation, Box<dyn std::error::Error>> {
    let ops = &block.operations[0].regions[0].blocks[0].operations;
    let smart_contract = ops.iter().find(|&op| {
        op.attributes
            .iter()
            .any(|attr| attr.name == "sym_name" && attr.value.contains("smart_contract"))
    });

    match smart_contract {
        Some(smart_contract) => Ok(smart_contract.clone()),
        None => Err(Box::new(error::NotFound {
            entity: "smart_contract".to_owned(),
        })),
    }
}
