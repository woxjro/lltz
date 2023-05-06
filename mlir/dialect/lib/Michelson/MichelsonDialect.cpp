//===- Dialect.cpp - Michelson IR Dialect registration in MLIR ------------------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//
//
// This file implements the dialect for the Michelson IR: custom type parsing and
// operation verification.
//
//===----------------------------------------------------------------------===//

#include "Michelson/Dialect.h"

#include "mlir/IR/Builders.h"
#include "mlir/IR/BuiltinTypes.h"
#include "mlir/IR/FunctionImplementation.h"
#include "mlir/IR/OpImplementation.h"

using namespace mlir;
using namespace mlir::michelson;

#include "Michelson/MichelsonDialect.cpp.inc"

//===----------------------------------------------------------------------===//
// MichelsonDialect
//===----------------------------------------------------------------------===//

/// Dialect initialization, the instance will be owned by the context. This is
/// the point of registration of types and operations for the dialect.
void MichelsonDialect::initialize() {
    /*
  addTypes<
#define GET_TYPEDEF_LIST
#include "Michelson/MichelsonTypes.cpp.inc"
      >();

  addOperations<
#define GET_OP_LIST
#include "Michelson/MichelsonOps.cpp.inc"
      >();
      */
}

//===----------------------------------------------------------------------===//
// Michelson Operations
//===----------------------------------------------------------------------===//


//===----------------------------------------------------------------------===//
// TableGen'd op method definitions
//===----------------------------------------------------------------------===//

#define GET_OP_CLASSES
#include "Michelson/MichelsonOps.cpp.inc"
