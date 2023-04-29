#ifndef MICHELSON_DIALECT
#define MICHELSON_DIALECT

include "mlir/IR/OpBase.td"

//===----------------------------------------------------------------------===//
// Michelson dialect definition.
//===----------------------------------------------------------------------===//

def MichelsonDialect : Dialect {
    let name = "michelson";
    let summary = "A MLIR dialect for michelson assembly languages.";
    let description = [{
        This dialect provides operations specific to michelson assembly languages.
    }];
    let cppNamespace = "::mlir::michelson";
}

//===----------------------------------------------------------------------===//
// Base standalone operation definition.
//===----------------------------------------------------------------------===//

class MichelsonOp<string mnemonic, list<Trait> traits = []> :
        Op<MichelsonDialect, mnemonic, traits>;

#endif // MICHELSON_DIALECT
