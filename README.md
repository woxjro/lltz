# LLTZ: Compiler from MLIR to Michelson

## build & usage

build `llvm-project`:
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

There are sample and test programs under the `examples` directory.

You can compile them by the following command.
```sh
$ cargo run --bin cli -- --input ./examples/mlir/boomerang.mlir
```

## License
