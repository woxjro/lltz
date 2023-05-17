use crate::formatter::format;
use crate::ty::Ty;
use crate::wrapped_instruction::WrappedInstruction;
use std::string::ToString;

pub struct Program {
    pub storage: Ty,
    pub parameter: Ty,
    pub code: Vec<WrappedInstruction>,
}

impl ToString for Program {
    fn to_string(&self) -> String {
        format!(
            r#"parameter {};
storage {};
code {{
{}
     }}"#,
            self.parameter.to_string(),
            self.storage.to_string(),
            format(&self.code, 7)
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::instruction::Instruction;
    use crate::program::Program;
    use crate::ty::Ty;
    use crate::wrapped_instruction::WrappedInstruction;
    #[test]
    fn it_works() {
        let program = Program {
            storage: Ty::Unit,
            parameter: Ty::Unit,
            code: vec![
                WrappedInstruction {
                    comment: Some("=> Unit".to_string()),
                    instruction: Instruction::Cdr,
                },
                WrappedInstruction {
                    comment: Some("=> {} : Unit".to_string()),
                    instruction: Instruction::Nil { ty: Ty::Operation },
                },
                WrappedInstruction {
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
