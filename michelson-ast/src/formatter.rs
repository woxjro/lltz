use crate::instruction_wrapper::InstructionWrapper;

pub fn format(instructions: &Vec<InstructionWrapper>, depth: usize, tab: &str) -> String {
    let mut res = String::from("");
    for instruction in instructions {
        res = format!(
            r#"{res}
{}"#,
            instruction.to_formatted_string(depth, tab)
        );
    }
    res.trim_matches('\n').to_string()
}

#[cfg(test)]
mod tests {
    use super::format;
    use crate::instruction::Instruction;
    use crate::instruction_wrapper::InstructionWrapper;
    use crate::ty::Ty;
    use crate::val::Val;
    #[test]
    fn it_works() {
        let instructions = vec![
            InstructionWrapper::Instruction {
                comment: Some("This is a comment".to_string()),
                instruction: Instruction::Push {
                    ty: Ty::Mutez,
                    val: Val::Mutez(999),
                },
            },
            InstructionWrapper::Instruction {
                comment: Some("This is a comment".to_string()),
                instruction: Instruction::Push {
                    ty: Ty::Mutez,
                    val: Val::Mutez(999),
                },
            },
            InstructionWrapper::Instruction {
                comment: None,
                instruction: Instruction::If {
                    instr1: vec![InstructionWrapper::Instruction {
                        comment: Some("This is a comment".to_string()),
                        instruction: Instruction::Push {
                            ty: Ty::Mutez,
                            val: Val::Mutez(999),
                        },
                    }],
                    instr2: vec![],
                },
            },
        ];
        let result = format(&instructions, 0, "    ");
        println!("{}", result);
        assert_eq!(result, String::from(""));
    }
}
