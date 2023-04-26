get_property(dialect_libs GLOBAL PROPERTY MLIR_DIALECT_LIBS)
get_property(conversion_libs GLOBAL PROPERTY MLIR_CONVERSION_LIBS)
set(LIBS
        ${dialect_libs}
        ${conversion_libs}
        MLIRArithDialect
        MLIROptLib
        MLIRMichelson
        )
add_llvm_executable(michelson-mlir-opt michelson-mlir-opt.cpp)

llvm_update_compile_flags(michelson-mlir-opt)
target_link_libraries(michelson-mlir-opt PRIVATE ${LIBS})

mlir_check_all_link_libraries(michelson-mlir-opt)
