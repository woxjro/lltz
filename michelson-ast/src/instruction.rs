pub enum Instruction {
    ////////////////////////////////////////////////
    ////////////////Control Structures//////////////
    ////////////////////////////////////////////////
    Apply,
    Exec,
    Failwith,
    //If { instr1: Vec<Instruction>, instr2: Vec<Instruction>, },
    //IF_CONS instr1 instr2,
    //IF_LEFT instr1 instr2,
    //IF_NONE instr1 instr2,
    //ITER inster,
    //LAMBDA ty1 ty2 instr,
    //LOOP instr,
    //LOOP_LEFT instr,
    //instr1 ; instr2,
    //{},
    ////////////////////////////////////////////////
    //////////Operations on data structures/////////
    ////////////////////////////////////////////////
    Car,
    Cdr,
    Concat,
    Cons,
    EmptyBigMap,
    EmptyMap,
    EmptySet,
    Get,
    //GetN { n: usize, },
    GetAndUpdate,
    //LEFT ty2,
    //MAP instr,
    Mem,
    Never,
    //NIL ty,
    //NONE ty,
    Pack,
    Pair,
    //PAIR n,
    //RIGHT ty1,
    Size,
    Slice,
    Some,
    Unit,
    //UNPACK ty,
    Unpair,
    //UNPAIR n,
    Update,
    //UPDATE n,
    ////////////////////////////////////////////////
    /////////////Blockchain operations//////////////
    ////////////////////////////////////////////////
    Address,
    Amount,
    Balance,
    ChainId,
    ////CONTRACT ty,
    //CREATE_CONTRACT { parameter ty1; storage ty2; code instr1 },
    ImplicitAccount,
    Level,
    Now,
    //Self,
    SelfAddress,
    Sender,
    SetDelegate,
    Source,
    TotalVotingPower,
    TransferTokens,
    VotingPower,
    ////////////////////////////////////////////////
    ////////////Operations on tickets///////////////
    ////////////////////////////////////////////////
    JointTickets,
    ReadTicket,
    SplitTicket,
    Ticket,
    ////////////////////////////////////////////////
    ////////////Cryptographic operations////////////
    ////////////////////////////////////////////////
    Blake2b,
    CheckSignature,
    HashKey,
    Keccak,
    PairingCheck,
    SaplingEmptyState,
    SaplingVerifyUpdate,
    Sha256,
    Sha3,
    Sha512,
    ////////////////////////////////////////////////
    //////////////Boolean operations////////////////
    ////////////////////////////////////////////////
    And,
    Not,
    Or,
    Xor,
    ////////////////////////////////////////////////
    ////////////Arithmetic operations///////////////
    ////////////////////////////////////////////////
    Abs,
    Add,
    Compare,
    Ediv,
    Eq,
    Ge,
    Gt,
    Int,
    Isnat,
    Le,
    Lsl,
    Lsr,
    Lt,
    Mul,
    Neg,
    Neq,
    Sub,
    ////////////////////////////////////////////////
    /////////////Stack manipulation/////////////////
    ////////////////////////////////////////////////
    //DigN { n: usize },
    //DugN { n: usize },
    Dip,
    //DipN { n: usize },
    Dup,
    //DupN { n: usize },
    Push,
    Drop,
    Swap,
}

impl Instruction {
    pub fn to_string(&self) -> String {
        match self {
            ////////////////////////////////////////////////
            ////////////////Control Structures//////////////
            ////////////////////////////////////////////////
            Instruction::Apply => "APPLY".to_string(),
            Instruction::Exec => "EXEC".to_string(),
            Instruction::Failwith => "FAILWITH".to_string(),
            //Instruction::If { instr1, instr2 } => "IF".to_string(),
            //IF_CONS instr1 instr2,
            //IF_LEFT instr1 instr2,
            //IF_NONE instr1 instr2,
            //ITER inster,
            //LAMBDA ty1 ty2 instr,
            //LOOP instr,
            //LOOP_LEFT instr,
            //instr1 ; instr2,
            //{},
            ////////////////////////////////////////////////
            //////////Operations on data structures/////////
            ////////////////////////////////////////////////
            Instruction::Car => "CAR".to_string(),
            Instruction::Cdr => "CDR".to_string(),
            Instruction::Concat => "CONCAT".to_string(),
            Instruction::Cons => "CONS".to_string(),
            Instruction::EmptyBigMap => "EMPTY_BIG_MAP".to_string(),
            Instruction::EmptyMap => "EMPTY_MAP".to_string(),
            Instruction::EmptySet => "EMPTY_SET".to_string(),
            Instruction::Get => "GET".to_string(),
            //GET { n: usize, },
            Instruction::GetAndUpdate => "GET_AND_UPDATE".to_string(),
            //LEFT ty2,
            //MAP instr,
            Instruction::Mem => "MEM".to_string(),
            Instruction::Never => "NEVER".to_string(),
            //NIL ty,
            //NONE ty,
            Instruction::Pack => "PACK".to_string(),
            Instruction::Pair => "PAIR".to_string(),
            //PAIR n,
            //RIGHT ty1,
            Instruction::Size => "SIZE".to_string(),
            Instruction::Slice => "SLICE".to_string(),
            Instruction::Some => "SOME".to_string(),
            Instruction::Unit => "UNIT".to_string(),
            //UNPACK ty,
            Instruction::Unpair => "UNPAIR".to_string(),
            //UNPAIR n,
            Instruction::Update => "UPDATE".to_string(),
            //UPDATE n,
            ////////////////////////////////////////////////
            /////////////Blockchain operations//////////////
            ////////////////////////////////////////////////
            Instruction::Address => "ADDRESS".to_string(),
            Instruction::Amount => "AMOUNT".to_string(),
            Instruction::Balance => "BALANCE".to_string(),
            Instruction::ChainId => "CHAIN_ID".to_string(),
            ////CONTRACT ty,
            //CREATE_CONTRACT { parameter ty1; storage ty2; code instr1 },
            Instruction::ImplicitAccount => "IMPLICIT_ACCOUNT".to_string(),
            Instruction::Level => "LEVEL".to_string(),
            Instruction::Now => "NOW".to_string(),
            //Instruction::Self => "SELF".to_string(),
            Instruction::SelfAddress => "SELF_ADDRESS".to_string(),
            Instruction::Sender => "SENDER".to_string(),
            Instruction::SetDelegate => "SET_DELEGATE".to_string(),
            Instruction::Source => "SOURCE".to_string(),
            Instruction::TotalVotingPower => "TOTAL_VOTING_POWER".to_string(),
            Instruction::TransferTokens => "TRANSFER_TOKENS".to_string(),
            Instruction::VotingPower => "VOTING_POWER".to_string(),
            ////Operations on tickets
            Instruction::JointTickets => "JOINT_TICKETS".to_string(),
            Instruction::ReadTicket => "READ_TICKET".to_string(),
            Instruction::SplitTicket => "SPLIT_TICKET".to_string(),
            Instruction::Ticket => "TICKET".to_string(),
            //Cryptographic operations
            Instruction::Blake2b => "BLAKE2B".to_string(),
            Instruction::CheckSignature => "CHECK_SIGNATURE".to_string(),
            Instruction::HashKey => "HASH_KEY".to_string(),
            Instruction::Keccak => "KECCAK".to_string(),
            Instruction::PairingCheck => "PAIRING_CHECK".to_string(),
            Instruction::SaplingEmptyState => "SAPLING_EMPTY_STATE".to_string(),
            Instruction::SaplingVerifyUpdate => "SAPLING_VERIFY_UPDATE".to_string(),
            Instruction::Sha256 => "SHA256".to_string(),
            Instruction::Sha3 => "SHA3".to_string(),
            Instruction::Sha512 => "SHA512".to_string(),
            //Boolean operations
            Instruction::And => "AND".to_string(),
            Instruction::Not => "NOT".to_string(),
            Instruction::Or => "OR".to_string(),
            Instruction::Xor => "XOR".to_string(),
            ////////////////////////////////////////////////
            ////////////Arithmetic operations///////////////
            ////////////////////////////////////////////////
            Instruction::Abs => "ABS".to_string(),
            Instruction::Add => "ADD".to_string(),
            Instruction::Compare => "COMPARE".to_string(),
            Instruction::Ediv => "EDIV".to_string(),
            Instruction::Eq => "EQ".to_string(),
            Instruction::Ge => "GE".to_string(),
            Instruction::Gt => "GT".to_string(),
            Instruction::Int => "INT".to_string(),
            Instruction::Isnat => "ISNAT".to_string(),
            Instruction::Le => "LE".to_string(),
            Instruction::Lsl => "LSL".to_string(),
            Instruction::Lsr => "LSR".to_string(),
            Instruction::Lt => "LT".to_string(),
            Instruction::Mul => "MUL".to_string(),
            Instruction::Neg => "NEG".to_string(),
            Instruction::Neq => "NEQ".to_string(),
            Instruction::Sub => "SUB".to_string(),
            ////////////////////////////////////////////////
            /////////////Stack manipulation/////////////////
            ////////////////////////////////////////////////
            //DIG { n: usize, },
            //DUG { n: usize, },
            Instruction::Dip => "DIP".to_string(),
            //DIP { n: usize, },
            Instruction::Dup => "DUP".to_string(),
            //DUP { n: usize, },
            Instruction::Push => "PUSH".to_string(),
            Instruction::Drop => "DROP".to_string(),
            Instruction::Swap => "SWAP".to_string(),
        }
    }
}
