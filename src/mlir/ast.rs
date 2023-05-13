use crate::json::mlir::ast as json_mlir;
use crate::mlir::dialect::michelson::ast::Type;
use crate::mlir::dialect::DialectKind;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    pub fn get_type(&self) -> Type {
        self.r#type.to_owned()
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub operations: Vec<Operation>,
    pub arguments: Vec<Argument>,
}

impl From<json_mlir::Block> for Block {
    fn from(block: json_mlir::Block) -> Self {
        Self {
            operations: block
                .operations
                .iter()
                .map(|operation| operation.to_owned().into())
                .collect::<Vec<_>>(),
            arguments: block
                .arguments
                .iter()
                .map(|argument| argument.to_owned().into())
                .collect::<Vec<_>>(),
        }
    }
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

impl From<json_mlir::Operation> for Operation {
    fn from(operation: json_mlir::Operation) -> Self {
        Self {
            attributes: operation
                .attributes
                .iter()
                .map(|attribute| attribute.to_owned().into())
                .collect::<Vec<_>>(),
            dialect: DialectKind::from(&operation.dialect as &str),
            name: operation.name.to_owned(),
            operands: operation
                .operands
                .iter()
                .map(|operand| operand.to_owned().into())
                .collect::<Vec<_>>(),
            regions: operation
                .regions
                .iter()
                .map(|region| region.to_owned().into())
                .collect::<Vec<_>>(),
            results: operation
                .results
                .iter()
                .map(|result| result.to_owned().into())
                .collect::<Vec<_>>(),
        }
    }
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

impl From<json_mlir::Argument> for Argument {
    fn from(argument: json_mlir::Argument) -> Self {
        Value::new(
            &argument.argument.to_owned(),
            DialectKind::from(&argument.dialect as &str),
            Type::from(argument.r#type.to_owned()),
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

impl From<json_mlir::Attribute> for Attribute {
    fn from(attribute: json_mlir::Attribute) -> Self {
        if attribute.name.contains("function_type") {
            Self {
                name: attribute.name.to_owned(),
                value: AttrValue::Type(Type::from(attribute.value.to_owned())),
            }
        } else {
            Self {
                name: attribute.name.to_owned(),
                value: AttrValue::String(attribute.value.to_owned()),
            }
        }
    }
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

impl From<json_mlir::Operand> for Operand {
    fn from(operand: json_mlir::Operand) -> Self {
        Value::new(
            &operand.operand.to_owned(),
            DialectKind::from(&operand.dialect as &str),
            Type::from(operand.r#type.to_owned()),
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

impl From<json_mlir::Result> for Result {
    fn from(result: json_mlir::Result) -> Self {
        Value::new(
            &result.result.to_owned(),
            DialectKind::from(&result.dialect as &str),
            Type::from(result.r#type.to_owned()),
        )
        .into()
    }
}

#[derive(Debug, Clone)]
pub struct Region {
    pub blocks: Vec<Block>,
}

impl From<json_mlir::Region> for Region {
    fn from(region: json_mlir::Region) -> Self {
        Self {
            blocks: region
                .blocks
                .iter()
                .map(|block| block.to_owned().into())
                .collect(),
        }
    }
}
