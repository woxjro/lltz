# LLTZ: Compiler from LLVM IR to Michelson

There is sample and test code under the `examples` directory.
Before compiling LLTZ IR code under the `examples` directory to Michelson, you must create an `out` directory under the `examples` directory.
```
mkdir ./examples/out/
```
After that, you can compile them by the following command.
```
cargo run --example simple_while
```

You can compile all sample codes by the following command.
This command checks whether generated Michelson codes are well-typed or not.
```
cargo test well_typed
```
