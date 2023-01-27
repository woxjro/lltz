use crate::instruction::Instruction;
use crate::instruction_wrapper::InstructionWrapper;

pub fn to_instruction_wrapper(instruction: Instruction) -> InstructionWrapper {
    InstructionWrapper::Instruction {
        comment: None,
        instruction,
    }
}

pub fn to_instruction_wrapper_with_comment(
    instruction: Instruction,
    comment: &str,
) -> InstructionWrapper {
    InstructionWrapper::Instruction {
        comment: Some(comment.to_string()),
        instruction,
    }
}
