# Michelson MLIR Dialect

## Building
This setup assumes that you have built LLVM and MLIR in `$BUILD_DIR` and installed them to `$PREFIX`. To build, run
```sh
mkdir build && cd build
cmake .. \
    -DMLIR_DIR=$PWD/../../llvm-project/build/lib/cmake/mlir
```
