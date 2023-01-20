# Front-end

## Building
```bash
mkdir build
cd build
```

```bash
CC=clang-14 \
    CXX=clang++-14 \
    cmake -DCMAKE_EXPORT_COMPILE_COMMANDS=YES \
    -DLLVM_DIR=~/develop/llvm-michelson/build_14x/lib/cmake/llvm \
    ..
```

## Running
```bash
sh ./c2cfg-json.sh <file_name>
#sh ./c2cfg-json.sh simple_loop
```
