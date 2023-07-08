# LLTZ: Compiler from MLIR to Michelson

### Requirements
- Working C and C++ toolchains(compiler, linker)
- cmake
- make or ninja

### Supported OS
This software has been tested and verified to work correctly on the following operating systems:
- `Ubuntu 22.04.2 LTS`


## Clone LLTZ
```sh
git clone --recursive https://github.com/woxjro/lltz
cd lltz
```

## Build & Usage

Build LLVM and MLIR:
```sh
$ cd llvm-project
$ mkdir build && cd build
$ cmake -G Ninja ../llvm \
    -DCMAKE_BUILD_TYPE=Release \
    -DLLVM_ENABLE_PROJECTS=mlir \
    -DLLVM_BUILD_EXAMPLES=ON \
    -DLLVM_TARGETS_TO_BUILD=X86 \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=YES \
    -DLLVM_CCACHE_BUILD=ON \
    -DCMAKE_C_COMPILER=clang \
    -DCMAKE_CXX_COMPILER=clang++
$ cmake --build . --parallel 7 --target check-mlir
```

Build `michelson-mlir-opt` and `michelson-mlir-lsp-server`:
```sh
$ pwd
<path/to/lltz>
$ cd mlir && mkdir build && cd build
$ cmake -G Ninja .. \
    -DMLIR_DIR=$PWD/../../llvm-project/build/lib/cmake/mlir \
    -DCMAKE_EXPORT_COMPILE_COMMANDS=YES
$ cmake --build .
```

There are sample and test code under the `examples` directory.

You can compile them by the following command.
```sh
$ cargo run --bin cli -- --input ./examples/mlir/boomerang.mlir
```

To show details, run the following command.
```sh
$ cargo run --bin cli -- --help
```
