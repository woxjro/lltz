//===- JsonDump.cpp - Passes to illustrate the IR nesting ---------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "mlir/IR/AsmState.h"
#include "mlir/IR/BuiltinOps.h"
#include "mlir/Pass/Pass.h"
#include "llvm/Support/JSON.h"

using namespace mlir;

namespace {
struct JsonDumpPass : public PassWrapper<JsonDumpPass, OperationPass<>> {
  MLIR_DEFINE_EXPLICIT_INTERNAL_INLINE_TYPE_ID(JsonDumpPass)

  StringRef getArgument() const final { return "dump-json"; }
  StringRef getDescription() const final { return "dump a Json."; }

  llvm::json::Array opArr;
  llvm::json::Array argArr;
  llvm::json::Object json = llvm::json::Object{
      {"operations", std::move(opArr)}};

  // TODO: Top Level の Operation は builtin.module で固定なはずなので，
  //       "operations" から始めない．
  //  Entry point for the pass.
  void runOnOperation() override {
    Operation *op = getOperation();

    llvm::json::Array arr;
    llvm::json::Object json =
        llvm::json::Object{{"operations", std::move(arr)}};

    auto opObj = convertOpToJson(op);
    opArr.push_back(std::move(opObj));
    json["operations"] = std::move(opArr);
    json["arguments"] = std::move(argArr);

    llvm::json::Value jsonVal(std::move(json));
    llvm::errs() << llvm::formatv("{0:2}\n", jsonVal);
  }

  /// The three methods below are mutually recursive and follow the nesting of
  /// the IR: operation->region->block->operation->...
  llvm::json::Object convertOpToJson(Operation *op) {

    llvm::json::Object opJson = llvm::json::Object{};
    opJson["dialect"] = op->getDialect()->getNamespace();
    opJson["name"] = op->getName().getStringRef();
    FallbackAsmResourceMap fallbackResourceMap;
    AsmState asmState(getTopLevelOp(op), OpPrintingFlags(), nullptr,
                      &fallbackResourceMap);

    llvm::json::Array resultsJson;
    for (auto result : op->getResults()) {
      std::string resultName;
      llvm::raw_string_ostream os(resultName);
      result.printAsOperand(os, asmState);
      os.flush();
      llvm::json::Object resultJson = llvm::json::Object{
          {"dialect", result.getType().getDialect().getNamespace()},
          {"result", resultName},
          {"type", llvm::formatv("{0}", result.getType())},
      };

      /*
      auto visitFn = [&](auto element) {
          if (element) {
              llvm::errs() << llvm::formatv("{0}", element) << '\n';
          }
      };

      result.getType().walkImmediateSubElements(visitFn, visitFn);
      */

      resultsJson.push_back(std::move(resultJson));
    }
    opJson["results"] = std::move(resultsJson);

    llvm::json::Array operandsJson;
    for (auto operand : op->getOperands()) {
      std::string operandName;
      llvm::raw_string_ostream os(operandName);
      operand.printAsOperand(os, asmState);
      os.flush();

      llvm::json::Object operandJson = llvm::json::Object{
          {"dialect", operand.getType().getDialect().getNamespace()},
          {"operand", operandName},
          {"type", llvm::formatv("{0}", operand.getType())},
      };

      // operand.getType().walkImmediateSubElements(visitFn, visitFn);

      operandsJson.push_back(std::move(operandJson));
    }
    opJson["operands"] = std::move(operandsJson);

    llvm::json::Array attributesJson;
    for (auto attribute : op->getAttrDictionary()) {
      llvm::json::Object attributeJson = llvm::json::Object{
          {"name", attribute.getName().getValue()},
          {"value", llvm::formatv("{0}", attribute.getValue())}};
      attributesJson.push_back(std::move(attributeJson));
    }
    opJson["attributes"] = std::move(attributesJson);

    llvm::json::Array regionsJson;
    for (Region &region : op->getRegions()) {
      llvm::json::Object regionJson = llvm::json::Object{};
      llvm::json::Array blocksJson;
      for (Block &block : region.getBlocks()) {
        llvm::json::Object blockJson = llvm::json::Object{};
        llvm::json::Array operationsJson;
        for (Operation &op : block.getOperations()) {
          auto operationJson = convertOpToJson(&op);
          operationsJson.push_back(std::move(operationJson));
        }
        blockJson["operations"] = std::move(operationsJson);

        llvm::json::Array argumentsJson;
        for (auto argument : block.getArguments()) {
          std::string argumentName;
          llvm::raw_string_ostream os(argumentName);
          argument.printAsOperand(os, asmState);
          os.flush();

          llvm::json::Object argumentJson = llvm::json::Object{
              {"dialect", argument.getType().getDialect().getNamespace()},
              {"argument", argumentName},
              {"type", llvm::formatv("{0}", argument.getType())},
          };

          argumentsJson.push_back(std::move(argumentJson));
        }
        blockJson["arguments"] = std::move(argumentsJson);

        blocksJson.push_back(std::move(blockJson));
      }
      regionJson["blocks"] = std::move(blocksJson);
      regionsJson.push_back(std::move(regionJson));
    }
    opJson["regions"] = std::move(regionsJson);
    return opJson;
  }

  Operation *topLevelOp = nullptr;
  Operation *getTopLevelOp(Operation *op) {
    if (topLevelOp == nullptr) {
      topLevelOp = op;
    }
    return topLevelOp;
  }
};
} // namespace

namespace mlir {
void registerJsonDumpPass() { PassRegistration<JsonDumpPass>(); }
} // namespace mlir
