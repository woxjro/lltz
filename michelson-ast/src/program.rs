use crate::formatter::format;
use crate::instruction_with_comment::InstructionWithComment;
use crate::ty::Ty;

pub struct Program {
    pub storage: Ty,
    pub parameter: Ty,
    pub code: Vec<InstructionWithComment>,
}

impl Program {
    pub fn to_string(&self) -> String {
        format!(
            r#"storage {};
parameter {};
code {{
{}
     }}"#,
            self.storage.to_string(),
            self.parameter.to_string(),
            format(&self.code, 7)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;
    use crate::instruction_with_comment::InstructionWithComment;
    use crate::program::Program;
    use crate::ty::Ty;
    use crate::val::Val;
    #[test]
    fn it_works() {
        let program = Program {
            storage: Ty::Mutez,
            parameter: Ty::Nat,
            code: vec![
                InstructionWithComment {
                    comment: None,
                    instruction: Instruction::Comment("### Comment ###".to_string()),
                },
                InstructionWithComment {
                    comment: Some("This is a comment".to_string()),
                    instruction: Instruction::Push {
                        ty: Ty::Mutez,
                        val: Val::Mutez(999),
                    },
                },
                InstructionWithComment {
                    comment: Some("This is a comment".to_string()),
                    instruction: Instruction::Push {
                        ty: Ty::Nat,
                        val: Val::Nat(999),
                    },
                },
                InstructionWithComment {
                    comment: None,
                    instruction: Instruction::If {
                        instr1: vec![InstructionWithComment {
                            comment: Some("This is a comment".to_string()),
                            instruction: Instruction::Push {
                                ty: Ty::Mutez,
                                val: Val::Mutez(999),
                            },
                        }],
                        instr2: vec![],
                    },
                },
            ],
        };

        let result = program.to_string();
        println!("{}", result);
        assert_eq!(result, String::from(""));
    }
}
