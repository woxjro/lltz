use serde::{Deserialize, Serialize};

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
    pub fn print(&self) {
        println!("while {{ {} }} (%{}) {{", self.header, self.header_cond);
        for lp in &self.sub_loops {
            lp.print();
        }
        println!("  {}", self.latch);
        println!("}} {}", self.exiting);
    }
}
