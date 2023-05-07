use serde::{Deserialize, Serialize};
use String as TypeString;
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

pub fn get_smart_contract_operation(block: Block) -> std::result::Result<Operation, ()> {
    let ops = &block.operations[0].regions[0].blocks[0].operations;
    let smart_contract = ops.iter().find(|&op| {
        op.attributes
            .iter()
            .any(|attr| attr.name == "sym_name" && attr.value.contains("smart_contract"))
    });
    Ok(smart_contract.unwrap().clone())
}
