use super::instruction::Instruction;

pub fn format(instructions: Vec<Instruction>, depth: usize) -> String {
    let mut res = String::from("");
    for instruction in instructions {
        if instruction.has_instructions() {
            //depth分全体をずらして足す
            todo!();
        } else {
            let space = "   ".repeat(depth);
            res = format!("{res}{space}{};\n", instruction.to_string());
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::format;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
