module {
  func.func @smart_contract(%parameter: !michelson.int, %storage: !michelson.bytes)
    -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.bytes> {

    %byt = "michelson.get_bytes"(%parameter) : (!michelson.int) -> !michelson.bytes
    %hash = "michelson.sha256"(%byt) : (!michelson.bytes) -> !michelson.bytes
    %operations = "michelson.make_list"() : () -> !michelson.list<!michelson.operation>

    %res = "michelson.make_pair"(%operations, %hash) :
      (!michelson.list<!michelson.operation> , !michelson.bytes)
        -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.bytes>

    return %res : !michelson.pair<!michelson.list<!michelson.operation>, !michelson.bytes>
  }
}

