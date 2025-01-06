module {
  func.func @smart_contract(%param: !michelson.int, %storage: !michelson.bytes) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.bytes> {
    %byt = "michelson.get_bytes"(%param): (!michelson.int) -> !michelson.bytes
    %hashed_param = "michelson.sha256"(%byt): (!michelson.bytes) -> !michelson.bytes
    %nil = "michelson.make_list"(): () -> !michelson.list<!michelson.operation>
    %p = "michelson.make_pair"(%nil, %hashed_param): (!michelson.list<!michelson.operation>, !michelson.bytes) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.bytes>
    return %p: !michelson.pair<!michelson.list<!michelson.operation>, !michelson.bytes>
  }
}
