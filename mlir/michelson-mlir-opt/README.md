# Json Dumping of MLIR

`mlir::michelson::MichelsonDialect`を追加すると以下のエラーがでる．
```
[100%] Linking CXX executable ../bin/michelson-mlir-opt
ld: error: undefined symbol: mlir::detail::TypeIDResolver<mlir::michelson::MichelsonDialect, void>::id
>>> referenced by michelson-mlir-opt.cpp
>>>               CMakeFiles/michelson-mlir-opt.dir/michelson-mlir-opt.cpp.o:(mlir::detail::TypeIDResolver<mlir::m
ichelson::MichelsonDialect, void>::resolveTypeID())

ld: error: undefined symbol: mlir::michelson::MichelsonDialect::MichelsonDialect(mlir::MLIRContext*)
>>> referenced by michelson-mlir-opt.cpp
>>>               CMakeFiles/michelson-mlir-opt.dir/michelson-mlir-opt.cpp.o:(mlir::michelson::MichelsonDialect* m
lir::MLIRContext::getOrLoadDialect<mlir::michelson::MichelsonDialect>()::'lambda'()::operator()() const)
collect2: error: ld returned 1 exit status
make[2]: *** [michelson-mlir-opt/CMakeFiles/michelson-mlir-opt.dir/build.make:411: bin/michelson-mlir-opt] Error 1
make[1]: *** [CMakeFiles/Makefile2:676: michelson-mlir-opt/CMakeFiles/michelson-mlir-opt.dir/all] Error 2
make: *** [Makefile:136: all] Error 2
```

## `mlir-opt`を切り出すやり方

```sh
$ cd <michelson-mlir-opt>
$ mkdir build && cd build
$ cmake .. \
    -DMLIR_DIR=$PWD/../../../llvm-project/build/lib/cmake/mlir
$ make
```


## 古いやり方（いろいろする）
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
