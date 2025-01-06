module {
  func.func @smart_contract(%param: !michelson.int, %storage: !michelson.bytes) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.bytes> {
    %byt = "michelson.get_bytes"(%param): (!michelson.int) -> !michelson.bytes
    %hash = "michelson.sha256"(%byt): (!michelson.bytes) -> !michelson.bytes
    %nil = "michelson.make_list"(): () -> !michelson.list<!michelson.operation>
    %p = "michelson.make_pair"(%nil, %hash): (!michelson.list<!michelson.operation>, !michelson.bytes) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.bytes>
    return %p: !michelson.pair<!michelson.list<!michelson.operation>, !michelson.bytes>
  }
}
