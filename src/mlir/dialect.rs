pub mod michelson;

trait Value {
    fn get_dialect(&self) -> DialectKind;
    fn get_id(&self) -> String;
    fn get_type(&self) -> dyn BaseType;
}

pub trait BaseType {
    fn get_dialect(&self) -> DialectKind;
}

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
