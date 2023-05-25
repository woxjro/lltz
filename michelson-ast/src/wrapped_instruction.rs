use crate::formatter::format;
use crate::instruction::Instruction;

#[macro_export]
macro_rules! instruction_row {
    ($instruction:expr) => {{
        michelson_ast::wrapped_instruction::WrappedInstruction {
            instruction: $instruction,
            comment: None,
        }
    }};

    ($instruction:expr, $comment:expr) => {{
        michelson_ast::wrapped_instruction::WrappedInstruction {
            instruction: $instruction,
            comment: Some($comment),
        }
    }};
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct WrappedInstruction {
    pub instruction: Instruction,
    pub comment: Option<String>,
}

impl From<Instruction> for WrappedInstruction {
    fn from(instruction: Instruction) -> Self {
        Self {
            comment: None,
            instruction,
        }
    }
}

impl WrappedInstruction {
    pub fn to_formatted_string(&self, accumulation: usize) -> String {
        let indent = " ".repeat(accumulation);
        let instruction = &self.instruction;
        let formatted_string = match instruction {
            Instruction::Comment(comment) => {
                format!("{indent}{label} {comment}", label = instruction.get_label(),)
            }
            ////////////////////////////////////////////////
            ////////////////Control Structures//////////////
            ////////////////////////////////////////////////
            Instruction::If { instr1, instr2 } => {
                format!(
                    r#"{indent}{label} {{
{formatted_instr1}
{indent}{space_label} }}
{indent}{space_label} {{
{formatted_instr2}
{indent}{space_label} }}"#,
                    label = instruction.get_label(),
                    space_label = " ".repeat(instruction.get_label_len()),
                    formatted_instr1 =
                        format(instr1, accumulation + instruction.get_label_len() + 3),
                    formatted_instr2 =
                        format(instr2, accumulation + instruction.get_label_len() + 3)
                )
            }
            Instruction::IfCons { instr1, instr2 } => {
                format!(
                    r#"{indent}{label} {{
{formatted_instr1}
{indent}{space_label} }}
{indent}{space_label} {{
{formatted_instr2}
{indent}{space_label} }}"#,
                    label = instruction.get_label(),
                    space_label = " ".repeat(instruction.get_label_len()),
                    formatted_instr1 =
                        format(instr1, accumulation + instruction.get_label_len() + 3),
                    formatted_instr2 =
                        format(instr2, accumulation + instruction.get_label_len() + 3)
                )
            }
            Instruction::IfLeft { instr1, instr2 } => {
                format!(
                    r#"{indent}{label} {{
{formatted_instr1}
{indent}{space_label} }}
{indent}{space_label} {{
{formatted_instr2}
{indent}{space_label} }}"#,
                    label = instruction.get_label(),
                    space_label = " ".repeat(instruction.get_label_len()),
                    formatted_instr1 =
                        format(instr1, accumulation + instruction.get_label_len() + 3),
                    formatted_instr2 =
                        format(instr2, accumulation + instruction.get_label_len() + 3)
                )
            }
            Instruction::IfNone { instr1, instr2 } => {
                format!(
                    r#"{indent}{label} {{
{formatted_instr1}
{indent}{space_label} }}
{indent}{space_label} {{
{formatted_instr2}
{indent}{space_label} }}"#,
                    label = instruction.get_label(),
                    space_label = " ".repeat(instruction.get_label_len()),
                    formatted_instr1 =
                        format(instr1, accumulation + instruction.get_label_len() + 3),
                    formatted_instr2 =
                        format(instr2, accumulation + instruction.get_label_len() + 3)
                )
            }
            Instruction::Iter { .. } => {
                todo!()
            }
            Instruction::Lambda { .. } => {
                todo!()
            }
            Instruction::Loop { instr } => {
                format!(
                    r#"{indent}{label} {{
{formatted_instr}
{indent}{space_label} }}"#,
                    label = instruction.get_label(),
                    space_label = " ".repeat(instruction.get_label_len()),
                    formatted_instr = format(instr, accumulation + instruction.get_label_len() + 3)
                )
            }
            Instruction::LoopLeft { instr } => {
                format!(
                    r#"{indent}{label} {{
{formatted_instr}
{indent}{space_label} }}"#,
                    label = instruction.get_label(),
                    space_label = " ".repeat(instruction.get_label_len()),
                    formatted_instr = format(instr, accumulation + instruction.get_label_len() + 3)
                )
            }
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
            Instruction::Left { .. } => todo!(),
            Instruction::Map { .. } => todo!(),
            Instruction::Nil { ty } => {
                format!("{indent}{} {}", instruction.get_label(), ty.to_string())
            }
            Instruction::Right { .. } => todo!(),
            Instruction::Unpack { .. } => todo!(),
            Instruction::UnpairN { .. } => todo!(),
            Instruction::UpdateN { .. } => todo!(),
            ////////////////////////////////////////////////
            /////////////Blockchain operations//////////////
            ////////////////////////////////////////////////
            Instruction::CreateContract { .. } => todo!(),
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
            Instruction::DupN(n) => format!("{indent}{} {n}", instruction.get_label()),
            Instruction::DigN(n) => format!("{indent}{} {n}", instruction.get_label()),
            Instruction::DugN(n) => format!("{indent}{} {n}", instruction.get_label()),
            Instruction::PairN(n) => format!("{indent}{} {n}", instruction.get_label()),
            _ => format!("{indent}{}", instruction.get_label()),
        };
        match &self.comment {
            Some(s) => format!("{formatted_string}; # {s}"),
            None => match instruction {
                Instruction::Comment(_) => format!("{formatted_string}"),
                _ => format!("{formatted_string};"),
            },
        }
    }
    pub fn count(&self) -> usize {
        self.instruction.count()
    }
}
