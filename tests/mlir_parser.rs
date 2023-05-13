use lltz::mlir::dialect::michelson::ast::Type;
#[test]
fn mlir_parser() {
    assert_eq!(Type::from("!michelson.mutez".to_owned()), Type::Mutez);
    assert_eq!(
        Type::from("!michelson.list<!michelson.mutez>".to_owned(),),
        Type::List {
            ty: Box::new(Type::Mutez)
        }
    );

    assert_eq!(
        Type::from(
            "!michelson.pair<!michelson.list<!michelson.operation>,!michelson.mutez>".to_owned(),
        ),
        Type::Pair {
            ty1: Box::new(Type::List {
                ty: Box::new(Type::Operation)
            }),
            ty2: Box::new(Type::Mutez)
        }
    );

    assert_eq!(
        Type::from(
            "(!michelson.mutez, !michelson.mutez) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.mutez>".to_owned(),
        ),
        Type::SmartContract {
            param: Box::new(Type::Mutez),
            storage: Box::new(Type::Mutez),
            res: Box::new(
                Type::Pair {
                    ty1: Box::new(Type::List {
                        ty: Box::new(Type::Operation)
                    }),
                    ty2: Box::new(Type::Mutez)
                }
            )
        }
    );

    assert_eq!(
        Type::from(
            "!michelson.pair<!michelson.list<!michelson.operation>,!michelson.mutez>".to_owned(),
        ),
        Type::Pair {
            ty1: Box::new(Type::List {
                ty: Box::new(Type::Operation)
            }),
            ty2: Box::new(Type::Mutez)
        }
    );
}
