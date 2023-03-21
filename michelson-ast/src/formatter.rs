use crate::instruction_with_comment::InstructionWithComment;

pub fn format(instructions: &Vec<InstructionWithComment>, accumulation: usize) -> String {
    let mut res = String::from("");
    for instruction in instructions {
        res = format!(
            r#"{res}
{}"#,
            instruction.to_formatted_string(accumulation)
        );
    }
    res.trim_matches('\n').to_string()
}
