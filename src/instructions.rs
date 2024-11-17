#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Instruction {
    Add(usize, usize),
    Sub(usize, usize),
    Mul(usize, usize),
    Div(usize, usize),
}
