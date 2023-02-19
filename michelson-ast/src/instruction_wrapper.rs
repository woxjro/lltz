use crate::formatter::format;
use crate::instruction::Instruction;
#[derive(Clone, Debug)]
pub enum InstructionWrapper {
    Comment(String),
    Instruction {
        instruction: Instruction,
        comment: Option<String>,
    },
}

impl InstructionWrapper {
    pub fn to_formatted_string(&self, depth: usize, tab: &str) -> String {
        let space = tab.repeat(depth);
        match self {
            InstructionWrapper::Comment(cmt) => format!("{space}# {}", cmt),
            InstructionWrapper::Instruction {
                instruction,
                comment,
            } => {
                let formatted_string = match instruction {
                    ////////////////////////////////////////////////
                    ////////////////Control Structures//////////////
                    ////////////////////////////////////////////////
                    Instruction::If { instr1, instr2 } => {
                        let label = instruction.get_label();
                        let space = tab.repeat(depth);
                        let space_label = " ".repeat(instruction.get_label_len());
                        let formatted_instr1 = format(instr1, depth + 1, tab);
                        let formatted_instr2 = format(instr2, depth + 1, tab);
                        format!(
                            r#"{space}{label} {{
{formatted_instr1}
{space}{space_label} }}
{space}{space_label} {{
{formatted_instr2}
{space}{space_label} }}"#
                        )
                    }
                    Instruction::IfCons { .. } => todo!(),
                    Instruction::IfLeft { .. } => todo!(),
                    Instruction::IfNone { instr1, instr2 } => {
                        let label = instruction.get_label();
                        let space = tab.repeat(depth);
                        let space_label = " ".repeat(instruction.get_label_len());
                        let formatted_instr1 = format(instr1, depth + 1, "          ");
                        let formatted_instr2 = format(instr2, depth + 1, "          ");
                        format!(
                            r#"{space}{label} {{
{formatted_instr1}
{space}{space_label} }}
{space}{space_label} {{
{formatted_instr2}
{space}{space_label} }}"#
                        )
                    }
                    //ITER inster,
                    //LAMBDA ty1 ty2 instr,
                    Instruction::Loop { instr } => {
                        let label = instruction.get_label();
                        let space = tab.repeat(depth);
                        let space_label = " ".repeat(instruction.get_label_len());
                        let formatted_instr = format(instr, depth + 1, tab);
                        format!(
                            r#"{space}{label} {{
{formatted_instr}
{space}{space_label} }}"#
                        )
                    }
                    Instruction::LoopLeft { .. } => todo!(),
                    //instr1 ; instr2,
                    //{},
                    ////////////////////////////////////////////////
                    //////////Operations on data structures/////////
                    ////////////////////////////////////////////////
                    Instruction::EmptyBigMap { kty, vty } => format!(
                        "{space}{} {} {}",
                        instruction.get_label(),
                        kty.to_string(),
                        vty.to_string()
                    ),
                    Instruction::EmptyMap { kty, vty } => {
                        format!(
                            "{space}{} {} {}",
                            instruction.get_label(),
                            kty.to_string(),
                            vty.to_string()
                        )
                    }
                    Instruction::None { ty } => {
                        format!("{space}{} {}", instruction.get_label(), ty.to_string())
                    }
                    Instruction::GetN(n) => format!("{space}{} {}", instruction.get_label(), n),
                    Instruction::Nil { ty } => {
                        format!("{space}{} {}", instruction.get_label(), ty.to_string())
                    }
                    ////////////////////////////////////////////////
                    /////////////Blockchain operations//////////////
                    ////////////////////////////////////////////////
                    //CREATE_CONTRACT { parameter ty1; storage ty2; code instr1 },
                    Instruction::Contract { ty } => {
                        format!("{space}{} {}", instruction.get_label(), ty.to_string())
                    }
                    ////////////////////////////////////////////////
                    ////////////Operations on tickets///////////////
                    ////////////////////////////////////////////////
                    ////////////////////////////////////////////////
                    ////////////Cryptographic operations////////////
                    ////////////////////////////////////////////////
                    ////////////////////////////////////////////////
                    //////////////Boolean operations////////////////
                    ////////////////////////////////////////////////
                    ////////////////////////////////////////////////
                    ////////////Arithmetic operations///////////////
                    ////////////////////////////////////////////////
                    ////////////////////////////////////////////////
                    /////////////Stack manipulation/////////////////
                    ////////////////////////////////////////////////
                    Instruction::Push { ty, val } => {
                        format!(
                            "{space}{} {} {}",
                            instruction.get_label(),
                            ty.to_string(),
                            val.to_string()
                        )
                    }
                    Instruction::DupN(n) => format!("{space}{} {}", instruction.get_label(), n),
                    Instruction::DigN(n) => format!("{space}{} {}", instruction.get_label(), n),
                    Instruction::DugN(n) => format!("{space}{} {}", instruction.get_label(), n),
                    Instruction::PairN(n) => format!("{space}{} {}", instruction.get_label(), n),
                    _ => format!("{space}{}", instruction.get_label()),
                };
                match comment {
                    Some(s) => format!("{formatted_string}; # {s}"),
                    None => format!("{formatted_string};"),
                }
            }
        }
    }
}
