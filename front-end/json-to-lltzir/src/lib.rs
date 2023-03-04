use serde::{Deserialize, Serialize};

use serde_json::Value;
use String as Label;
use String as Register;

#[derive(Serialize, Deserialize, Debug)]
pub struct Cfg {
    blocks: Vec<Label>,
    exiting: Label,
    header: Label,
    header_cond: Register,
    latch: Label,
    parent: Option<Label>,
    r#loop: Label,
    sub_loops: Vec<Cfg>,
}

impl Cfg {
    fn instructions_to_string(instructions: &Value) -> String {
        let mut res = "".to_string();
        match instructions {
            Value::Array(instructions) => {
                for instruction in instructions {
                    if !instruction["original_instruction"]
                        .to_string()
                        .contains("br")
                    {
                        let s = instruction["original_instruction"].to_string();
                        let r = s.trim_matches(|c| c == ' ' || c == '"');

                        res = format!("{res}\n{r}");
                    }
                }
            }
            e => {
                panic!("{:?}", e)
            }
        };

        format!("{res}\n")
    }
    pub fn print(&self, lltz_ir: &Value) {
        let blocks = &lltz_ir["functions"][0]["blocks"];
        println!(
            "while {{ {} }} (%{}) {{",
            Cfg::instructions_to_string(&blocks[self.header.clone()]["instructions"]),
            self.header_cond
        );
        for lp in &self.sub_loops {
            lp.print(lltz_ir);
        }
        println!(
            "  {}",
            Cfg::instructions_to_string(&blocks[self.latch.clone()]["instructions"])
        );
        println!("}}");
        println!(
            "{}",
            Cfg::instructions_to_string(&blocks[self.exiting.clone()]["instructions"])
        );
    }
}
