module {
  irdl.dialect @michelson {
    ///////////////////////////////
    /////////   Types     /////////
    ///////////////////////////////
    irdl.type @address{}

    irdl.type @mutez {}

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

      %res = irdl.is @pair

      irdl.operands(%fst, %snd)
      irdl.results(%res)
    }

    irdl.operation @make_list {
      %elem = irdl.any

      %res = irdl.is @list

      irdl.operands(%elem)
      irdl.results(%res)
    }

    irdl.operation @get_unit {
      %0 = irdl.is @unit
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
      %source = irdl.is @address
      irdl.operands()
      irdl.results(%source)
    }

    irdl.operation @get_contract {
      %addr = irdl.is @address
      %param = irdl.any
      %res = irdl.parametric @contract<%param>
      irdl.operands(%addr)
      irdl.results(%res)
    }

    irdl.operation @transfer_tokens {
      %tokens = irdl.is @mutez
      %parameter = irdl.any
      %ct = irdl.parametric @contract<%parameter>
      %res = irdl.is @operation
      irdl.operands(%parameter, %tokens, %ct)
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
