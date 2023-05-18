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
        "simple_blockchain_operations",
        "simple_blockchain_operations2",
        "simple_contract_and_operation",
        "complex_smartcontract",
    ];

    for file_name in file_names {
        println!("======{file_name}=====");
        //cargo run --example simple_add
        let build_output = Command::new("cargo")
            .args(["run", "--example", file_name])
            .output()
            .expect("error");

        let build_result = String::from_utf8(build_output.stdout).unwrap();
        if build_result.is_empty() {
            panic!("failed to bulid {}.rs", file_name);
        } else
        /* 成功するとstack レイアウトなどが出力される */
        {
            println!("====Build Completed===");
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
        println!("{}", &result);
        let well_typed = result.contains("Well typed");
        assert_eq!(well_typed, true);
    }
}

#[test]
fn mlir_well_typed_test() {
    let file_names = ["mlir_simplest", "mlir_get_amount", "mlir_boomerang"];

    for file_name in file_names {
        println!("======{file_name}=====");
        let build_output = Command::new("cargo")
            .args(["run", "--example", file_name])
            .output()
            .expect("error");

        let build_result = String::from_utf8(build_output.stdout).unwrap();
        if !build_result.is_empty() {
            panic!("{file_name} build failed");
        }

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
        println!("{}", &result);
        let well_typed = result.contains("Well typed");
        assert_eq!(well_typed, true);
    }
}
