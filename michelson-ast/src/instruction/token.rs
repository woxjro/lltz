#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Token {
    Comment,
    ////////////////////////////////////////////////
    ////////////////Control Structures//////////////
    ////////////////////////////////////////////////
    Apply,
    Exec,
    Failwith,
    If,
    IfCons,
    IfLeft,
    IfNone,
    Iter,
    Lambda,
    Loop,
    LoopLeft,
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
    GetN,
    GetAndUpdate,
    Left,
    Map,
    Mem,
    Never,
    Nil,
    None,
    Pack,
    Pair,
    PairN,
    Right,
    Size,
    Slice,
    Some,
    Unit,
    Unpack,
    Unpair,
    UnpairN,
    Update,
    UpdateN,
    ////////////////////////////////////////////////
    /////////////Blockchain operations//////////////
    ////////////////////////////////////////////////
    Address,
    Amount,
    Balance,
    ChainId,
    Contract,
    CreateContract,
    ImplicitAccount,
    Level,
    Now,
    Slf, // Self
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
    Bytes,
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
    Nat,
    Neg,
    Neq,
    Sub,
    ////////////////////////////////////////////////
    /////////////Stack manipulation/////////////////
    ////////////////////////////////////////////////
    DigN,
    DugN,
    Dip,
    DipN,
    Dup,
    DupN,
    Push,
    Drop,
    Swap,
    ////////////////////////////////////////////////
    /////////////      Macro       /////////////////
    ////////////////////////////////////////////////
    Assert,
    AssertSome,
}

impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Comment => "#",
            ////////////////////////////////////////////////
            ////////////////Control Structures//////////////
            ////////////////////////////////////////////////
            Token::Apply => "APPLY",
            Token::Exec => "EXEC",
            Token::Failwith => "FAILWITH",
            Token::If => "IF",
            Token::IfCons => "IF_CONS",
            Token::IfLeft => "IF_LEFT",
            Token::IfNone => "IF_NONE",
            Token::Loop => "LOOP",
            Token::LoopLeft => "LOOP_LEFT",
            Token::Iter => "ITER",
            Token::Lambda => "LAMBDA",
            ////////////////////////////////////////////////
            //////////Operations on data structures/////////
            ////////////////////////////////////////////////
            Token::Car => "CAR",
            Token::Cdr => "CDR",
            Token::Concat => "CONCAT",
            Token::Cons => "CONS",
            Token::EmptyBigMap => "EMPTY_BIG_MAP",
            Token::EmptyMap => "EMPTY_MAP",
            Token::EmptySet => "EMPTY_SET",
            Token::Get => "GET",
            Token::GetN => "GET",
            Token::GetAndUpdate => "GET_AND_UPDATE",
            Token::Left => "LEFT",
            Token::Map => "MAP",
            Token::Mem => "MEM",
            Token::Never => "NEVER",
            Token::Nil => "NIL",
            Token::None => "NONE",
            Token::Pack => "PACK",
            Token::Pair => "PAIR",
            Token::PairN => "PAIR",
            Token::Right => "RIGHT",
            Token::Size => "SIZE",
            Token::Slice => "SLICE",
            Token::Some => "SOME",
            Token::Unit => "UNIT",
            Token::Unpack => "UNPACK",
            Token::Unpair => "UNPAIR",
            Token::UnpairN => "UNPAIR",
            Token::Update => "UPDATE",
            Token::UpdateN => "UPDATE",
            ////////////////////////////////////////////////
            /////////////Blockchain operations//////////////
            ////////////////////////////////////////////////
            Token::Address => "ADDRESS",
            Token::Amount => "AMOUNT",
            Token::Balance => "BALANCE",
            Token::ChainId => "CHAIN_ID",
            Token::Contract => "CONTRACT",
            Token::CreateContract => "CREATE_CONTRACT",
            Token::ImplicitAccount => "IMPLICIT_ACCOUNT",
            Token::Level => "LEVEL",
            Token::Now => "NOW",
            Token::Slf => "SELF",
            Token::SelfAddress => "SELF_ADDRESS",
            Token::Sender => "SENDER",
            Token::SetDelegate => "SET_DELEGATE",
            Token::Source => "SOURCE",
            Token::TotalVotingPower => "TOTAL_VOTING_POWER",
            Token::TransferTokens => "TRANSFER_TOKENS",
            Token::VotingPower => "VOTING_POWER",
            ////////////////////////////////////////////////
            ////////////Operations on tickets///////////////
            ////////////////////////////////////////////////
            Token::JointTickets => "JOINT_TICKETS",
            Token::ReadTicket => "READ_TICKET",
            Token::SplitTicket => "SPLIT_TICKET",
            Token::Ticket => "TICKET",
            ////////////////////////////////////////////////
            ////////////Cryptographic operations////////////
            ////////////////////////////////////////////////
            Token::Blake2b => "BLAKE2B",
            Token::CheckSignature => "CHECK_SIGNATURE",
            Token::HashKey => "HASH_KEY",
            Token::Keccak => "KECCAK",
            Token::PairingCheck => "PAIRING_CHECK",
            Token::SaplingEmptyState => "SAPLING_EMPTY_STATE",
            Token::SaplingVerifyUpdate => "SAPLING_VERIFY_UPDATE",
            Token::Sha256 => "SHA256",
            Token::Sha3 => "SHA3",
            Token::Sha512 => "SHA512",
            ////////////////////////////////////////////////
            //////////////Boolean operations////////////////
            ////////////////////////////////////////////////
            Token::And => "AND",
            Token::Not => "NOT",
            Token::Or => "OR",
            Token::Xor => "XOR",
            ////////////////////////////////////////////////
            ////////////Arithmetic operations///////////////
            ////////////////////////////////////////////////
            Token::Abs => "ABS",
            Token::Add => "ADD",
            Token::Bytes => "BYTES",
            Token::Compare => "COMPARE",
            Token::Ediv => "EDIV",
            Token::Eq => "EQ",
            Token::Ge => "GE",
            Token::Gt => "GT",
            Token::Int => "INT",
            Token::Isnat => "ISNAT",
            Token::Le => "LE",
            Token::Lsl => "LSL",
            Token::Lsr => "LSR",
            Token::Lt => "LT",
            Token::Mul => "MUL",
            Token::Nat => "NAT",
            Token::Neg => "NEG",
            Token::Neq => "NEQ",
            Token::Sub => "SUB",
            ////////////////////////////////////////////////
            /////////////Stack manipulation/////////////////
            ////////////////////////////////////////////////
            Token::DigN => "DIG",
            Token::DugN => "DUG",
            Token::Dip => "DIP",
            Token::DipN => "DIP",
            Token::Dup => "DUP",
            Token::DupN => "DUP",
            Token::Push => "PUSH",
            Token::Drop => "DROP",
            Token::Swap => "SWAP",
            ////////////////////////////////////////////////
            /////////////      Macro       /////////////////
            ////////////////////////////////////////////////
            Token::Assert => "ASSERT",
            Token::AssertSome => "ASSERT_SOME",
        }
        .to_string()
    }
}
