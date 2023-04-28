add_mlir_library(MLIRJsonDump
  JsonDump.cpp

  EXCLUDE_FROM_LIBMLIR

  LINK_LIBS PUBLIC
  MLIRPass
  MLIRTestDialect
  )

#target_include_directories(MLIRTestIR
#  PRIVATE
#  ${CMAKE_CURRENT_SOURCE_DIR}/../Dialect/Test
#  ${CMAKE_CURRENT_BINARY_DIR}/../Dialect/Test
#  )
