use crate::mlir::dialect::michelson::ast::Type;
use crate::mlir::dialect::DialectKind;

#[derive(Debug, Clone)]
pub struct Block {
    pub operations: Vec<Operation>,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub attributes: Vec<Attribute>,
    pub dialect: DialectKind,
    pub name: String,
    pub operands: Vec<Operand>,
    pub regions: Vec<Region>,
    pub results: Vec<Result>,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub argument: String,
    pub dialect: DialectKind,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub enum AttrValue {
    String(String),
    Type(Type),
}
#[derive(Debug, Clone)]
pub struct Attribute {
    pub name: String,
    pub value: AttrValue,
}

#[derive(Debug, Clone)]
pub struct Operand {
    pub dialect: DialectKind,
    pub operand: String,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub struct Result {
    pub dialect: DialectKind,
    pub result: String,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub struct Region {
    pub blocks: Vec<Block>,
}
