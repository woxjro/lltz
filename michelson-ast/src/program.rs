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
            storage: Ty::Unit,
            parameter: Ty::Unit,
            code: vec![
                InstructionWithComment {
                    comment: Some("=> Unit".to_string()),
                    instruction: Instruction::Cdr,
                },
                InstructionWithComment {
                    comment: Some("=> {} : Unit".to_string()),
                    instruction: Instruction::Nil { ty: Ty::Operation },
                },
                InstructionWithComment {
                    comment: Some("=> (Pair {} Unit)".to_string()),
                    instruction: Instruction::Pair,
                },
            ],
        };

        let result = program.to_string();
        println!("{}", result);
        assert_eq!(result, String::from("storage unit;\nparameter unit;\ncode {\n       CDR; # => Unit\n       NIL operation; # => {} : Unit\n       PAIR; # => (Pair {} Unit)\n     }"));
    }
}
