#!/bin/bash

clang-14 -S -emit-llvm -g ./examples/$1.c -o ./examples/llvm-ir/$1.ll

#opt-14 -enable-new-pm=0 -load ./build/libLLVMReconstructCFG.so -reconstruct-cfg ./examples/llvm-ir/$1.ll >/dev/null
opt-14 -enable-new-pm=0 -load ./build/libLLVMReconstructCFG.so -cfg-to-json ./examples/llvm-ir/$1.ll >/dev/null

### emit a cfg image
#opt-14 -S -enable-new-pm=0 ./examples/llvm-ir/$1.ll -dot-cfg > /dev/null

#dot -Tpng .main.dot -o ./examples/cfg/$1.png

#rm .*.dot
