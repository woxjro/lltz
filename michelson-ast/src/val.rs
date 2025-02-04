#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Val {
    Address(String),
    //BigMap { kty: Box<Ty>, vty: Box<Ty> },
    //Bls12_381_fr,
    //Bls12_381_g1,
    //Bls12_381_g2,
    Bool(bool),
    //Bytes,
    //Chain_id,
    //Contract { ty: Ty },
    Int(i128),
    Key(String),
    //Key_hash,
    //Lambda { ty1: Ty, ty2: Ty },
    //List { ty: Type },
    //Map { kty: Ty, vty: Ty },
    Mutez(i128),
    Nat(u128),
    //Never,
    //Operation,
    //Option { ty: Ty },
    //Or{ ty1, ty2},
    //Pair {ty1, ty2},
    //Sapling_state {n},
    //Sapling_transaction {n},
    //Set cty,
    Signature(String),
    String(String),
    //Ticket cty,
    //Timepstamp,
    //Unit,
}

impl ToString for Val {
    fn to_string(&self) -> String {
        match self {
            Val::Address(addr) => format!("\"{}\"", addr.clone()),
            Val::Bool(b) => if *b { "True" } else { "False" }.to_string(),
            Val::Int(i) => i.to_string(),
            Val::Mutez(m) => m.to_string(),
            Val::Nat(n) => n.to_string(),
            Val::String(s) => format!("\"{}\"", s.clone()),
            Val::Key(k) => format!("\"{}\"", k.clone()),
            Val::Signature(s) => format!("\"{}\"", s.clone()),
            // v => todo!("{:?} is not implemented", v),
        }
    }
}
