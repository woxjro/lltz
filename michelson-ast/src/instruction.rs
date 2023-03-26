use super::ty::Ty;
use super::val::Val;
use crate::wrapped_instruction::WrappedInstruction;
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum Instruction {
    Comment(String),
    ////////////////////////////////////////////////
    ////////////////Control Structures//////////////
    ////////////////////////////////////////////////
    Apply,
    Exec,
    Failwith,
    If {
        instr1: Vec<WrappedInstruction>,
        instr2: Vec<WrappedInstruction>,
    },
    IfCons {
        instr1: Vec<WrappedInstruction>,
        instr2: Vec<WrappedInstruction>,
    },
    IfLeft {
        instr1: Vec<WrappedInstruction>,
        instr2: Vec<WrappedInstruction>,
    },
    IfNone {
        instr1: Vec<WrappedInstruction>,
        instr2: Vec<WrappedInstruction>,
    },
    //ITER inster,
    //LAMBDA ty1 ty2 instr,
    //LOOP instr,
    Loop {
        instr: Vec<WrappedInstruction>,
    },
    LoopLeft {
        instr: Vec<WrappedInstruction>,
    },
    //instr1 ; instr2,
    //{},
    ////////////////////////////////////////////////
    //////////Operations on data structures/////////
    ////////////////////////////////////////////////
    Car,
    Cdr,
    Concat,
    Cons,
    EmptyBigMap {
        kty: Ty,
        vty: Ty,
    },
    EmptyMap {
        kty: Ty,
        vty: Ty,
    },
    EmptySet,
    Get,
    GetN(usize),
    GetAndUpdate,
    //LEFT ty2,
    //MAP instr,
    Mem,
    Never,
    Nil {
        ty: Ty,
    },
    None {
        ty: Box<Ty>,
    },
    Pack,
    Pair,
    PairN(usize),
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
    Contract {
        ty: Ty,
    },
    //CREATE_CONTRACT { parameter ty1; storage ty2; code instr1 },
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
    DigN(usize),
    DugN(usize),
    Dip,
    DipN(usize),
    Dup,
    DupN(usize),
    Push {
        ty: Ty,
        val: Val,
    },
    Drop,
    Swap,
    ////////////////////////////////////////////////
    /////////////      Macro       /////////////////
    ////////////////////////////////////////////////
    AssertSome,
}

impl Instruction {
    pub fn get_label(&self) -> String {
        match self {
            Instruction::Comment(comment) => comment.clone(),
            ////////////////////////////////////////////////
            ////////////////Control Structures//////////////
            ////////////////////////////////////////////////
            Instruction::Apply => "APPLY".to_string(),
            Instruction::Exec => "EXEC".to_string(),
            Instruction::Failwith => "FAILWITH".to_string(),
            Instruction::If { .. } => "IF".to_string(),
            Instruction::IfCons { .. } => "IF_CONS".to_string(),
            Instruction::IfLeft { .. } => "IF_LEFT".to_string(),
            Instruction::IfNone { .. } => "IF_NONE".to_string(),
            Instruction::Loop { .. } => "LOOP".to_string(),
            Instruction::LoopLeft { .. } => "LOOP_LEFT".to_string(),
            //ITER inster,
            //LAMBDA ty1 ty2 instr,
            //instr1 ; instr2,
            //{},
            ////////////////////////////////////////////////
            //////////Operations on data structures/////////
            ////////////////////////////////////////////////
            Instruction::Car => "CAR".to_string(),
            Instruction::Cdr => "CDR".to_string(),
            Instruction::Concat => "CONCAT".to_string(),
            Instruction::Cons => "CONS".to_string(),
            Instruction::EmptyBigMap { .. } => "EMPTY_BIG_MAP".to_string(),
            Instruction::EmptyMap { .. } => "EMPTY_MAP".to_string(),
            Instruction::EmptySet => "EMPTY_SET".to_string(),
            Instruction::Get => "GET".to_string(),
            Instruction::GetN(_) => "GET".to_string(),
            Instruction::GetAndUpdate => "GET_AND_UPDATE".to_string(),
            //LEFT ty2,
            //MAP instr,
            Instruction::Mem => "MEM".to_string(),
            Instruction::Never => "NEVER".to_string(),
            Instruction::Nil { .. } => "NIL".to_string(),
            Instruction::None { .. } => "NONE".to_string(),
            Instruction::Pack => "PACK".to_string(),
            Instruction::Pair => "PAIR".to_string(),
            Instruction::PairN(_) => "PAIR".to_string(),
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
            Instruction::Contract { .. } => "CONTRACT".to_string(),
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
            ////////////////////////////////////////////////
            ////////////Operations on tickets///////////////
            ////////////////////////////////////////////////
            Instruction::JointTickets => "JOINT_TICKETS".to_string(),
            Instruction::ReadTicket => "READ_TICKET".to_string(),
            Instruction::SplitTicket => "SPLIT_TICKET".to_string(),
            Instruction::Ticket => "TICKET".to_string(),
            ////////////////////////////////////////////////
            ////////////Cryptographic operations////////////
            ////////////////////////////////////////////////
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
            ////////////////////////////////////////////////
            //////////////Boolean operations////////////////
            ////////////////////////////////////////////////
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
            Instruction::DigN(_) => "DIG".to_string(),
            Instruction::DugN(_) => "DUG".to_string(),
            Instruction::Dip => "DIP".to_string(),
            Instruction::DipN(_) => "DIP".to_string(),
            Instruction::Dup => "DUP".to_string(),
            Instruction::DupN(_) => "DUP".to_string(),
            Instruction::Push { .. } => "PUSH".to_string(),
            Instruction::Drop => "DROP".to_string(),
            Instruction::Swap => "SWAP".to_string(),
            ////////////////////////////////////////////////
            /////////////      Macro       /////////////////
            ////////////////////////////////////////////////
            Instruction::AssertSome => "ASSERT_SOME".to_string(),
            e => todo!("{:?} is not implemented", e),
        }
    }

    pub fn get_label_len(&self) -> usize {
        self.get_label().len()
    }

    pub fn to_wrapped_instruction(&self) -> WrappedInstruction {
        WrappedInstruction {
            comment: None,
            instruction: self.clone(),
        }
    }

    /// 命令数を返す関数
    /// コメントは0， PUSH, DIG, DUG などは1，IFなど内部に命令列を持つ命令は再帰的に計算
    pub fn count(&self) -> usize {
        match self {
            Instruction::Comment(_) => 0,
            ////////////////////////////////////////////////
            ////////////////Control Structures//////////////
            ////////////////////////////////////////////////
            Instruction::If { instr1, instr2 } => {
                instr1.iter().map(|instr| instr.count()).sum::<usize>()
                    + instr2.iter().map(|instr| instr.count()).sum::<usize>()
                    + 1
            }
            Instruction::IfCons { instr1, instr2 } => {
                instr1.iter().map(|instr| instr.count()).sum::<usize>()
                    + instr2.iter().map(|instr| instr.count()).sum::<usize>()
                    + 1
            }
            Instruction::IfLeft { instr1, instr2 } => {
                instr1.iter().map(|instr| instr.count()).sum::<usize>()
                    + instr2.iter().map(|instr| instr.count()).sum::<usize>()
                    + 1
            }
            Instruction::IfNone { instr1, instr2 } => {
                instr1.iter().map(|instr| instr.count()).sum::<usize>()
                    + instr2.iter().map(|instr| instr.count()).sum::<usize>()
                    + 1
            }
            Instruction::Loop { instr } => {
                instr.iter().map(|instr| instr.count()).sum::<usize>() + 1
            }
            Instruction::LoopLeft { instr } => {
                instr.iter().map(|instr| instr.count()).sum::<usize>() + 1
            }
            //ITER inster,
            //LAMBDA ty1 ty2 instr,
            //instr1 ; instr2,
            //{},
            ////////////////////////////////////////////////
            //////////Operations on data structures/////////
            ////////////////////////////////////////////////
            _ => 1,
        }
    }
}
