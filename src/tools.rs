pub mod example {
    use michelson_ast;
    use std::fs::File;
    use std::io::prelude::*;

    pub fn emit_file(
        file_name: &str,
        storage: &str,
        parameter: &str,
        program: michelson_ast::program::Program,
    ) {
        let contents = format!("{command_typecheck}{command_mock}{michelson_code}",
            michelson_code = program.to_string(),
            command_typecheck = format!("#tezos-client --mode mockup --base-dir \
                /tmp/mockup typecheck script ./examples/out/{file_name}.tz\n"),
            command_mock = format!("#tezos-client --mode mockup --base-dir /tmp/mockup \
                run script ./examples/out/{file_name}.tz on storage '{storage}' and input '{parameter}' --trace-stack\n")
        );
        let mut file = File::create(format!("examples/out/{file_name}.tz")).unwrap();
        file.write_all(contents.as_bytes()).unwrap();
    }
} /* example */
