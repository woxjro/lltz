#[derive(Debug, Clone)]
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
