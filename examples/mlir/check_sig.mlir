module {
  func.func @smart_contract(%parameter: !michelson.key, %storage: !michelson.pair<!michelson.signature, !michelson.string>)
    -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.pair<!michelson.signature, !michelson.string>> {

    %signature = "michelson.get_fst"(%storage) : (!michelson.pair<!michelson.signature, !michelson.string>) -> !michelson.signature
    %string = "michelson.get_snd"(%storage) : (!michelson.pair<!michelson.signature, !michelson.string>) -> !michelson.string
    %bytes = "michelson.pack"(%string) : (!michelson.string) -> !michelson.bytes

    %result = "michelson.check_signature"(%parameter, %signature, %bytes) : (!michelson.key, !michelson.signature, !michelson.bytes) -> !michelson.bool

    "michelson.assert"(%result) : (!michelson.bool) -> ()

    %nil = "michelson.make_list"() : () -> !michelson.list<!michelson.operation>
    %pair = "michelson.make_pair"(%nil, %storage) :
      (!michelson.list<!michelson.operation> , !michelson.pair<!michelson.signature, !michelson.string>)
        -> !michelson.pair<!michelson.list<!michelson.operation>, !michelson.pair<!michelson.signature, !michelson.string>>

    return %pair: !michelson.pair<!michelson.list<!michelson.operation>, !michelson.pair<!michelson.signature, !michelson.string>>
  }
}
