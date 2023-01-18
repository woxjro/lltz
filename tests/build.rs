use std::process::Command;

//examples/* のサンプルコードがビルド出来るかテストする
#[test]
fn build() {
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

        //FIXME: .len() == 0よりも良い方法
        let build_result = String::from_utf8(build_output.stdout).unwrap();
        if build_result.len() == 0 {
            panic!("failed to bulid {}.rs", file_name);
        } else {
            println!("====Build Completed===");
        }
    }
}
