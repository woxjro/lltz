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
    pub fn get_label(&self) -> String {
        match self {
            Instruction::Comment(comment) => comment.to_owned(),
            ////////////////////////////////////////////////
            ////////////////Control Structures//////////////
            ////////////////////////////////////////////////
            Instruction::Apply => Token::Apply.to_string(),
            Instruction::Exec => Token::Exec.to_string(),
            Instruction::Failwith => Token::Failwith.to_string(),
            Instruction::If { .. } => Token::If.to_string(),
            Instruction::IfCons { .. } => Token::IfCons.to_string(),
            Instruction::IfLeft { .. } => Token::IfLeft.to_string(),
            Instruction::IfNone { .. } => Token::IfNone.to_string(),
            Instruction::Loop { .. } => Token::Loop.to_string(),
            Instruction::LoopLeft { .. } => Token::LoopLeft.to_string(),
            //ITER inster,
            //LAMBDA ty1 ty2 instr,
            //instr1 ; instr2,
            //{},
            ////////////////////////////////////////////////
            //////////Operations on data structures/////////
            ////////////////////////////////////////////////
            Instruction::Car => Token::Car.to_string(),
            Instruction::Cdr => Token::Cdr.to_string(),
            Instruction::Concat => Token::Concat.to_string(),
            Instruction::Cons => Token::Cons.to_string(),
            Instruction::EmptyBigMap { .. } => Token::EmptyBigMap.to_string(),
            Instruction::EmptyMap { .. } => Token::EmptyMap.to_string(),
            Instruction::EmptySet => Token::EmptySet.to_string(),
            Instruction::Get => Token::Get.to_string(),
            Instruction::GetN(_) => Token::GetN.to_string(),
            Instruction::GetAndUpdate => Token::GetAndUpdate.to_string(),
            //LEFT ty2,
            //MAP instr,
            Instruction::Mem => Token::Mem.to_string(),
            Instruction::Never => Token::Never.to_string(),
            Instruction::Nil { .. } => Token::Nil.to_string(),
            Instruction::None { .. } => Token::None.to_string(),
            Instruction::Pack => Token::Pack.to_string(),
            Instruction::Pair => Token::Pair.to_string(),
            Instruction::PairN(_) => Token::PairN.to_string(),
            //RIGHT ty1,
            Instruction::Size => Token::Size.to_string(),
            Instruction::Slice => Token::Slice.to_string(),
            Instruction::Some => Token::Some.to_string(),
            Instruction::Unit => Token::Unit.to_string(),
            //UNPACK ty,
            Instruction::Unpair => Token::Unpair.to_string(),
            //UNPAIR n,
            Instruction::Update => Token::Update.to_string(),
            //UPDATE n,
            ////////////////////////////////////////////////
            /////////////Blockchain operations//////////////
            ////////////////////////////////////////////////
            Instruction::Address => Token::Address.to_string(),
            Instruction::Amount => Token::Amount.to_string(),
            Instruction::Balance => Token::Balance.to_string(),
            Instruction::ChainId => Token::ChainId.to_string(),
            Instruction::Contract { .. } => Token::Contract.to_string(),
            //CREATE_CONTRACT { parameter ty1; storage ty2; code instr1 },
            Instruction::ImplicitAccount => Token::ImplicitAccount.to_string(),
            Instruction::Level => Token::Level.to_string(),
            Instruction::Now => Token::Now.to_string(),
            //Instruction::Self => //Token::,
            Instruction::SelfAddress => Token::SelfAddress.to_string(),
            Instruction::Sender => Token::Sender.to_string(),
            Instruction::SetDelegate => Token::SetDelegate.to_string(),
            Instruction::Source => Token::Source.to_string(),
            Instruction::TotalVotingPower => Token::TotalVotingPower.to_string(),
            Instruction::TransferTokens => Token::TransferTokens.to_string(),
            Instruction::VotingPower => Token::VotingPower.to_string(),
            ////////////////////////////////////////////////
            ////////////Operations on tickets///////////////
            ////////////////////////////////////////////////
            Instruction::JointTickets => Token::JointTickets.to_string(),
            Instruction::ReadTicket => Token::ReadTicket.to_string(),
            Instruction::SplitTicket => Token::SplitTicket.to_string(),
            Instruction::Ticket => Token::Ticket.to_string(),
            ////////////////////////////////////////////////
            ////////////Cryptographic operations////////////
            ////////////////////////////////////////////////
            Instruction::Blake2b => Token::Blake2b.to_string(),
            Instruction::CheckSignature => Token::CheckSignature.to_string(),
            Instruction::HashKey => Token::HashKey.to_string(),
            Instruction::Keccak => Token::Keccak.to_string(),
            Instruction::PairingCheck => Token::PairingCheck.to_string(),
            Instruction::SaplingEmptyState => Token::SaplingEmptyState.to_string(),
            Instruction::SaplingVerifyUpdate => Token::SaplingVerifyUpdate.to_string(),
            Instruction::Sha256 => Token::Sha256.to_string(),
            Instruction::Sha3 => Token::Sha3.to_string(),
            Instruction::Sha512 => Token::Sha512.to_string(),
            ////////////////////////////////////////////////
            //////////////Boolean operations////////////////
            ////////////////////////////////////////////////
            Instruction::And => Token::And.to_string(),
            Instruction::Not => Token::Not.to_string(),
            Instruction::Or => Token::Or.to_string(),
            Instruction::Xor => Token::Xor.to_string(),
            ////////////////////////////////////////////////
            ////////////Arithmetic operations///////////////
            ////////////////////////////////////////////////
            Instruction::Abs => Token::Abs.to_string(),
            Instruction::Add => Token::Add.to_string(),
            Instruction::Compare => Token::Compare.to_string(),
            Instruction::Ediv => Token::Ediv.to_string(),
            Instruction::Eq => Token::Eq.to_string(),
            Instruction::Ge => Token::Ge.to_string(),
            Instruction::Gt => Token::Gt.to_string(),
            Instruction::Int => Token::Int.to_string(),
            Instruction::Isnat => Token::Isnat.to_string(),
            Instruction::Le => Token::Le.to_string(),
            Instruction::Lsl => Token::Lsl.to_string(),
            Instruction::Lsr => Token::Lsr.to_string(),
            Instruction::Lt => Token::Lt.to_string(),
            Instruction::Mul => Token::Mul.to_string(),
            Instruction::Neg => Token::Neg.to_string(),
            Instruction::Neq => Token::Neq.to_string(),
            Instruction::Sub => Token::Sub.to_string(),
            ////////////////////////////////////////////////
            /////////////Stack manipulation/////////////////
            ////////////////////////////////////////////////
            Instruction::DigN(_) => Token::DigN.to_string(),
            Instruction::DugN(_) => Token::DugN.to_string(),
            Instruction::Dip => Token::Dip.to_string(),
            Instruction::DipN(_) => Token::DipN.to_string(),
            Instruction::Dup => Token::Dup.to_string(),
            Instruction::DupN(_) => Token::DupN.to_string(),
            Instruction::Push { .. } => Token::Push.to_string(),
            Instruction::Drop => Token::Drop.to_string(),
            Instruction::Swap => Token::Swap.to_string(),
            ////////////////////////////////////////////////
            /////////////      Macro       /////////////////
            ////////////////////////////////////////////////
            Instruction::AssertSome => Token::AssertSome.to_string(),
            e => todo!("{:?} is not implemented", e),
        }
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
