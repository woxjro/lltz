use std::process::Command;

//examples/* のサンプルコードがwell typedかをテストする
#[test]
fn well_typed_test() {
    let file_names = [
        "simple_add",
        "simple_if",
        "simple_while",
        "simple_pointer",
        "simple_struct",
    ];

    for file_name in file_names {
        println!("======{file_name}=====");
        //cargo run --example simple_add
        let _ = Command::new("cargo")
            .args(["run", "--example", file_name])
            .output()
            .expect("faled to execute process");

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
