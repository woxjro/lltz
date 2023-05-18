module {
  func.func @smart_contract(%parameter: !michelson.unit, %storage: !michelson.unit)
    -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.unit> {

    %amount = "michelson.get_amount"() : () -> !michelson.mutez
    %nil = "michelson.make_list"() : () -> !michelson.list<!michelson.operation>
    %address = "michelson.get_source"() : () -> !michelson.address
    %some_contract = "michelson.get_contract"(%address) :
      (!michelson.address) -> !michelson.option<!michelson.contract<!michelson.unit>>

    %contract = "michelson.assert_some"(%some_contract) :
      (!michelson.option<!michelson.contract<!michelson.unit>>) -> !michelson.contract<!michelson.unit>

    %operation = "michelson.transfer_tokens"(%parameter, %amount, %contract) :
      (!michelson.unit, !michelson.mutez, !michelson.contract<!michelson.unit>) -> !michelson.operation

    %operations = "michelson.cons"(%nil, %operation) :
      (!michelson.list<!michelson.operation>, !michelson.operation) -> !michelson.list<!michelson.operation>


    %res = "michelson.make_pair"(%operations, %parameter) :
      (!michelson.list<!michelson.operation> , !michelson.unit)
        -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.unit>

    return %res : !michelson.pair<!michelson.list<!michelson.operation>, !michelson.unit>
  }
}
