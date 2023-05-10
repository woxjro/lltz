pub mod michelson;

#[derive(Debug, Clone)]
pub enum DialectKind {
    Func,
    Michelson,
}

impl From<&str> for DialectKind {
    fn from(s: &str) -> DialectKind {
        if s == "func" {
            DialectKind::Func
        } else if s == "michelson" {
            DialectKind::Michelson
        } else {
            todo!()
        }
    }
}
