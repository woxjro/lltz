pub enum Ty {
    Address,
    BigMap { kty: Box<Ty>, vty: Box<Ty> },
    //Bls12_381_fr,
    //Bls12_381_g1,
    //Bls12_381_g2,
    Bool,
    //Bytes,
    //Chain_id,
    //Contract { ty: Ty },
    Int,
    //Key,
    //Key_hash,
    //Lambda { ty1: Ty, ty2: Ty },
    //List { ty: Type },
    //Map { kty: Ty, vty: Ty },
    Mutez,
    Nat,
    //Never,
    Operation,
    //Option { ty: Ty },
    //Or{ ty1, ty2},
    //Pair {ty1, ty2},
    //Sapling_state {n},
    //Sapling_transaction {n},
    //Set cty,
    //Signature,
    String,
    //Ticket cty,
    //Timepstamp,
    Unit,
}

impl Ty {
    pub fn to_string(&self) -> String {
        match self {
            Ty::Address => "address".to_string(),
            Ty::BigMap { kty, vty } => {
                format!("(big_map {} {})", kty.to_string(), vty.to_string())
            }
            //Bls12_381_fr,
            //Bls12_381_g1,
            //Bls12_381_g2,
            Ty::Bool => "bool".to_string(),
            //Chain_id,
            //Contract { ty: Ty },
            Ty::Int => "int".to_string(), //Bytes,
            //Key,
            //Key_hash,
            //Lambda { ty1: Ty, ty2: Ty },
            //List { ty: Type },
            //Map { kty: Ty, vty: Ty },
            Ty::Mutez => "mutez".to_string(),
            Ty::Nat => "nat".to_string(),
            //Never,
            Ty::Operation => "operation".to_string(),
            //Option { ty: Ty },
            //Or{ ty1, ty2},
            //Pair {ty1, ty2},
            //Sapling_state {n},
            //Sapling_transaction {n},
            //Set cty,
            //Signature,
            Ty::String => "string".to_string(),
            //Ticket cty,
            //Timepstamp,
            Ty::Unit => "unit".to_string(),
        }
    }
}