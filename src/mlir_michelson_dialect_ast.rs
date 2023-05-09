#[derive(Debug, Clone)]
pub enum Type {
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
    Mutez,
    Operation,
    Pair,
    List,
}
