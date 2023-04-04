//===- MLIRGen.h - MLIR Generation from a Chibichelson AST -------------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file declares a simple interface to perform IR generation targeting MLIR
// from a Module AST for the Chibichelson language.
//
//===----------------------------------------------------------------------===//

#ifndef CHIBICHELSON_MLIRGEN_H
#define CHIBICHELSON_MLIRGEN_H

#include <memory>

namespace mlir {
class MLIRContext;
template <typename OpTy>
class OwningOpRef;
class ModuleOp;
} // namespace mlir

namespace chibichelson {
class ModuleAST;

/// Emit IR for the given Chibichelson moduleAST, returns a newly created MLIR module
/// or nullptr on failure.
mlir::OwningOpRef<mlir::ModuleOp> mlirGen(mlir::MLIRContext &context,
                                          ModuleAST &moduleAST);
} // namespace chibichelson

#endif // CHIBICHELSON_MLIRGEN_H
