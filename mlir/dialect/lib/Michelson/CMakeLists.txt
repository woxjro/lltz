add_mlir_dialect_library(MLIRMichelson
        MichelsonDialect.cpp
        MichelsonOps.cpp

        ADDITIONAL_HEADER_DIRS
        ${PROJECT_SOURCE_DIR}/include/Michelson

        DEPENDS
        MLIRMichelsonOpsIncGen

        LINK_LIBS PUBLIC
        MLIRIR
    )
