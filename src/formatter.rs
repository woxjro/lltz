use crate::instruction_wrapper::InstructionWrapper;

pub fn format(instructions: &Vec<InstructionWrapper>, accumulation: usize) -> String {
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
