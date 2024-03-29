set(LLVM_OPTIONAL_SOURCES
  null.cpp
)

get_property(dialect_libs GLOBAL PROPERTY MLIR_DIALECT_LIBS)
get_property(conversion_libs GLOBAL PROPERTY MLIR_CONVERSION_LIBS)
set(LLVM_LINK_COMPONENTS
  Core
  Support
  AsmParser
  )

if(MLIR_INCLUDE_TESTS)
  set(test_libs
    MLIRAffineTransformsTestPasses
    MLIRShapeTestPasses
    MLIRSPIRVTestPasses
    MLIRTestAnalysis
    MLIRTestDialect
    MLIRTestIR
    MLIRTestPass
    MLIRTestReducer
    MLIRTestRewrite
    MLIRTestTransformDialect
    MLIRTestTransforms
    )
endif()

set(LIBS
  ${dialect_libs}
  ${conversion_libs}
  ${test_libs}
  MLIRAffineAnalysis
  MLIRAnalysis
  MLIRDialect
  MLIRLspServerLib
  MLIRParser
  MLIRPass
  MLIRTransforms
  MLIRTransformUtils
  MLIRSupport
  MLIRIR
  )

#add_mlir_tool(michelson-mlir-lsp-server
#  michelson-mlir-lsp-server.cpp
#
#  DEPENDS
#  ${LIBS}
#  )

add_llvm_executable(michelson-mlir-lsp-server michelson-mlir-lsp-server.cpp)
target_link_libraries(michelson-mlir-lsp-server PRIVATE ${LIBS})
llvm_update_compile_flags(michelson-mlir-lsp-server)

mlir_check_all_link_libraries(michelson-mlir-lsp-server)
