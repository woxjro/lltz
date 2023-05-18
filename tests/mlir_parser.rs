use lltz::mlir::ast::Type;
use lltz::mlir::dialect::func::ast::Type as FType;
use lltz::mlir::dialect::michelson::ast::Type as MType;
#[test]
fn mlir_parser() {
    assert_eq!(MType::from("!michelson.mutez".to_owned()), MType::Mutez);
    assert_eq!(
        MType::from("!michelson.list<!michelson.mutez>".to_owned(),),
        MType::List {
            ty: Box::new(MType::Mutez)
        }
    );

    assert_eq!(
        MType::from(
            "!michelson.pair<!michelson.list<!michelson.operation>,!michelson.mutez>".to_owned(),
        ),
        MType::Pair {
            ty1: Box::new(MType::List {
                ty: Box::new(MType::Operation)
            }),
            ty2: Box::new(MType::Mutez)
        }
    );

    assert_eq!(
            FType::from(
                "(!michelson.mutez, !michelson.mutez) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.mutez>".to_owned(),
            ),
            FType::Function {
                arguments: vec![ Type::Michelson(MType::Mutez), Type::Michelson(MType::Mutez) ],
                results: vec![
                   Type::Michelson( MType::Pair {
                        ty1: Box::new(MType::List {
                            ty: Box::new(MType::Operation)
                        }),
                        ty2: Box::new(MType::Mutez)
                    })
                ]
            }
        );

    assert_eq!(
        MType::from(
            "!michelson.pair<!michelson.list<!michelson.operation>,!michelson.mutez>".to_owned(),
        ),
        MType::Pair {
            ty1: Box::new(MType::List {
                ty: Box::new(MType::Operation)
            }),
            ty2: Box::new(MType::Mutez)
        }
    );
}
