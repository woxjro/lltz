use super::instruction::Instruction;

pub fn format(instructions: &Vec<Instruction>, depth: usize, tab: &str) -> String {
    let mut res = String::from("");
    for instruction in instructions {
        res = match instruction {
            Instruction::Comment(_) => {
                format!(
                    r#"{res}
{}"#,
                    instruction.to_formatted_string(depth, tab)
                )
            }
            _ => format!(
                r#"{res}
{};"#,
                instruction.to_formatted_string(depth, tab)
            ),
        };
    }
    res.trim_matches('\n').to_string()
}

#[cfg(test)]
mod tests {
    use super::format;
    use crate::instruction::Instruction;
    use crate::ty::Ty;
    use crate::val::Val;
    #[test]
    fn it_works() {
        let instructions = vec![Instruction::If {
            instr1: vec![
                Instruction::Comment("This is a comment".to_string()),
                Instruction::Push {
                    ty: Ty::Mutez,
                    val: Val::Mutez(999),
                },
            ],
            instr2: vec![Instruction::If {
                instr1: vec![],
                instr2: vec![],
            }],
        }];
        let result = format(&instructions, 0, "    ");
        println!("{}", result);
        assert_eq!(result, String::from(""));
    }
}
