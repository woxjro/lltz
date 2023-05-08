module {
  func.func @smart_contract(%storage: !michelson.mutez, %parameter: !michelson.mutez)
    -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.mutez> {

    %amount = "michelson.get_amount"() : () -> !michelson.mutez
    %operations = "michelson.make_list"() : () -> !michelson.list<!michelson.operation>

    %res = "michelson.make_pair"(%operations, %amount) :
      (!michelson.list<!michelson.operation> , !michelson.mutez)
        -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.mutez>

    return %res : !michelson.pair<!michelson.list<!michelson.operation>, !michelson.mutez>
  }
}
