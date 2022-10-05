pub enum Ty {
    //Address,
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
    //Mutez,
    //Nat,
    //Never,
    //Operation,
    //Option { ty: Ty },
    //Or{ ty1, ty2},
    //Pair {ty1, ty2},
    //Sapling_state {n},
    //Sapling_transaction {n},
    //Set cty,
    //Signature,
    //String,
    //Ticket cty,
    //Timepstamp,
    Unit,
}

fn to_string(ty: Ty) -> String {
    match ty {
        Ty::BigMap { kty, vty } => {
            format!("(big_map {} {})", to_string(*kty), to_string(*vty))
        }
        Ty::Bool => String::from("bool"),
        Ty::Int => String::from("int"),
        Ty::Unit => String::from("unit"),
    }
}
