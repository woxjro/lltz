#ifndef Michelson_OPS
#define Michelson_OPS

include "MichelsonDialect.td"
include "mlir/Interfaces/InferTypeOpInterface.td"
include "mlir/Interfaces/SideEffectInterfaces.td"


def OperationType : OpaqueType<"michelson", "Operation", "opaque operation type">;
def MutezType : SI<64>;
def IntType : SI<128>;
def NatType : UI<128>;

def MichelsonGetAmount: MichelsonOp<"get_amount", []> {
    let summary = "Get the amount of the current transaction.";
    let description = [{
        The `michelson.get_amount` operation returns the amount of the current transaction.

        Example:
        ```mlir
        %0 = michelson.get_amount : mutez
        ```
    }];

    let arguments = (ins);
    let results = (outs MutezType:$res);

    let assemblyFormat = [{
        attr-dict `:` type($res)
    }];
}

#endif // Michelson_OPS
