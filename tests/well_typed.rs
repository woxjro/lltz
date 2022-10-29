use std::process::Command;

//examples/* のサンプルコードがwell typedかをテストする
#[test]
fn well_typed_test() {
    let file_names = [
        "simple_add",
        "simple_add_nat",
        "simple_add_mutez",
        "simple_if",
        "simple_while",
        "simple_pointer",
        "simple_struct",
        "simple_struct2",
        "simple_llvm_memcpy",
        "simple_smartcontract",
        "complex_smartcontract",
    ];

    for file_name in file_names {
        println!("======{file_name}=====");
        //cargo run --example simple_add
        let build_output = Command::new("cargo")
            .args(["run", "--example", file_name])
            .output()
            .expect("error");

        //FIXME: .len() == 0よりも良い方法
        let build_result = String::from_utf8(build_output.stdout).unwrap();
        if build_result.len() == 0 {
            panic!("failed to bulid {}.rs", file_name);
        }

        //tezos-client --mode mockup typecheck script ./examples/out/simple_add.tz
        let output = Command::new("tezos-client")
            .args([
                "--mode",
                "mockup",
                "typecheck",
                "script",
                &format!("./examples/out/{file_name}.tz"),
            ])
            .output()
            .expect("faled to execute process");

        let result = String::from_utf8(output.stdout).unwrap();
        let well_typed = result.contains("Well typed");
        assert_eq!(well_typed, true);
    }
}
