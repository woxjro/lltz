# chibichelson

```bash
mkdir build && cd build
cmake .. \
    -DMLIR_DIR=$PWD/../../../llvm-project/build/lib/cmake/mlir
```

```
make
[ 33%] Linking CXX executable chibichelsonc-ch1
/usr/bin/ld: CMakeFiles/chibichelsonc-ch1.dir/chibichelsonc.cpp.o:(.data.rel.ro._ZTIN4llvm2cl3optIN1
2_GLOBAL__N_16ActionELb0ENS0_6parserIS3_EEEE+0x18): undefined reference to `typeinfo for llvm::cl::O
ption'
/usr/bin/ld: CMakeFiles/chibichelsonc-ch1.dir/chibichelsonc.cpp.o:(.data.rel.ro._ZTIN4llvm2cl6parser
IN12_GLOBAL__N_16ActionEEE+0x10): undefined reference to `typeinfo for llvm::cl::generic_parser_base
'
/usr/bin/ld: CMakeFiles/chibichelsonc-ch1.dir/chibichelsonc.cpp.o:(.data.rel.ro._ZTIN4llvm2cl15Optio
nValueCopyIN12_GLOBAL__N_16ActionEEE+0x10): undefined reference to `typeinfo for llvm::cl::GenericOp
tionValue'
/usr/bin/ld: CMakeFiles/chibichelsonc-ch1.dir/chibichelsonc.cpp.o:(.data.rel.ro._ZTIN4llvm2cl15Optio
nValueCopyINSt7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEEEE[_ZTIN4llvm2cl15OptionValueCopyINSt
7__cxx1112basic_stringIcSt11char_traitsIcESaIcEEEEE]+0x10): undefined reference to `typeinfo for llv
m::cl::GenericOptionValue'
collect2: error: ld returned 1 exit status
make[2]: *** [CMakeFiles/chibichelsonc-ch1.dir/build.make:118: chibichelsonc-ch1] Error 1
make[1]: *** [CMakeFiles/Makefile2:299: CMakeFiles/chibichelsonc-ch1.dir/all] Error 2
make: *** [Makefile:91: all] Error 2
```
