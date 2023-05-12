use crate::json::mlir::ast::{Argument as JArgument, Operand as JOperand, Result as JResult};
use crate::json::to_mlir::string_to_mlir;
use crate::mlir::dialect::michelson::ast::Type;
use crate::mlir::dialect::DialectKind;
use michelson_ast::ty::Ty as MTy;
use std::any::Any;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Value {
    id: String,
    dialect: DialectKind,
    r#type: Type,
}

impl Value {
    pub fn new(id: &str, dialect: DialectKind, r#type: Type) -> Self {
        Self {
            id: id.to_owned(),
            dialect,
            r#type,
        }
    }
    pub fn get_dialect(&self) -> DialectKind {
        self.dialect.to_owned()
    }
    pub fn get_id(&self) -> String {
        self.id.to_owned()
    }
    pub fn get_type(&self) -> Box<dyn BaseType> {
        Box::new(self.r#type.to_owned())
    }
    pub fn try_to_get_michelson_type(&self) -> std::result::Result<MTy, &str> {
        let base_type: Box<dyn Any> = Box::new(self.r#type.to_owned());
        // TODO: if let Some(ty) = base_type.downcast_ref::<MichelsonType>() {
        if let Some(ty) = base_type.downcast_ref::<Type>() {
            Ok(ty.michelify())
        } else {
            Err("A casting to MichelsonType has failed.")
        }
    }
}

pub trait BaseType {
    fn get_dialect(&self) -> DialectKind;
}

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
    value: Value,
}

impl Argument {
    pub fn get_value(&self) -> Value {
        self.value.to_owned()
    }
}

impl From<Value> for Argument {
    fn from(value: Value) -> Self {
        Self { value }
    }
}

impl From<JArgument> for Argument {
    fn from(argument: JArgument) -> Self {
        Value::new(
            &argument.argument.to_owned(),
            DialectKind::from(&argument.dialect as &str),
            string_to_mlir(argument.r#type.to_owned()),
        )
        .into()
    }
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
    value: Value,
}

impl Operand {
    pub fn get_value(&self) -> Value {
        self.value.to_owned()
    }
}

impl From<Value> for Operand {
    fn from(value: Value) -> Self {
        Self { value }
    }
}

impl From<JOperand> for Operand {
    fn from(operand: JOperand) -> Self {
        Value::new(
            &operand.operand.to_owned(),
            DialectKind::from(&operand.dialect as &str),
            string_to_mlir(operand.r#type.to_owned()),
        )
        .into()
    }
}

#[derive(Debug, Clone)]
pub struct Result {
    value: Value,
}

impl Result {
    pub fn get_value(&self) -> Value {
        self.value.to_owned()
    }
}

impl From<Value> for Result {
    fn from(value: Value) -> Self {
        Self { value }
    }
}

impl From<JResult> for Result {
    fn from(result: JResult) -> Self {
        Value::new(
            &result.result.to_owned(),
            DialectKind::from(&result.dialect as &str),
            string_to_mlir(result.r#type.to_owned()),
        )
        .into()
    }
}

#[derive(Debug, Clone)]
pub struct Region {
    pub blocks: Vec<Block>,
}
