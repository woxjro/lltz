use michelson_ast::{
    instruction::Instruction, program::Program, ty::Ty, wrapped_instruction::WrappedInstruction,
};

fn main() {
    let program = Program {
        storage: Ty::Unit,
        parameter: Ty::Unit,
        code: vec![
            WrappedInstruction {
                comment: Some("=> Unit".to_owned()),
                instruction: Instruction::Cdr,
            },
            WrappedInstruction {
                comment: Some("=> {} : Unit".to_owned()),
                instruction: Instruction::Nil { ty: Ty::Operation },
            },
            WrappedInstruction {
                comment: Some("=> (Pair {} Unit)".to_owned()),
                instruction: Instruction::Pair,
            },
        ],
    };

    println!("{}", program.to_string());
}
