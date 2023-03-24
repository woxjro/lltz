# michelson-ast

## Overview
michelson-ast-rs is a Rust library for generating Michelson code. This library can handle the Abstract Syntax Tree (AST) of Michelson, the smart contract language for Tezos.

## Usage
To generate Michelson code using this library, you can write a program like the following:
```rust
use michelson_ast::instruction::Instruction;
use michelson_ast::instruction_with_comment::WrappedInstruction;
use michelson_ast::program::Program;
use michelson_ast::ty::Ty;
use michelson_ast::val::Val;

fn main() {
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
}
```

## Example output
```
storage unit;
parameter unit;
code {
       CDR; # => Unit
       NIL operation; # => {} : Unit
       PAIR; # => (Pair {} Unit)
     }
```
