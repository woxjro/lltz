//===- Dialect.h - Dialect definition for the Michelson IR ----------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file implements the IR Dialect for the Michelson language.
// See docs/Tutorials/Michelson/Ch-2.md for more information.
//
//===----------------------------------------------------------------------===//

#ifndef MLIR_TUTORIAL_MICHELSON_DIALECT_H_
#define MLIR_TUTORIAL_MICHELSON_DIALECT_H_

#include "mlir/IR/Dialect.h"
#include "mlir/IR/FunctionInterfaces.h"
#include "mlir/IR/SymbolTable.h"
#include "mlir/Interfaces/CallInterfaces.h"
#include "mlir/Interfaces/SideEffectInterfaces.h"

/// Include the auto-generated header file containing the declaration of the michelson
/// dialect.
#include "Michelson/MichelsonOpsDialect.h.inc"

/// Include the auto-generated header file containing the declarations of the
/// michelson operations.
#define GET_OP_CLASSES
#include "Michelson/MichelsonOps.h.inc"

#endif // MLIR_TUTORIAL_MICHELSON_DIALECT_H_
