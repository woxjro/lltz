use lalrpop_util;
use lalrpop_util::lalrpop_mod;

pub mod compiler;
pub mod json_to_mlir;
pub mod lltz_ir;
pub mod mlir_michelson_dialect_ast;

lalrpop_mod!(pub michelson_type);
#[test]
fn michelson_type() {
    assert!(michelson_type::TypeParser::new()
        .parse("!michelson.mutez")
        .is_ok());
    assert!(michelson_type::TypeParser::new()
        .parse("!michelson.list<!michelson.mutez>")
        .is_ok());
    assert!(michelson_type::TypeParser::new()
        .parse("!michelson.pair<!michelson.list<!michelson.operation>,!michelson.mutez>")
        .is_ok());
    assert!(michelson_type::TypeParser::new()
        .parse("(!michelson.mutez, !michelson.mutez) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.mutez>")
        .is_ok());
    let ast = michelson_type::TypeParser::new()
        .parse("(!michelson.mutez, !michelson.mutez) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.mutez>");
    println!("{:?}", ast);
    println!(
        "{:?}",
        json_to_mlir::string_to_michelson_type(
            "!michelson.pair<!michelson.list<!michelson.operation>,!michelson.mutez>".to_string(),
        )
    );
}
