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
    pub fn to_formatted_string(&self, accumulation: usize) -> String {
        let indent = " ".repeat(accumulation);
        match self {
            InstructionWrapper::Comment(cmt) => format!("{indent}# {}", cmt),
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
                        let space_label = " ".repeat(instruction.get_label_len());
                        let formatted_instr1 =
                            format(instr1, accumulation + instruction.get_label_len() + 3);
                        let formatted_instr2 =
                            format(instr2, accumulation + instruction.get_label_len() + 3);
                        format!(
                            r#"{indent}{label} {{
{formatted_instr1}
{indent}{space_label} }}
{indent}{space_label} {{
{formatted_instr2}
{indent}{space_label} }}"#
                        )
                    }
                    Instruction::IfCons { .. } => todo!(),
                    Instruction::IfLeft { .. } => todo!(),
                    Instruction::IfNone { instr1, instr2 } => {
                        let label = instruction.get_label();
                        let space_label = " ".repeat(instruction.get_label_len());
                        let formatted_instr1 =
                            format(instr1, accumulation + instruction.get_label_len() + 3);
                        let formatted_instr2 =
                            format(instr2, accumulation + instruction.get_label_len() + 3);
                        format!(
                            r#"{indent}{label} {{
{formatted_instr1}
{indent}{space_label} }}
{indent}{space_label} {{
{formatted_instr2}
{indent}{space_label} }}"#
                        )
                    }
                    //ITER inster,
                    //LAMBDA ty1 ty2 instr,
                    Instruction::Loop { instr } => {
                        let label = instruction.get_label();
                        let space_label = " ".repeat(instruction.get_label_len());
                        let formatted_instr =
                            format(instr, accumulation + instruction.get_label_len() + 3);
                        format!(
                            r#"{indent}{label} {{
{formatted_instr}
{indent}{space_label} }}"#
                        )
                    }
                    Instruction::LoopLeft { .. } => todo!(),
                    //instr1 ; instr2,
                    //{},
                    ////////////////////////////////////////////////
                    //////////Operations on data structures/////////
                    ////////////////////////////////////////////////
                    Instruction::EmptyBigMap { kty, vty } => format!(
                        "{indent}{} {} {}",
                        instruction.get_label(),
                        kty.to_string(),
                        vty.to_string()
                    ),
                    Instruction::EmptyMap { kty, vty } => {
                        format!(
                            "{indent}{} {} {}",
                            instruction.get_label(),
                            kty.to_string(),
                            vty.to_string()
                        )
                    }
                    Instruction::None { ty } => {
                        format!("{indent}{} {}", instruction.get_label(), ty.to_string())
                    }
                    Instruction::GetN(n) => format!("{indent}{} {}", instruction.get_label(), n),
                    Instruction::Nil { ty } => {
                        format!("{indent}{} {}", instruction.get_label(), ty.to_string())
                    }
                    ////////////////////////////////////////////////
                    /////////////Blockchain operations//////////////
                    ////////////////////////////////////////////////
                    //CREATE_CONTRACT { parameter ty1; storage ty2; code instr1 },
                    Instruction::Contract { ty } => {
                        format!("{indent}{} {}", instruction.get_label(), ty.to_string())
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
                            "{indent}{} {} {}",
                            instruction.get_label(),
                            ty.to_string(),
                            val.to_string()
                        )
                    }
                    Instruction::DupN(n) => format!("{indent}{} {}", instruction.get_label(), n),
                    Instruction::DigN(n) => format!("{indent}{} {}", instruction.get_label(), n),
                    Instruction::DugN(n) => format!("{indent}{} {}", instruction.get_label(), n),
                    Instruction::PairN(n) => format!("{indent}{} {}", instruction.get_label(), n),
                    _ => format!("{indent}{}", instruction.get_label()),
                };
                match comment {
                    Some(s) => format!("{formatted_string}; # {s}"),
                    None => format!("{formatted_string};"),
                }
            }
        }
    }
}
