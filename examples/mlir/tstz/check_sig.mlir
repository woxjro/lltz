module {
  func.func @smart_contract(%param: !michelson.key, %storage: !michelson.pair<!michelson.signature, !michelson.string>) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.pair<!michelson.signature, !michelson.string>> {
    %signature = "michelson.get_fst"(%storage): (!michelson.pair<!michelson.signature, !michelson.string>) -> !michelson.signature
    %str = "michelson.get_snd"(%storage): (!michelson.pair<!michelson.signature, !michelson.string>) -> !michelson.string
    %byt = "michelson.pack"(%str): (!michelson.string) -> !michelson.bytes
    %result = "michelson.check_signature"(%param, %signature, %byt): (!michelson.key, !michelson.signature, !michelson.bytes) -> !michelson.bool
    "michelson.assert"(%result): (!michelson.bool) -> ()
    %nil = "michelson.make_list"(): () -> !michelson.list<!michelson.operation>
    %p = "michelson.make_pair"(%nil, %storage): (!michelson.list<!michelson.operation>, !michelson.pair<!michelson.signature, !michelson.string>) -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.pair<!michelson.signature, !michelson.string>>
    return %p: !michelson.pair<!michelson.list<!michelson.operation>, !michelson.pair<!michelson.signature, !michelson.string>>
  }
}
