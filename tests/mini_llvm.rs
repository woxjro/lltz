//use lltz::lltz_ir::Type;

#[test]
fn lltz_ir_test() {
    /*
    let res = Type::to_entrypoint_ty(Type::Int);
    assert_eq!(res, String::from("int"));

    let res = Type::to_entrypoint_ty(Type::Bool);
    assert_eq!(res, String::from("bool"));
    let res = Type::to_entrypoint_ty(Type::Struct {
        id: String::from("Storage"),
        fields: vec![Type::Int, Type::Int, Type::Int],
    });
    assert_eq!(res, String::from("(pair int int int)"));

    let res = Type::to_entrypoint_ty(Type::Struct {
        id: String::from("Storage"),
        fields: vec![Type::Int, Type::Bool, Type::Bool, Type::Int],
    });
    assert_eq!(res, String::from("(pair int bool bool int)"));

    let struct_prefecture = Type::Struct {
        id: String::from("Prefecture"),
        fields: vec![Type::Int, Type::Int],
    };

    //%struct.Fish = type { i32, i32, i32, %struct.Prefecture }
    let struct_fish = Type::Struct {
        id: String::from("Fish"),
        fields: vec![Type::Int, Type::Int, Type::Int, struct_prefecture.clone()],
    };

    let res = Type::to_entrypoint_ty(struct_fish);
    assert_eq!(res, String::from("(pair int int int (pair int int))"));

    let res = Type::to_entrypoint_ty(Type::Struct {
        id: String::from("Storage"),
        fields: vec![],
    });
    assert_eq!(res, String::from("unit"));

    let res = Type::to_entrypoint_ty(Type::Struct {
        id: String::from("aaa"),
        fields: vec![
            Type::Struct {
                id: String::from("bbb"),
                fields: vec![Type::Int],
            },
            Type::Struct {
                id: String::from("ccc"),
                fields: vec![],
            },
            Type::Int,
        ],
    });
    assert_eq!(res, String::from("(pair int unit int)"));
    */
}
