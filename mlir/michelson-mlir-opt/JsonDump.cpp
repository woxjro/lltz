//===- JsonDump.cpp - Passes to illustrate the IR nesting ---------===//
//
// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// See https://llvm.org/LICENSE.txt for license information.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
//
//===----------------------------------------------------------------------===//

#include "mlir/IR/BuiltinOps.h"
#include "mlir/IR/AsmState.h"
#include "mlir/Pass/Pass.h"
#include "llvm/Support/JSON.h"

using namespace mlir;

namespace {
/// This pass illustrates the IR nesting through printing.
struct JsonDumpPass
    : public PassWrapper<JsonDumpPass, OperationPass<>> {
  MLIR_DEFINE_EXPLICIT_INTERNAL_INLINE_TYPE_ID(JsonDumpPass)

  StringRef getArgument() const final { return "dump-json"; }
  StringRef getDescription() const final { return "dump a Json."; }


  llvm::json::Array opArr;
  llvm::json::Object json = llvm::json::Object{{"operations", std::move(opArr)}};

  // Entry point for the pass.
  void runOnOperation() override {
    //llvm::json::Object obj = llvm::json::Object{{"position", "xxx"}};
    //llvm::json::Value val(std::move(obj));
    //llvm::outs() << llvm::formatv("{0:2}\n", val);
    Operation *op = getOperation();

    llvm::json::Array arr;
    llvm::json::Object json = llvm::json::Object{{"operations", std::move(arr)}};

    auto opObj = convertOpToJson(op);
    opArr.push_back(std::move(opObj));
    json["operations"] = std::move(opArr);

    llvm::json::Value jsonVal(std::move(json));
    llvm::errs() << llvm::formatv("{0:2}\n", jsonVal);

    //resetIndent();
    //printOperation(op);
  }


  /// The three methods below are mutually recursive and follow the nesting of
  /// the IR: operation->region->block->operation->...
  llvm::json::Object convertOpToJson(Operation *op) {
    /*
    printIndent() << "visiting op: '" << op->getName() << "' with "
                  << op->getNumOperands() << " operands and "
                  << op->getNumResults() << " results\n";
    */

    llvm::json::Object opJson = llvm::json::Object{};
    opJson["dialect"] = op->getDialect()->getNamespace();
    opJson["name"] = op->getName().getStringRef();
    opJson["num_results"] = op->getNumResults();

    llvm::json::Array operandsJson;

    Operation *parentOp = op->getParentOp();
    if (!parentOp) {
      //os << "<<UNLINKED BLOCK>>\n";
    } else {
        while (auto *nextOp = parentOp->getParentOp()) {
            parentOp = nextOp;
        }
    }
    AsmState state(parentOp);
    //AsmState asmState = AsmState(ctx);
    for (auto operand: op->getOperands()) {
        operand.printAsOperand(llvm::outs(), state);
        //operand.print(llvm::outs());
        llvm::json::Object operandJson = llvm::json::Object{
            { "dialect", operand.getType().getDialect().getNamespace() },
        };
        operandsJson.push_back(std::move(operandJson));
    }
    opJson["operands"] = std::move(operandsJson);


    llvm::json::Array regionsJson;
    for (Region &region : op->getRegions()) {
        llvm::json::Object regionJson = llvm::json::Object{};
        regionJson["block_size"] = region.getBlocks().size();
        llvm::json::Array blocksJson;
        for (Block &block : region.getBlocks()) {
            llvm::json::Object blockJson = llvm::json::Object{};
            blockJson["operations_size"] = block.getOperations().size();
            llvm::json::Array operationsJson;
            for (Operation &op : block.getOperations()) {
              auto operationJson = convertOpToJson(&op);
              operationsJson.push_back(std::move(operationJson));
            }
            blockJson["operations"] = std::move(operationsJson);
            blocksJson.push_back(std::move(blockJson));
        }
        regionJson["blocks"] = std::move(blocksJson);
        regionsJson.push_back(std::move(regionJson));
    }
    opJson["regions"] = std::move(regionsJson);
    return opJson;
  }

  /// The three methods below are mutually recursive and follow the nesting of
  /// the IR: operation->region->block->operation->...
  void printOperation(Operation *op) {
    // Print the operation itself and some of its properties
    printIndent() << "visiting op: '" << op->getName() << "' with "
                  << op->getNumOperands() << " operands and "
                  << op->getNumResults() << " results\n";
    // Print the operation attributes
    if (!op->getAttrs().empty()) {
      printIndent() << op->getAttrs().size() << " attributes:\n";
      for (NamedAttribute attr : op->getAttrs())
        printIndent() << " - '" << attr.getName().getValue() << "' : '"
                      << attr.getValue() << "'\n";
    }


    // Recurse into each of the regions attached to the operation.
    printIndent() << " " << op->getNumRegions() << " nested regions:\n";
    auto indent = pushIndent();
    for (Region &region : op->getRegions())
      printRegion(region);
  }

  void printRegion(Region &region) {
    // A region does not hold anything by itself other than a list of blocks.
    printIndent() << "Region with " << region.getBlocks().size()
                  << " blocks:\n";
    auto indent = pushIndent();
    for (Block &block : region.getBlocks())
      printBlock(block);
  }

  void printBlock(Block &block) {
    // Print the block intrinsics properties (basically: argument list)
    printIndent()
        << "Block with " << block.getNumArguments() << " arguments, "
        << block.getNumSuccessors()
        << " successors, and "
        // Note, this `.size()` is traversing a linked-list and is O(n).
        << block.getOperations().size() << " operations\n";

    // Block main role is to hold a list of Operations: let's recurse.
    auto indent = pushIndent();
    for (Operation &op : block.getOperations())
      printOperation(&op);
  }

  /// Manages the indentation as we traverse the IR nesting.
  int indent;
  struct IdentRAII {
    int &indent;
    IdentRAII(int &indent) : indent(indent) {}
    ~IdentRAII() { --indent; }
  };
  void resetIndent() { indent = 0; }
  IdentRAII pushIndent() { return IdentRAII(++indent); }

  llvm::raw_ostream &printIndent() {
    for (int i = 0; i < indent; ++i)
      llvm::errs() << "  ";
    return llvm::errs();
  }
};
} // namespace

namespace mlir {
void registerJsonDumpPass() {
  PassRegistration<JsonDumpPass>();
}
} // namespace mlir
