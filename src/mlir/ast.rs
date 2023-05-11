use crate::mlir::dialect::michelson::ast::Type;
use crate::mlir::dialect::DialectKind;
use michelson_ast::ty::Ty as MTy;
use std::any::Any;

pub trait Value {
    fn get_dialect(&self) -> DialectKind;
    fn get_id(&self) -> String;
    fn get_type(&self) -> Box<dyn BaseType>;
    fn try_to_get_michelson_type(&self) -> std::result::Result<MTy, &str>;
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
    pub argument: String,
    pub dialect: DialectKind,
    pub r#type: Type,
}

impl Value for Argument {
    fn get_dialect(&self) -> DialectKind {
        self.dialect.to_owned()
    }
    fn get_id(&self) -> String {
        self.argument.to_owned()
    }
    fn get_type(&self) -> Box<dyn BaseType> {
        Box::new(self.r#type.to_owned())
    }
    fn try_to_get_michelson_type(&self) -> std::result::Result<MTy, &str> {
        let base_type: Box<dyn Any> = Box::new(self.r#type.to_owned());
        // TODO: if let Some(ty) = base_type.downcast_ref::<MichelsonType>() {
        if let Some(ty) = base_type.downcast_ref::<Type>() {
            Ok(ty.michelify())
        } else {
            Err("A casting to MichelsonType has failed.")
        }
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
    pub dialect: DialectKind,
    pub operand: String,
    pub r#type: Type,
}

impl Value for Operand {
    fn get_dialect(&self) -> DialectKind {
        self.dialect.to_owned()
    }
    fn get_id(&self) -> String {
        self.operand.to_owned()
    }
    fn get_type(&self) -> Box<dyn BaseType> {
        Box::new(self.r#type.to_owned())
    }
    fn try_to_get_michelson_type(&self) -> std::result::Result<MTy, &str> {
        let base_type: &dyn Any = &self.get_type();
        // TODO: if let Some(ty) = base_type.downcast_ref::<MichelsonType>() {
        if let Some(ty) = base_type.downcast_ref::<Type>() {
            Ok(ty.michelify())
        } else {
            Err("A casting to MichelsonType has failed.")
        }
    }
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
