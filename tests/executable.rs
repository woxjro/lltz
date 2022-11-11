use std::process::Command;

//examples/* のサンプルコードが実行可能かをテストする
#[test]
fn executable_test() {
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
        let _ = Command::new("cargo")
            .args(["run", "--example", file_name])
            .output()
            .expect("faled to execute process");

        //tezos-client --mode mockup run script ./examples/out/simple_while.tz on storage 'Unit' and input 'Unit' --trace-stack
        let output = Command::new("tezos-client")
            .args([
                "--mode",
                "mockup",
                "run",
                "script",
                &format!("./examples/out/{file_name}.tz"),
                "on",
                "storage",
                "'Unit'",
                "and",
                "input",
                "'Unit'",
            ])
            .output()
            .expect("faled to execute process");

        let result = String::from_utf8(output.stdout).unwrap();
        println!("result: {}", &result);
        let executable = result.contains("emitted operations");
        assert_eq!(executable, true);
    }
}
