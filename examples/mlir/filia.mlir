module {
func.func @smart_contract(%parameter: !michelson.mutez, %storage: !michelson.mutez) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.mutez> {
%amount = "michelson.get_amount"() : () -> !michelson.mutez
%operations = "michelson.make_list"() : () -> !michelson.list<!michelson.operation>
%pair = "michelson.make_pair"(%operations, %amount) : (!michelson.list<!michelson.operation>, !michelson.mutez) -> !michelson.pair<!michelson.list<!michelson.operation>,!michelson.mutez>
return %pair : !michelson.pair<!michelson.list<!michelson.operation>,!michelson.mutez>
 }
}
