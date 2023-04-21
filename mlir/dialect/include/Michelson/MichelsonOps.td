#ifndef Michelson_OPS
#define Michelson_OPS

include "MichelsonDialect.td"
include "MichelsonTypes.td"
include "mlir/Interfaces/SideEffectInterfaces.td"

/*
    def AddressType : OpaqueType<"michelson", "Address", "opaque address type">;
    def OperationType : OpaqueType<"michelson", "Operation", "opaque operation type">;
    def ContractType : OpaqueType<"michelson", "Contract", "opaque contract type">;
    def Mutez : SI<64>;
    def Int : SI<128>;
    def Nat : UI<128>;

    def NumericType: AnyTypeOf<[Mutez, Int, Nat]>;

    def MichelsonType : AnyTypeOf<[
                            OperationType,
                            ContractType,
                            NumericType,
                        ]>;

    //TODO: ConstantOp, FuncOp, PairType
    def AddOp: MichelsonOp<"add", [Pure]> {
        let summary = "addition operation";
        let description = [{ }];

        //let arguments = (ins NumericType:$lhs, NumericType:$rhs);
        //let results = (outs NumericType);

        // Indicate that the operation has a custom parser and printer method.
        let hasCustomAssemblyFormat = 1;

        // Allow building an AddOp with from the two input operands.
        /*
        let builders = [
            OpBuilder<(ins "Value":$lhs, "Value":$rhs)>
        ];
        */
    }
*/

def GetAmount: MichelsonOp<"get_amount", [Pure]> {
    let summary = "Get the amount of the current transaction.";
    let description = [{
        The `michelson.get_amount` operation returns the amount of the current transaction.

        Example:
        ```mlir
        %0 = michelson.get_amount() : !michelson.mutez
        ```
    }];

    //let arguments = (ins);
    //let results = (outs MichelsonMutezType:$res);
}

#endif // Michelson_OPS
