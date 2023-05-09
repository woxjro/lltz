module {
  func.func @smart_contract(%storage: !michelson.unit, %parameter: !michelson.unit)
    -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.unit> {

    %unit = "michelson.get_unit"() : () -> !michelson.unit
    %operations = "michelson.make_list"() : () -> !michelson.list<!michelson.operation>

    %res = "michelson.make_pair"(%operations, %unit) :
      (!michelson.list<!michelson.operation> , !michelson.unit)
        -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.unit>

    return %res : !michelson.pair<!michelson.list<!michelson.operation>, !michelson.unit>
  }
}
