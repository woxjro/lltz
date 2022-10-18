use mini_llvm_michelson_compiler::mini_llvm::{reserved_type2michelson_pair, Type};

#[test]
fn mini_llvm_test() {
    let res = reserved_type2michelson_pair(Type::I32);
    assert_eq!(res, String::from("int"));

    let res = reserved_type2michelson_pair(Type::I1);
    assert_eq!(res, String::from("bool"));

    let res = reserved_type2michelson_pair(Type::Struct {
        id: String::from("Storage"),
        fields: vec![Type::I32, Type::I32, Type::I32],
    });
    assert_eq!(res, String::from("(pair int int int)"));

    let res = reserved_type2michelson_pair(Type::Struct {
        id: String::from("Storage"),
        fields: vec![Type::I32, Type::I1, Type::I1, Type::I32],
    });
    assert_eq!(res, String::from("(pair int bool bool int)"));

    let struct_prefecture = Type::Struct {
        id: String::from("Prefecture"),
        fields: vec![Type::I32, Type::I32],
    };

    //%struct.Fish = type { i32, i32, i32, %struct.Prefecture }
    let struct_fish = Type::Struct {
        id: String::from("Fish"),
        fields: vec![Type::I32, Type::I32, Type::I32, struct_prefecture.clone()],
    };

    let res = reserved_type2michelson_pair(struct_fish);
    assert_eq!(res, String::from("(pair int int int (pair int int))"));

    let res = reserved_type2michelson_pair(Type::Struct {
        id: String::from("Storage"),
        fields: vec![],
    });
    assert_eq!(res, String::from("unit"));
}
