pub fn format(michelson_instructions: &Vec<String>, tab: &str, tab_depth: usize) -> String {
    let mut indent = String::new();
    for _ in 0..tab_depth {
        indent.push_str(tab);
    }

    michelson_instructions
        .iter()
        .map(|e| format!("{indent}{e}\n"))
        .collect::<String>()
}
