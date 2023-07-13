

# LLTZ: Compiler from MLIR to Michelson

<div align="left">
    <a href="https://github.com/woxjro/lltz/"><img alt="github" src="https://img.shields.io/badge/github-woxjro/lltz-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20"></a>
    <a href="https://github.com/woxjro/lltz/actions"><img alt="build status" src="https://img.shields.io/github/actions/workflow/status/woxjro/lltz/rust.yml?style=for-the-badge" height="20"></a>
    <a href="https://github.com/woxjro/lltz/blob/master/LICENSE"><img alt="license" src="https://img.shields.io/github/license/woxjro/lltz?style=for-the-badge&labelColor=555555" height="20"></a>
</div>


<img src="https://github.com/woxjro/lltz/assets/63214188/a108fc50-5758-49a4-bbed-af31d7a62c80" alt="lltz-logo" height="150px" align="right" />


<br>

[WIP] LLTZ is a compiler from [MLIR](https://mlir.llvm.org/) to [Michelson](https://tezos.gitlab.io/michelson-reference/), VM of blockchain [Tezos](https://tezos.com/).


<details>
 <summary><strong>Table of contents</strong></summary>
 <br/>


- [LLTZ: Compiler from MLIR to Michelson](#lltz-compiler-from-mlir-to-michelson)
  - [Features](#features)
  - [Motivation](#motivation)
  <!-- - [Performance Analysis](#performance-analysis) -->
  - [Prerequisites](#prerequisites)
  - [Supported OS](#supported-os)
  - [Clone LLTZ](#clone-lltz)
  - [Build & Usage](#build--usage)
  - [Contributing](#contributing)
  <!-- - [Special thanks](#special-thanks) -->


<br/>
</details>


## Features
- **Reusable** and **readable** intermediate representation **Michelson Dialect**
- Compilation method with **easy analysis** and **practical gas consumption**
- World's first compiler from MLIR to a blockchain VM

<img width="4493" alt="lltz_overview" src="https://github.com/woxjro/lltz/assets/63214188/29ae6dce-7ded-4a4f-8e39-3a93f4023bd6">

## Motivation
A Domain-Specific Language (DSL) is generally necessary to write smart contracts.
These DSLs have contributed to the advancement of blockchain technology.
However, these impose a high learning cost on developers, becoming a barrier to entry into blockchain ecosystems such as Tezos.

To address this issue and enable smart contract development using general-purpose programming languages, I've developed LLTZ as a first step.
This compiler compiles MLIR, an intermediate representation defined by the [LLVM](https://llvm.org/) compiler infrastructure, into Michelson.
Unlike [LLVM IR](https://llvm.org/docs/LangRef.html), [JVM](https://docs.oracle.com/javase/specs/jvms/se7/html/) and [WebAssembly](https://webassembly.org/), MLIR provides a higher level of abstraction with control structures and types and allows the definition of custom intermediate representations for specific domains through a feature called [Dialect](https://mlir.llvm.org/docs/Dialects/).

If smart contracts can be written in a general-purpose programming language, a single program can run both off-chain and on-chain.

## Prerequisites
- Working C and C++ toolchains(compiler, linker)
- `cmake`
- `make` or `ninja`
- [`tezos-client`](https://wiki.tezos.com/build/clients/): Only required for running tests.

## Supported OS
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

## Contributing
This project is currently in the Proof of Concept (PoC) stage.
Contributions are kindly requested to be postponed until further notice.

