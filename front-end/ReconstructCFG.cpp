//===-- CFGToJSON.cpp - Export CFG to JSON --------------------------------===//
//
//                     The LLVM Compiler Infrastructure
//
// This file is distributed under the University of Illinois Open Source
// License. See LICENSE.TXT for details.
//
//===----------------------------------------------------------------------===//
///
/// \file
/// Exports an LLVM control flow graph (CFG), including function calls, to JSON.
///
//===----------------------------------------------------------------------===//

#include "llvm/Analysis/LoopInfo.h"
#include "llvm/IR/CFG.h"
#include "llvm/IR/Function.h"
#include "llvm/IR/InlineAsm.h"
#include "llvm/IR/Instructions.h"
#include "llvm/IR/IntrinsicInst.h"
#include "llvm/IR/LegacyPassManager.h"
#include "llvm/IR/Module.h"
#include "llvm/Pass.h"
#include "llvm/Support/CommandLine.h"
#include "llvm/Support/FileSystem.h"
#include "llvm/Support/Path.h"
#include "llvm/Transforms/IPO/PassManagerBuilder.h"

#include "json/json.h"

#define DEBUG_TYPE "cfg-to-json"

namespace {

using SourceRange = std::pair<llvm::DebugLoc, llvm::DebugLoc>;

llvm::cl::opt<std::string> OutDir("cfg-outdir",
                                  llvm::cl::desc("Output directory"),
                                  llvm::cl::value_desc("directory"),
                                  llvm::cl::init("./examples/cfg-json/"));

// Not available in older LLVM versions
static std::string getNameOrAsOperand(const llvm::Value *V) {
    if (!V->getName().empty()) {
        return std::string(V->getName());
    }

    std::string BBName;
    llvm::raw_string_ostream OS(BBName);
    V->printAsOperand(OS, false);
    return OS.str();
}

// Adapted from seadsa
static const llvm::Value *
getCalledFunctionThroughAliasesAndCasts(const llvm::Value *V) {
    const llvm::Value *CalledV = V->stripPointerCasts();

    if (const llvm::Function *F =
            llvm::dyn_cast<const llvm::Function>(CalledV)) {
        return F;
    }

    if (const llvm::GlobalAlias *GA =
            llvm::dyn_cast<const llvm::GlobalAlias>(CalledV)) {
        if (const llvm::Function *F = llvm::dyn_cast<const llvm::Function>(
                GA->getAliasee()->stripPointerCasts())) {
            return F;
        }
    }

    return CalledV;
}

class CFGToJSON : public llvm::ModulePass {
  public:
    static char ID;
    CFGToJSON() : ModulePass(ID) {}

    virtual void getAnalysisUsage(llvm::AnalysisUsage &) const override;
    virtual void print(llvm::raw_ostream &,
                       const llvm::Module *) const override;
    virtual bool runOnModule(llvm::Module &) override;
};

class ReconstructCFGPass: public llvm::LoopInfoWrapperPass {
  public:
    virtual bool runOnFunction(llvm::Function &) override;
};


} // anonymous namespace

char CFGToJSON::ID = 0;

bool ReconstructCFGPass::runOnFunction(llvm::Function &F) {
    llvm::errs() << "Function " << F.getName() << '\n';
    return false;
}

// Adapted from llvm::CFGPrinter::getSimpleNodeLabel
static std::string getBBLabel(const llvm::BasicBlock *BB) {
    if (!BB->getName().empty()) {
        return BB->getName().str();
    }

    std::string Str;
    llvm::raw_string_ostream OS(Str);

    BB->printAsOperand(OS, false);
    return OS.str();
}

static SourceRange getSourceRange(const llvm::BasicBlock *BB) {
    llvm::DebugLoc Start;
    for (const auto &I : *BB) {
        const auto &DbgLoc = I.getDebugLoc();
        if (DbgLoc) {
            Start = DbgLoc;
            break;
        }
    }

    return {Start, BB->getTerminator()->getDebugLoc()};
}

void CFGToJSON::getAnalysisUsage(llvm::AnalysisUsage &AU) const {
    AU.setPreservesAll();
}

void CFGToJSON::print(llvm::raw_ostream &OS, const llvm::Module *M) const {
    // Nothing to do here
}

bool CFGToJSON::runOnModule(llvm::Module &M) {
    llvm::SmallPtrSet<const llvm::BasicBlock *, 32> SeenBBs;
    llvm::SmallVector<const llvm::BasicBlock *, 32> Worklist;

    Json::Value JFuncs, JBlocks, JEdges, JCalls, JUnresolvedCalls, JReturns;

    for (const auto &F : M) {
        if (F.isDeclaration()) {
            continue;
        }
        SeenBBs.clear();
        Worklist.clear();
        Worklist.push_back(&F.getEntryBlock());

        JBlocks.clear();
        JEdges.clear();
        JCalls.clear();
        JUnresolvedCalls.clear();
        JReturns.clear();

        while (!Worklist.empty()) {
            auto *BB = Worklist.pop_back_val();

            // Prevent loops
            if (!SeenBBs.insert(BB).second) {
                continue;
            }

            // Save the basic block
            const auto &BBLabel = getBBLabel(BB);
            const auto &[SrcStart, SrcEnd] = getSourceRange(BB);

            Json::Value JBlock;
            JBlock["start_line"] =
                SrcStart ? SrcStart.getLine() : Json::Value();
            JBlock["end_line"] = SrcEnd ? SrcEnd.getLine() : Json::Value();
            Json::Value JInstructions;
            for (const auto &I : *BB) {
                std::string str;
                llvm::raw_string_ostream ss(str);
                ss << I;
                JInstructions.append(ss.str());
            }
            JBlock["instructions"] = JInstructions;

            JBlocks[BBLabel] = JBlock;

            // Save the intra-procedural edges
            for (auto SI = succ_begin(BB), SE = succ_end(BB); SI != SE; ++SI) {
                Json::Value JEdge;
                JEdge["src"] = BBLabel;
                JEdge["dst"] = getBBLabel(*SI);
                JEdge["type"] = BB->getTerminator()->getOpcodeName();
                JEdges.append(JEdge);

                Worklist.push_back(*SI);
            }

            // Save the inter-procedural edges
            for (auto &I : *BB) {
                // Skip debug instructions
                if (llvm::isa<llvm::DbgInfoIntrinsic>(&I)) {
                    continue;
                }

                if (const auto *CB = llvm::dyn_cast<llvm::CallBase>(&I)) {
                    if (CB->isIndirectCall()) {
                        JUnresolvedCalls.append(BBLabel);
                    } else {
                        const auto *Target =
                            getCalledFunctionThroughAliasesAndCasts(
                                CB->getCalledOperand());

                        Json::Value JCall;
                        JCall["src"] = BBLabel;
                        JCall["dst"] = [&Target]() {
                            if (const auto *IAsm =
                                    llvm::dyn_cast<llvm::InlineAsm>(Target)) {
                                return IAsm->getAsmString();
                            } else {
                                return getNameOrAsOperand(Target);
                            }
                        }();
                        JCall["type"] = I.getOpcodeName();

                        JCalls.append(JCall);
                    }
                }
            }

            const auto *Term = BB->getTerminator();
            assert(!llvm::isa<llvm::CatchSwitchInst>(Term) &&
                   "catchswitch instruction not yet supported");
            assert(!llvm::isa<llvm::CatchReturnInst>(Term) &&
                   "catchret instruction not yet supported");
            assert(!llvm::isa<llvm::CleanupReturnInst>(Term) &&
                   "cleanupret instruction not yet supported");
            if (llvm::isa<llvm::ReturnInst>(Term) ||
                llvm::isa<llvm::ResumeInst>(Term)) {
                Json::Value JReturn;
                JReturn["block"] = BBLabel;
                JReturn["type"] = Term->getOpcodeName();

                JReturns.append(JReturn);
            }
        }

        // Save function
        Json::Value JFunc;
        JFunc["name"] = getNameOrAsOperand(&F);
        JFunc["entry"] = getBBLabel(&F.getEntryBlock());
        JFunc["blocks"] = JBlocks;
        JFunc["edges"] = JEdges;
        JFunc["calls"] = JCalls;
        JFunc["returns"] = JReturns;
        JFunc["unresolved_calls"] = JUnresolvedCalls;
        JFuncs.append(JFunc);
    }

    // Print the results
    Json::Value JMod;
    JMod["module"] = M.getName().str();
    JMod["functions"] = JFuncs;

    const auto ModName = llvm::sys::path::filename(M.getName());
    llvm::SmallString<32> Filename(OutDir.c_str());
    llvm::sys::path::append(Filename, "cfg." + ModName + ".json");
    llvm::errs() << "Writing module '" << M.getName() << "' to '" << Filename
                 << "'...";

    std::error_code EC;
    llvm::raw_fd_ostream File(
        Filename, EC, llvm::sys::fs::CreationDisposition::CD_CreateAlways);

    if (!EC) {
        File << JMod.toStyledString();
    } else {
        llvm::errs() << "  error opening file for writing!";
    }
    llvm::errs() << "\n";

    return false;
}

//static llvm::RegisterPass<CFGToJSON> tmp("cfg-to-json", "Export a CFG to JSON",
//                                       false, false);

static llvm::RegisterPass<ReconstructCFGPass> tmp("reconstruct-cfg", "Reconstruct CFG",
                                       false, false);


static void registerReconstructCFGPass(const llvm::PassManagerBuilder &,
                              llvm::legacy::PassManagerBase &PM) {
    PM.add(new ReconstructCFGPass());
}

static llvm::RegisterStandardPasses
    RegisterReconstructCFGPass(llvm::PassManagerBuilder::EP_OptimizerLast,
                      registerReconstructCFGPass);

static llvm::RegisterStandardPasses
    RegisterReconstructCFGPass0(llvm::PassManagerBuilder::EP_EnabledOnOptLevel0,
                       registerReconstructCFGPass);
