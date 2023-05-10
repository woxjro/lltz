#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Unit,
    Mutez,
    Operation,
    Pair {
        fst: Box<Type>,
        snd: Box<Type>,
    },
    List {
        ty: Box<Type>,
    },
    SmartContract {
        param: Box<Type>,
        storage: Box<Type>,
        res: Box<Type>,
    },
}

pub enum Tok {
    Unit,
    Mutez,
    Operation,
    Pair,
    List,
}

#[derive(Debug, Clone)]
pub struct Block {
    pub operations: Vec<Operation>,
    pub arguments: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub attributes: Vec<Attribute>,
    pub dialect: String,
    pub name: String,
    pub operands: Vec<Operand>,
    pub regions: Vec<Region>,
    pub results: Vec<Result>,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub argument: String,
    pub dialect: String,
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
    pub dialect: String,
    pub operand: String,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub struct Result {
    pub dialect: String,
    pub result: String,
    pub r#type: Type,
}

#[derive(Debug, Clone)]
pub struct Region {
    pub blocks: Vec<Block>,
}
