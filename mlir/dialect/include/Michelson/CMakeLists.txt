#add_mlir_dialect(MichelsonOps michelson)

# add_mlir_dialect マクロを使うのはやめて自分で書く．
#      add_mlir_dialect(MichelsonOps michelson)
# Declare a dialect in the include directory
#function(add_mlir_dialect dialect dialect_namespace)
  set(LLVM_TARGET_DEFINITIONS MichelsonOps.td)
  mlir_tablegen(MichelsonOps.h.inc -gen-op-decls)
  mlir_tablegen(MichelsonOps.cpp.inc -gen-op-defs)
  set(LLVM_TARGET_DEFINITIONS MichelsonTypes.td)
  mlir_tablegen(MichelsonTypes.h.inc -gen-typedef-decls -typedefs-dialect=michelson)
  mlir_tablegen(MichelsonTypes.cpp.inc -gen-typedef-defs -typedefs-dialect=michelson)
  add_public_tablegen_target(MLIRMichelsonTypesIncGen)
  set(LLVM_TARGET_DEFINITIONS MichelsonDialect.td)
  mlir_tablegen(MichelsonDialect.h.inc -gen-dialect-decls -dialect=michelson)
  mlir_tablegen(MichelsonDialect.cpp.inc -gen-dialect-defs -dialect=michelson)
  add_public_tablegen_target(MLIRMichelsonOpsIncGen)
  add_dependencies(mlir-headers MLIRMichelsonOpsIncGen)
#endfunction()


add_mlir_doc(MichelsonDialect MichelsonDialect Michelson/ -gen-dialect-doc)
add_mlir_doc(MichelsonOps MichelsonOps Michelson/ -gen-op-doc)
