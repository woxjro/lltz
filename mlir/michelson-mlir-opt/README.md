# Json Dumping of MLIR

## いろいろする
### 1. `JsonDump.cpp`を適切な場所に設置する
```sh
$ cp ./JsonDump.cpp ../llvm-project/mlir/test/lib/IR/JsonDump.cpp
```

### 2. `JsonDump`Passを登録する
TODO


## How To Build
```sh
$ cd <llvm-project>
$ mkdir build_mlir-opt && cd build_mlir-opt
$ cmake ../llvm \
   -DLLVM_ENABLE_PROJECTS=mlir \
   -DLLVM_BUILD_EXAMPLES=ON \
   -DLLVM_TARGETS_TO_BUILD=X86 \
   -DCMAKE_BUILD_TYPE=Release \
   -DLLVM_ENABLE_ASSERTIONS=ON \
   -DCMAKE_EXPORT_COMPILE_COMMANDS=YES \
   -DLLVM_CCACHE_BUILD=ON \
   -DCMAKE_C_COMPILER=clang-14 \
   -DCMAKE_CXX_COMPILER=clang++-14
```

## Initial Build
```sh
$ make --jobs=10
```

## Rebuild
```sh
$ make mlir-opt --jobs=10
```

## Execution
```sh
$ mlir-opt --dump-json -allow-unregistered-dialect ../mlir/test/IR/slice.mlir 1> /dev/null
```
or
```sh
$ mlir-opt --dump-json -allow-unregistered-dialect ../mlir/test/IR/slice.mlir 1> /dev/null 2> <file>.json
```
