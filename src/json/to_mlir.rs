use lalrpop_util;
use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub mlir_parser);

use crate::mlir::dialect::michelson::ast::Type;

pub fn string_to_mlir(s: String) -> Type {
    mlir_parser::TypeParser::new().parse(&s).unwrap()
}
