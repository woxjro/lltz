use super::instruction::Instruction;

pub fn format(instructions: &Vec<Instruction>, depth: usize, tab: &str) -> String {
    let mut res = String::from("");
    for instruction in instructions {
        res = format!(
            r#"{res}
{};"#,
            instruction.to_formatted_string(depth, tab)
        );
    }
    res.trim_matches('\n').to_string()
}

#[cfg(test)]
mod tests {
    use super::super::instruction::Instruction;
    use super::format;
    #[test]
    fn it_works() {
        let instructions = vec![
            Instruction::Add,
            Instruction::Sub,
            Instruction::Mul,
            Instruction::If {
                instr1: vec![Instruction::Add, Instruction::Sub, Instruction::Mul],
                instr2: vec![Instruction::If {
                    instr1: vec![Instruction::Add, Instruction::Sub, Instruction::Mul],
                    instr2: vec![Instruction::Add, Instruction::Sub, Instruction::Mul],
                }],
            },
            Instruction::Add,
            Instruction::Sub,
            Instruction::Mul,
        ];
        let result = super::format(&instructions, 0, "    ");
        println!("{}", result);
        assert_eq!(result, String::from(""));
    }
}
