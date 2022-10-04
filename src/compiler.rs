use super::mini_llvm::Instruction;
use std::collections::HashMap;
mod phase;
pub fn compile(instructions: Vec<Instruction>) -> String {
    //レジスタの下処理
    let mut register2stack_ptr = HashMap::new();
    let mut memory_types = HashMap::new();
    let mut stack_ptr = 0;
    let mut memory_ptr = 0;

    let mut michelson_code = String::new();
    let space = "       ";

    phase::analyse_registers_and_memory(
        &mut register2stack_ptr,
        &mut memory_types,
        &mut stack_ptr,
        &mut memory_ptr,
        &instructions,
    );
    dbg!(&register2stack_ptr);
    dbg!(&memory_types);

    michelson_code = phase::prepare(
        michelson_code,
        space,
        &mut register2stack_ptr,
        &mut memory_types,
    );

    michelson_code = phase::body(
        michelson_code,
        space,
        &mut register2stack_ptr,
        &mut memory_types,
        &instructions,
    );

    //後処理:レジスタ領域・メモリ領域をDROPする
    michelson_code = phase::exit(
        michelson_code,
        space,
        &mut register2stack_ptr,
        &mut memory_types,
    );
    michelson_code
}
