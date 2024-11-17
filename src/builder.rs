use std::io::BufWriter;

use crate::instructions::Instruction;

pub struct Builder {
    insrt: Vec<Instruction>,
}

impl Builder {
    pub fn new() -> Self {
        Self { insrt: vec![] }
    }

    pub fn emit(&self) -> Self {
        let mut output = String::new();

        let mut instr_ptr = 0;
        while isntr_ptr < self.insrt.len() {
            instr_ptr += 1;
        }
    }
}
