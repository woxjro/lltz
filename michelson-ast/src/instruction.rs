mod token;
use crate::ty::Ty;
use crate::val::Val;
use crate::wrapped_instruction::WrappedInstruction;
use token::Token;
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
        ty: Ty,
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
    fn get_token(&self) -> Token {
        match self {
            Instruction::Comment(_) => Token::Comment,
            ////////////////////////////////////////////////
            ////////////////Control Structures//////////////
            ////////////////////////////////////////////////
            Instruction::Apply => Token::Apply,
            Instruction::Exec => Token::Exec,
            Instruction::Failwith => Token::Failwith,
            Instruction::If { .. } => Token::If,
            Instruction::IfCons { .. } => Token::IfCons,
            Instruction::IfLeft { .. } => Token::IfLeft,
            Instruction::IfNone { .. } => Token::IfNone,
            Instruction::Loop { .. } => Token::Loop,
            Instruction::LoopLeft { .. } => Token::LoopLeft,
            //ITER inster,
            //LAMBDA ty1 ty2 instr,
            //instr1 ; instr2,
            //{},
            ////////////////////////////////////////////////
            //////////Operations on data structures/////////
            ////////////////////////////////////////////////
            Instruction::Car => Token::Car,
            Instruction::Cdr => Token::Cdr,
            Instruction::Concat => Token::Concat,
            Instruction::Cons => Token::Cons,
            Instruction::EmptyBigMap { .. } => Token::EmptyBigMap,
            Instruction::EmptyMap { .. } => Token::EmptyMap,
            Instruction::EmptySet => Token::EmptySet,
            Instruction::Get => Token::Get,
            Instruction::GetN(_) => Token::GetN,
            Instruction::GetAndUpdate => Token::GetAndUpdate,
            //LEFT ty2,
            //MAP instr,
            Instruction::Mem => Token::Mem,
            Instruction::Never => Token::Never,
            Instruction::Nil { .. } => Token::Nil,
            Instruction::None { .. } => Token::None,
            Instruction::Pack => Token::Pack,
            Instruction::Pair => Token::Pair,
            Instruction::PairN(_) => Token::PairN,
            //RIGHT ty1,
            Instruction::Size => Token::Size,
            Instruction::Slice => Token::Slice,
            Instruction::Some => Token::Some,
            Instruction::Unit => Token::Unit,
            //UNPACK ty,
            Instruction::Unpair => Token::Unpair,
            //UNPAIR n,
            Instruction::Update => Token::Update,
            //UPDATE n,
            ////////////////////////////////////////////////
            /////////////Blockchain operations//////////////
            ////////////////////////////////////////////////
            Instruction::Address => Token::Address,
            Instruction::Amount => Token::Amount,
            Instruction::Balance => Token::Balance,
            Instruction::ChainId => Token::ChainId,
            Instruction::Contract { .. } => Token::Contract,
            //CREATE_CONTRACT { parameter ty1; storage ty2; code instr1 },
            Instruction::ImplicitAccount => Token::ImplicitAccount,
            Instruction::Level => Token::Level,
            Instruction::Now => Token::Now,
            Instruction::Slf => Token::Slf,
            Instruction::SelfAddress => Token::SelfAddress,
            Instruction::Sender => Token::Sender,
            Instruction::SetDelegate => Token::SetDelegate,
            Instruction::Source => Token::Source,
            Instruction::TotalVotingPower => Token::TotalVotingPower,
            Instruction::TransferTokens => Token::TransferTokens,
            Instruction::VotingPower => Token::VotingPower,
            ////////////////////////////////////////////////
            ////////////Operations on tickets///////////////
            ////////////////////////////////////////////////
            Instruction::JointTickets => Token::JointTickets,
            Instruction::ReadTicket => Token::ReadTicket,
            Instruction::SplitTicket => Token::SplitTicket,
            Instruction::Ticket => Token::Ticket,
            ////////////////////////////////////////////////
            ////////////Cryptographic operations////////////
            ////////////////////////////////////////////////
            Instruction::Blake2b => Token::Blake2b,
            Instruction::CheckSignature => Token::CheckSignature,
            Instruction::HashKey => Token::HashKey,
            Instruction::Keccak => Token::Keccak,
            Instruction::PairingCheck => Token::PairingCheck,
            Instruction::SaplingEmptyState => Token::SaplingEmptyState,
            Instruction::SaplingVerifyUpdate => Token::SaplingVerifyUpdate,
            Instruction::Sha256 => Token::Sha256,
            Instruction::Sha3 => Token::Sha3,
            Instruction::Sha512 => Token::Sha512,
            ////////////////////////////////////////////////
            //////////////Boolean operations////////////////
            ////////////////////////////////////////////////
            Instruction::And => Token::And,
            Instruction::Not => Token::Not,
            Instruction::Or => Token::Or,
            Instruction::Xor => Token::Xor,
            ////////////////////////////////////////////////
            ////////////Arithmetic operations///////////////
            ////////////////////////////////////////////////
            Instruction::Abs => Token::Abs,
            Instruction::Add => Token::Add,
            Instruction::Compare => Token::Compare,
            Instruction::Ediv => Token::Ediv,
            Instruction::Eq => Token::Eq,
            Instruction::Ge => Token::Ge,
            Instruction::Gt => Token::Gt,
            Instruction::Int => Token::Int,
            Instruction::Isnat => Token::Isnat,
            Instruction::Le => Token::Le,
            Instruction::Lsl => Token::Lsl,
            Instruction::Lsr => Token::Lsr,
            Instruction::Lt => Token::Lt,
            Instruction::Mul => Token::Mul,
            Instruction::Neg => Token::Neg,
            Instruction::Neq => Token::Neq,
            Instruction::Sub => Token::Sub,
            ////////////////////////////////////////////////
            /////////////Stack manipulation/////////////////
            ////////////////////////////////////////////////
            Instruction::DigN(_) => Token::DigN,
            Instruction::DugN(_) => Token::DugN,
            Instruction::Dip => Token::Dip,
            Instruction::DipN(_) => Token::DipN,
            Instruction::Dup => Token::Dup,
            Instruction::DupN(_) => Token::DupN,
            Instruction::Push { .. } => Token::Push,
            Instruction::Drop => Token::Drop,
            Instruction::Swap => Token::Swap,
            ////////////////////////////////////////////////
            /////////////      Macro       /////////////////
            ////////////////////////////////////////////////
            Instruction::AssertSome => Token::AssertSome,
        }
    }

    pub fn get_label(&self) -> String {
        self.get_token().to_string()
    }

    pub fn get_label_len(&self) -> usize {
        self.get_label().len()
    }

    pub fn to_wrapped_instruction(&self) -> WrappedInstruction {
        WrappedInstruction::from(self.clone())
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
