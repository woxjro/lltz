module {
  irdl.dialect @michelson {
    ///////////////////////////////
    /////////   Types     /////////
    ///////////////////////////////
    irdl.type @address {}

    irdl.type @bytes {}

    irdl.type @mutez {}

    irdl.type @int {}

    irdl.type @nat {}

    irdl.type @contract {
      %parameter = irdl.any

      irdl.parameters(%parameter)
    }

    irdl.type @operation {}

    irdl.type @unit {}

    irdl.type @pair {
      %fst = irdl.any
      %snd = irdl.any

      irdl.parameters(%fst, %snd)
    }

    irdl.type @list {
      %elem = irdl.any
      irdl.parameters(%elem)
    }

    irdl.type @option {
      %elem = irdl.any
      irdl.parameters(%elem)
    }

    ////////////////////////////////
    ////////   Operations  /////////
    ////////////////////////////////
    irdl.operation @make_pair {
      %fst = irdl.any
      %snd = irdl.any

      %res = irdl.any

      irdl.operands(%fst, %snd)
      irdl.results(%res)
    }

    irdl.operation @make_list {
      %res = irdl.any

      irdl.operands()
      irdl.results(%res)
    }

    irdl.operation @get_unit {
      %0 = irdl.any
      irdl.operands()
      irdl.results(%0)
    }

    irdl.operation @cons {
      %e = irdl.any
      %l = irdl.parametric @list<%e>
      irdl.operands(%l, %e)
      irdl.results(%l)
    }

    // Blockchain Operations
    irdl.operation @get_amount {
      %0 = irdl.any
      irdl.operands()
      irdl.results(%0)
    }

    irdl.operation @get_source {
      %source = irdl.any
      irdl.operands()
      irdl.results(%source)
    }

    irdl.operation @get_contract {
      %addr = irdl.any
      %param = irdl.any
      %contract = irdl.parametric @contract<%param>
      %res = irdl.parametric @option<%contract>
      irdl.operands(%addr)
      irdl.results(%res)
    }

    irdl.operation @transfer_tokens {
      %tokens = irdl.any
      %parameter = irdl.any
      %ct = irdl.parametric @contract<%parameter>
      %res = irdl.any
      irdl.operands(%parameter, %tokens, %ct)
      irdl.results(%res)
    }

    // Cryptographic operations
    irdl.operation @sha256 {
      %byt = irdl.any
      %res = irdl.any

      irdl.operands(%byt)
      irdl.results(%res)
    }

    irdl.operation @sha3 {
      %byt = irdl.any
      %res = irdl.any

      irdl.operands(%byt)
      irdl.results(%res)
    }

    irdl.operation @sha512 {
      %byt = irdl.any
      %res = irdl.any

      irdl.operands(%byt)
      irdl.results(%res)
    }

    // Macros
    irdl.operation @assert_some {
      %inner = irdl.any
      %op = irdl.parametric @option<%inner>
      irdl.operands(%op)
      irdl.results(%inner)
    }
  }
}
