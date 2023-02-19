# LLTZ: Compiler from LLVM IR to Michelson

There are sample and test programs under the `examples` directory.

You can compile them by the following command.
```
cargo run --example <file_name>
```
For example, to compile `examples/simple_while.rs`, run the following command.
```
cargo run --example simple_while
```

Compiled Michelson programs are output under the `examples/out` directory.

You can compile all sample programs by the following command.
```
cargo test build
```
