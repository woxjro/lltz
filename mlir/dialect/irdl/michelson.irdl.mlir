module {
  irdl.dialect @michelson {

    ///////////////////////////////
    /////////   Types     /////////
    ///////////////////////////////
    irdl.type @mutez {}

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
      %0 = irdl.any
      %1 = irdl.parametric @unit<%0>
      irdl.operands()
      irdl.results(%0)
    }

    irdl.operation @get_amount {
      %0 = irdl.any
      %1 = irdl.parametric @mutez<%0>
      irdl.operands()
      irdl.results(%0)
    }

  }
}
