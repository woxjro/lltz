use lalrpop_util;
use lalrpop_util::lalrpop_mod;

pub mod compiler;
pub mod json;
pub mod lltz_ir;
pub mod mlir;

lalrpop_mod!(pub mlir_parser);
#[test]
fn mlir() {
    assert!(mlir_parser::TypeParser::new()
        .parse("!michelson.mutez")
        .is_ok());
    assert!(mlir_parser::TypeParser::new()
        .parse("!michelson.list<!michelson.mutez>")
        .is_ok());
    assert!(mlir_parser::TypeParser::new()
        .parse("!michelson.pair<!michelson.list<!michelson.operation>,!michelson.mutez>")
        .is_ok());
    assert!(mlir_parser::TypeParser::new()
        .parse("(!michelson.mutez, !michelson.mutez) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.mutez>")
        .is_ok());
    let ast = mlir_parser::TypeParser::new()
        .parse("(!michelson.mutez, !michelson.mutez) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.mutez>");
    println!("{:?}", ast);
    println!(
        "{:?}",
        json_to_mlir::string_to_mlir(
            "!michelson.pair<!michelson.list<!michelson.operation>,!michelson.mutez>".to_string(),
        )
    );
}
