cmake_minimum_required(VERSION 3.13.4)
project(chibichelson LANGUAGES CXX C)


set(CMAKE_BUILD_WITH_INSTALL_NAME_DIR ON)

set(CMAKE_CXX_STANDARD 17 CACHE STRING "C++ standard to conform to")

find_package(MLIR REQUIRED CONFIG)
find_package(LLVM REQUIRED CONFIG)

message(STATUS "Using MLIRConfig.cmake in: ${MLIR_DIR}")
message(STATUS "Using LLVMConfig.cmake in: ${LLVM_DIR}")
message(STATUS "CMAKE_BINARY_DIR: ${CMAKE_BINARY_DIR}")
message(STATUS "LLVM_INCLUDE_DIRS: ${LLVM_INCLUDE_DIRS}")
message(STATUS "PROJECT_SOURCE_DIR: ${PROJECT_SOURCE_DIR}")

set(LLVM_RUNTIME_OUTPUT_INTDIR ${CMAKE_BINARY_DIR}/bin)
set(LLVM_LIBRARY_OUTPUT_INTDIR ${CMAKE_BINARY_DIR}/lib)
set(MLIR_BINARY_DIR ${CMAKE_BINARY_DIR})
set(CMAKE_EXE_LINKER_FLAGS ${CMAKE_EXE_LINKER_FLAGS})
list(APPEND CMAKE_MODULE_PATH "${MLIR_CMAKE_DIR}")
list(APPEND CMAKE_MODULE_PATH "${LLVM_CMAKE_DIR}")
include(TableGen)
include(AddLLVM)
include(AddMLIR)
include(HandleLLVMOptions)

#if(MLIR_ENABLE_BINDINGS_PYTHON)
#  include(MLIRDetectPythonEnv)
#  mlir_configure_python_dev_packages()
#endif()

include_directories(${LLVM_INCLUDE_DIRS})
include_directories(${MLIR_INCLUDE_DIRS})
include_directories(${PROJECT_SOURCE_DIR}/include)
include_directories(${PROJECT_BINARY_DIR}/include)
link_directories(${LLVM_BUILD_LIBRARY_DIR})
add_definitions(${LLVM_DEFINITIONS})


# For a better template to copy, see examples/standalone
set(LLVM_LINK_COMPONENTS
  Support
  Option
  )

set(SOURCES
    chibichelsonc.cpp
    parser/AST.cpp
    include/chibichelson/AST.h
    include/chibichelson/Lexer.h
    include/chibichelson/Parser.h
)

include_directories(include/)
add_compile_options(-fno-rtti)
add_executable(chibichelsonc ${SOURCES})
target_link_libraries(chibichelsonc
  PRIVATE
    MLIRSupport
)
#if(MLIR_ENABLE_BINDINGS_PYTHON)
#  message(STATUS "Enabling Python API")
#  add_subdirectory(python)
#endif()
#add_subdirectory(test)
#add_subdirectory(michelson-opt)
#add_subdirectory(michelson-translate)
