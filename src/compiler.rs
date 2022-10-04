use super::mini_llvm::Instruction;
use super::mini_llvm::Type;
use std::collections::HashMap;
mod phase;
pub fn compile(instructions: Vec<Instruction>) -> String {
    let mut register2stack_ptr = HashMap::new();
    let mut register2ty: HashMap<String, Type> = HashMap::new();
    let mut memory_types = HashMap::new();
    let mut stack_ptr = 0;
    let mut memory_ptr = 0;

    let mut michelson_code = String::new();
    let space = "       ";

    //レジスタの下処理
    phase::analyse_registers_and_memory(
        &mut register2stack_ptr,
        &mut register2ty,
        &mut memory_types,
        &mut stack_ptr,
        &mut memory_ptr,
        &instructions,
    );

    dbg!(&register2stack_ptr);
    dbg!(&register2ty);
    dbg!(&memory_types);

    michelson_code = phase::prepare(
        michelson_code,
        space,
        &mut register2stack_ptr,
        &mut register2ty,
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
