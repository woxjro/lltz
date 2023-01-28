//===-- front-end.cpp - Implementation of LLTZ front-end
//--------------------------------===//
//
//                     The LLVM Compiler Infrastructure
//
// This file is distributed under the University of Illinois Open Source
// License. See LICENSE.TXT for details.
//
//===----------------------------------------------------------------------===//
///
///
//===----------------------------------------------------------------------===//

#include "llvm/Analysis/LoopInfo.h"
#include "llvm/Analysis/LoopPass.h"
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
                                  llvm::cl::init("./examples/out/"));

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

class ReconstructMichelsonPrimitivePass : public llvm::ModulePass {
  public:
    static char ID;
    ReconstructMichelsonPrimitivePass() : ModulePass(ID) {}

    virtual void getAnalysisUsage(llvm::AnalysisUsage &) const override;
    virtual void print(llvm::raw_ostream &,
                       const llvm::Module *) const override;
    virtual bool runOnModule(llvm::Module &) override;
};

class ReconstructCFGPass : public llvm::LoopPass {
  public:
    static char ID;
    ReconstructCFGPass() : llvm::LoopPass(ID) {}
    virtual bool runOnLoop(llvm::Loop *, llvm::LPPassManager &) override;
};

} // anonymous namespace

char ReconstructMichelsonPrimitivePass::ID = 0;
char ReconstructCFGPass::ID = 1;

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

Json::Value LoopToJson(llvm::Loop *L) {
    Json::Value JLoop;
    Json::Value JSubLoops;
    Json::Value JBlocks;
    // unsigned int depth = L->getLoopDepth();
    auto LoopLatchBB = L->getLoopLatch();
    auto LoopHeaderBB = L->getHeader();
    auto LoopExitBB = L->getExitBlock();

    JSubLoops = Json::arrayValue;
    for (auto &SL : L->getSubLoops()) {
        JSubLoops.append(LoopToJson(SL));
    }

    JLoop["loop"] = L->getName().str();

    if (L->getParentLoop() != nullptr) {
        JLoop["parent"] = L->getParentLoop()->getName().str();
    } else {
        JLoop["parent"] = Json::nullValue;
    }

    JLoop["loop"] = L->getName().str();
    JLoop["sub_loops"] = JSubLoops;
    JLoop["header"] = getBBLabel(LoopHeaderBB);

    // header の terminator 命令の 条件分岐レジスタ i1 を取得する．
    llvm::Instruction *terminator = LoopHeaderBB->getTerminator();
    if (llvm::isa<llvm::BranchInst>(terminator)) {
        llvm::BranchInst *br = llvm::dyn_cast<llvm::BranchInst>(terminator);
        if (br->isConditional()) {
            llvm::Value *cond = br->getOperand(0);
            JLoop["header_cond"] = cond->getName().str();
        } else {
            llvm::errs()
                << "<header> の terminator が 条件分岐 br ではありません．"
                << '\n';
        }
    }

    JLoop["latch"] = getBBLabel(LoopLatchBB);
    JLoop["exiting"] = getBBLabel(LoopExitBB);

    JBlocks = Json::arrayValue;
    for (const auto &BB : L->getBlocks()) {
        if (BB == LoopHeaderBB || BB == LoopLatchBB) {
            continue;
        } else {
            JBlocks.append(getBBLabel(BB));
        }
    }

    JLoop["blocks"] = JBlocks;
    return JLoop;
}

bool ReconstructCFGPass::runOnLoop(llvm::Loop *L, llvm::LPPassManager &LPM) {
    Json::Value JLoop;

    // L が SubLoop の場合は解析しない．
    if (L->getParentLoop() != nullptr) {
        return false;
    }

    JLoop = LoopToJson(L);

    const auto FuncName = llvm::sys::path::filename(L->getName());
    llvm::SmallString<32> Filename(OutDir.c_str());
    llvm::sys::path::append(Filename, "cfg." + FuncName + ".json");
    llvm::errs() << "Writing loop'" << L->getName() << "' to '" << Filename
                 << "'...";

    std::error_code EC;
    llvm::raw_fd_ostream File(
        Filename, EC, llvm::sys::fs::CreationDisposition::CD_CreateAlways);
    if (!EC) {
        File << JLoop.toStyledString();
    } else {
        llvm::errs() << "  error opening file for writing!";
    }
    llvm::errs() << "\n";

    return false;
}

void ReconstructMichelsonPrimitivePass::getAnalysisUsage(
    llvm::AnalysisUsage &AU) const {
    AU.setPreservesAll();
}

void ReconstructMichelsonPrimitivePass::print(llvm::raw_ostream &OS,
                                              const llvm::Module *M) const {
    // Nothing to do here
}

bool ReconstructMichelsonPrimitivePass::runOnModule(llvm::Module &M) {

    //@llvm.dbg.declare をscanし，MetadataからMichelsonの型を取得する．
    llvm::DenseMap<const llvm::Value *, llvm::DIDerivedType *>
        Address2MichelsonType;
    for (auto &F : M) {
        for (auto &BB : F) {
            for (auto &I : BB) {
                if (llvm::isa<llvm::DbgDeclareInst>(&I)) {
                    // auto *inst = llvm::dyn_cast<llvm::DbgDeclareInst>(&I);
                    const llvm::DbgDeclareInst *dbgDeclare =
                        llvm::dyn_cast<llvm::DbgDeclareInst>(&I);

                    // llvm.dbg.declare命令の2番目の引数(metadata !15)を取得
                    llvm::Metadata *metadata = dbgDeclare->getVariable();

                    // Metadataが付与されている'''LLVM IR の'''register
                    llvm::Value *address = dbgDeclare->getAddress();
                    // !15 = !DILocalVariable(name: "res", scope: !8, file: !9,
                    // line: 6, type: !16)
                    llvm::DILocalVariable *variable =
                        llvm::dyn_cast<llvm::DILocalVariable>(metadata);
                    // !16 = !DIDerivedType(tag: DW_TAG_typedef, name: "Mutez",
                    // file: !9, line: 2, baseType: !13)
                    llvm::DIType *type = variable->getType();
                    llvm::DIDerivedType *derivedType =
                        llvm::dyn_cast<llvm::DIDerivedType>(type);

                    auto p = std::make_pair(address, derivedType);
                    Address2MichelsonType.insert(p);
                    // "Mutez"
                    llvm::StringRef typeName = derivedType->getName();
                    llvm::errs() << "%" << address->getName().str() << " : "
                                 << typeName << "*" << '\n';
                    continue;
                }
            }
        }
    }

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

            Json::Value JBlock;
            Json::Value JInstructions;
            for (const auto &I : *BB) {
                // Skip debug instructions
                if (llvm::isa<llvm::DbgDeclareInst>(&I)) {
                    continue;
                }

                std::string str;
                llvm::raw_string_ostream ss(str);
                ss << I;

                Json::Value JInstruction, JOps;
                JInstruction["opcode"] = I.getOpcodeName();

                // JInstruction の 各要素の label
                // はllvm.org/docs/LangRefのSyntaxに従う 例:
                // https://llvm.org/docs/LangRef.html#alloca-instruction
                if (llvm::isa<llvm::AllocaInst>(&I)) {
                    const auto allocaInst =
                        llvm::dyn_cast<llvm::AllocaInst>(&I);
                    const llvm::Value *allocaPointer = allocaInst;
                    JInstruction["result"] = allocaPointer->getName().str();
                    auto p = Address2MichelsonType.find(allocaPointer);

                    if (p != Address2MichelsonType.end()) { //見つかった場合
                        JInstruction["type"] = p->getSecond()->getName().str();
                    } else { //無い場合
                        JInstruction["type"] = "XXXXXX";
                    }
                } else if (llvm::isa<llvm::StoreInst>(&I)) {
                    const auto storeInst = llvm::dyn_cast<llvm::StoreInst>(&I);

                    const llvm::Value *ptr = storeInst->getPointerOperand();
                    const llvm::Value *value = storeInst->getValueOperand();

                    if (llvm::isa<llvm::ConstantInt>(
                            value)) { // FIXME:
                                      // 即値が整数じゃない場合の対応が必要
                        const auto cons =
                            llvm::dyn_cast<llvm::ConstantInt>(value);
                        JInstruction["value"] =
                            std::to_string(cons->getSExtValue());
                    } else {
                        JInstruction["value"] = value->getName().str();
                    }
                    JInstruction["pointer"] = ptr->getName().str();

                    auto p = Address2MichelsonType.find(ptr);

                    if (p != Address2MichelsonType.end()) { //見つかった場合
                        JInstruction["ty"] = p->getSecond()->getName().str();
                        //JInstruction["ptr_ty"] = p->getSecond()->getName().str() + "*";
                    } else { //無い場合
                        JInstruction["ty"] = "XXXXXX";
                        //JInstruction["ptr_ty"] = "XXXXXX";
                    }
                } else if (llvm::isa<llvm::LoadInst>(&I)) {
                    const auto loadInst =
                        llvm::dyn_cast<llvm::LoadInst>(&I);
                    //const llvm::Value *loadPointer = loadInst;

                    JInstruction["result"] = loadInst->getName().str();

                    const llvm::Value *pointer = loadInst->getPointerOperand();
                    JInstruction["pointer"] = pointer->getName().str();
                    auto p = Address2MichelsonType.find(pointer);

                    if (p != Address2MichelsonType.end()) { //見つかった場合
                        JInstruction["ty"] = p->getSecond()->getName().str();
                    } else { //無い場合
                        JInstruction["ty"] = "XXXXXX";
                    }
                } else if (llvm::isa<llvm::GetElementPtrInst>(&I)) {
                    // TODO
                }
                JInstruction["original_instruction"] = ss.str();
                JInstructions.append(JInstruction);
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
                if (llvm::isa<llvm::DbgDeclareInst>(&I)) {
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
    llvm::sys::path::append(Filename, "metadata." + ModName + ".json");
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

//// register ReconstructMichelsonPrimitivePass {{{
static llvm::RegisterPass<ReconstructMichelsonPrimitivePass>
    tmp0("cfg-to-json", "Export a CFG to JSON", false, false);
static void
registerReconstructMichelsonPrimitivePass(const llvm::PassManagerBuilder &,
                                          llvm::legacy::PassManagerBase &PM) {
    PM.add(new ReconstructMichelsonPrimitivePass());
}

static llvm::RegisterStandardPasses RegisterReconstructMichelsonPrimitivePass(
    llvm::PassManagerBuilder::EP_OptimizerLast,
    registerReconstructMichelsonPrimitivePass);

static llvm::RegisterStandardPasses RegisterReconstructMichelsonPrimitivePass0(
    llvm::PassManagerBuilder::EP_EnabledOnOptLevel0,
    registerReconstructMichelsonPrimitivePass);

//// }}}

//// register ReconstructCFGPass {{{
static llvm::RegisterPass<ReconstructCFGPass>
    tmp("reconstruct-cfg", "Reconstruct CFG", false, false);

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
//// }}}
