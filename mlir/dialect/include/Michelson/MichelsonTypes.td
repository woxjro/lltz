#ifndef Michelson_TYPES
#define Michelson_TYPES

include "MichelsonDialect.td"
include "mlir/IR/AttrTypeBase.td"

/// Base class for all Michelson dialect types.
class MichelsonType<string name, string typeMnemonic, list<Trait> traits = []>
    : TypeDef<MichelsonDialect, name, traits> {
  let mnemonic = typeMnemonic;
}

def Mutez: MichelsonType<"MichelsonMutez", "mutez"> {
    let summary = "Michelson mutez type";
    let description = [{
        The `!michelson.mutez` type is an Michelson mutez type.

        Example:
        ```mlir
        !michelson.mutez
        ```
    }];

    let parameters = (ins);
}


def AddressType : OpaqueType<"michelson", "Address", "opaque address type">;
def OperationType : OpaqueType<"michelson", "Operation", "opaque operation type">;
def ContractType : OpaqueType<"michelson", "Contract", "opaque contract type">;
//def Int : SI<128>;
//def Nat : UI<128>;
//
//def NumericType: AnyTypeOf<[Mutez, Int, Nat]>;
//
//def MichelsonType : AnyTypeOf<[
//                        OperationType,
//                        ContractType,
//                        NumericType,
//                    ]>;

#endif // Michelson_TYPES
