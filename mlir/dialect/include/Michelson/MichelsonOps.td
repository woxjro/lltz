#ifndef Michelson_OPS
#define Michelson_OPS

include "MichelsonDialect.td"
include "mlir/Interfaces/SideEffectInterfaces.td"

def OperationType : OpaqueType<"michelson", "Operation", "opaque operation type">;
def MutezType : SI<64>;
def IntType : SI<128>;
def NatType : UI<128>;

def NumericType: AnyTypeOf<[MutezType, IntType, NatType]>;

def MichelsonType : AnyTypeOf<[OperationType, NumericType]>;


def AddOp: MichelsonOp<"add", [Pure]> {
    let summary = "addition operation";
    let description = [{ }];

    let arguments = (ins NumericType:$lhs, NumericType:$rhs);
    let results = (outs NumericType);

    // Indicate that the operation has a custom parser and printer method.
    let hasCustomAssemblyFormat = 1;

    // Allow building an AddOp with from the two input operands.
    let builders = [
        OpBuilder<(ins "Value":$lhs, "Value":$rhs)>
    ];
}

//def GetAmount: MichelsonOp<"get_amount", [Pure]> {
//    let summary = "Get the amount of the current transaction.";
//    let description = [{
//        The `michelson.get_amount` operation returns the amount of the current transaction.
//
//        Example:
//        ```mlir
//        %0 = michelson.get_amount : mutez
//        ```
//    }];
//
//    let arguments = (ins);
//    let results = (outs MutezType:$res);
//
//    let assemblyFormat = [{
//        attr-dict `:` type($res)
//    }];
//}

#endif // Michelson_OPS
