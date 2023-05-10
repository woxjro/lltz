use lltz::json;
use lltz::mlir::dialect::michelson::ast::Type;
#[test]
fn mlir_parser() {
    assert_eq!(
        json::to_mlir::string_to_mlir("!michelson.mutez".to_owned(),),
        Type::Mutez
    );
    assert_eq!(
        json::to_mlir::string_to_mlir("!michelson.list<!michelson.mutez>".to_owned(),),
        Type::List {
            ty: Box::new(Type::Mutez)
        }
    );

    assert_eq!(
        json::to_mlir::string_to_mlir(
            "!michelson.pair<!michelson.list<!michelson.operation>,!michelson.mutez>".to_owned(),
        ),
        Type::Pair {
            fst: Box::new(Type::List {
                ty: Box::new(Type::Operation)
            }),
            snd: Box::new(Type::Mutez)
        }
    );

    assert_eq!(
        json::to_mlir::string_to_mlir(
            "(!michelson.mutez, !michelson.mutez) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.mutez>".to_owned(),
        ),
        Type::SmartContract {
            param: Box::new(Type::Mutez),
            storage: Box::new(Type::Mutez),
            res: Box::new(
                Type::Pair {
                    fst: Box::new(Type::List {
                        ty: Box::new(Type::Operation)
                    }),
                    snd: Box::new(Type::Mutez)
                }
            )
        }
    );

    assert_eq!(
        json::to_mlir::string_to_mlir(
            "!michelson.pair<!michelson.list<!michelson.operation>,!michelson.mutez>".to_owned(),
        ),
        Type::Pair {
            fst: Box::new(Type::List {
                ty: Box::new(Type::Operation)
            }),
            snd: Box::new(Type::Mutez)
        }
    );
}
