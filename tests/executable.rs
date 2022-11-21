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
        let build_output = Command::new("cargo")
            .args(["run", "--example", file_name])
            .output()
            .expect("faled to execute process");

        //FIXME: .len() == 0よりも良い方法
        let build_result = String::from_utf8(build_output.stdout).unwrap();
        if build_result.len() == 0 {
            panic!("failed to bulid {}.rs", file_name);
        } else {
            println!("====Build Completed===");
        }

        //tezos-client --mode mockup run script ./examples/out/simple_while.tz on storage 'Unit' and input 'Unit' --trace-stack
        //tezos-client --mode mockup --base-dir /tmp/mockup run script ./examples/out/simple_contract_and_operation.tz on storage 'Unit' and input 'Unit' --trace-stack
        let mut command = Command::new("tezos-client");
        command.args([
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
        ]);
        println!("{:?}", command);

        let output = command.output().expect("faled to execute process");

        let result = String::from_utf8(output.stdout).unwrap();
        println!("result: {}", &result);
        let executable = result.contains("emitted operations");
        assert_eq!(executable, true);
    }
}
