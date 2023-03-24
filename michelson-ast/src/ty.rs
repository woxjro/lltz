use std::string::ToString;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Ty {
    Address,
    BigMap { kty: Box<Ty>, vty: Box<Ty> },
    Bls12_381Fr,
    Bls12_381G1,
    Bls12_381G2,
    Bool,
    Bytes,
    ChainId,
    Contract { ty: Box<Ty> },
    Int,
    Key,
    KeyHash,
    Lambda { ty1: Box<Ty>, ty2: Box<Ty> },
    List { ty: Box<Ty> },
    Map { kty: Box<Ty>, vty: Box<Ty> },
    Mutez,
    Nat,
    Never,
    Operation,
    Option { ty: Box<Ty> },
    Or { ty1: Box<Ty>, ty2: Box<Ty> },
    Pair { ty1: Box<Ty>, ty2: Box<Ty> },
    //Sapling_state {n},
    //Sapling_transaction {n},
    Set { cty: Box<Ty> },
    Signature,
    String,
    Ticket { cty: Box<Ty> },
    Timestamp,
    Unit,
}

impl ToString for Ty {
    fn to_string(&self) -> String {
        match self {
            Ty::Address => "address".to_string(),
            Ty::BigMap { kty, vty } => {
                format!("(big_map {} {})", kty.to_string(), vty.to_string())
            }
            Ty::Bls12_381Fr => "bls12_381_fr".to_string(),
            Ty::Bls12_381G1 => "Bls12_381_g1".to_string(),
            Ty::Bls12_381G2 => "Bls12_381_g2".to_string(),
            Ty::Bool => "bool".to_string(),
            Ty::ChainId => "chain_id".to_string(),
            Ty::Contract { ty } => format!("(contract {})", ty.to_string()),
            Ty::Int => "int".to_string(),
            Ty::Bytes => "bytes".to_string(),
            Ty::Key => "key".to_string(),
            Ty::KeyHash => "key_hash".to_string(),
            Ty::Lambda { .. } => todo!(),
            Ty::List { .. } => todo!(),
            Ty::Map { kty, vty } => {
                format!("(map {} {})", kty.to_string(), vty.to_string())
            }
            Ty::Mutez => "mutez".to_string(),
            Ty::Nat => "nat".to_string(),
            Ty::Never => "never".to_string(),
            Ty::Operation => "operation".to_string(),
            Ty::Option { ty } => format!("(option {})", ty.to_string()),
            Ty::Or { .. } => todo!(),
            Ty::Pair { ty1, ty2 } => format!("(pair {} {})", ty1.to_string(), ty2.to_string()),
            //Sapling_state {n},
            //Sapling_transaction {n},
            Ty::Set { .. } => todo!(),
            Ty::Signature => "signature".to_string(),
            Ty::String => "string".to_string(),
            Ty::Ticket { .. } => todo!(),
            Ty::Timestamp => "timestamp".to_string(),
            Ty::Unit => "unit".to_string(),
        }
    }
}
