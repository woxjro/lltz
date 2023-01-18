use lltz::mini_llvm::Type;

#[test]
fn mini_llvm_test() {
    let res = Type::struct_type2michelson_pair(Type::Int);
    assert_eq!(res, String::from("int"));

    let res = Type::struct_type2michelson_pair(Type::Bool);
    assert_eq!(res, String::from("bool"));
    let res = Type::struct_type2michelson_pair(Type::Struct {
        id: String::from("Storage"),
        fields: vec![Type::Int, Type::Int, Type::Int],
    });
    assert_eq!(res, String::from("(pair int int int)"));

    let res = Type::struct_type2michelson_pair(Type::Struct {
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

    let res = Type::struct_type2michelson_pair(struct_fish);
    assert_eq!(res, String::from("(pair int int int (pair int int))"));

    let res = Type::struct_type2michelson_pair(Type::Struct {
        id: String::from("Storage"),
        fields: vec![],
    });
    assert_eq!(res, String::from("unit"));

    let res = Type::struct_type2michelson_pair(Type::Struct {
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
}
