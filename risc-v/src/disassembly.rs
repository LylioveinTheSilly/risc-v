//! Helper structs for disassembling instructions
use crate::Word;

#[derive(Debug, Clone, Copy)]
pub enum Operand {
    /// Immediate value
    Immediate(Word),
    /// Register index
    Register(Word),
    /// Register index whose value is added to offset
    /// to calculate effective jump offset
    RegisterOffset(Word),
    /// Register index whose value is treated as unsigned number
    RegisterUnsigned(Word),
    /// Offset to a memory relative to program counter
    Offset(Word),
}

#[derive(Debug, Clone)]
pub struct Disassembly {
    pub mnemonic: &'static str,
    pub operands: Vec<Operand>,
}
impl Disassembly {
    pub fn new(mnemonic: &'static str, operands: Vec<Operand>) -> Self {
        Self { mnemonic, operands }
    }
}