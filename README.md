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

build `llvm-project`
```sh
$ cmake -G Ninja ../llvm \
    -DCMAKE_BUILD_TYPE=Release \
    -DLLVM_ENABLE_PROJECTS=mlir \
    -DLLVM_BUILD_EXAMPLES=ON \
    -DLLVM_TARGETS_TO_BUILD=X86 \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=YES \
    -DLLVM_CCACHE_BUILD=ON \
    -DCMAKE_C_COMPILER=ccache-clang \
    -DCMAKE_CXX_COMPILER=ccache-clang++
$ cmake --build . --parallel 7 --target check-mlir
```
