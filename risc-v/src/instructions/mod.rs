use crate::{RV32, Word}; 
use crate::exception::Exception;
use std::fmt::{Debug, Formatter};
use crate::disassembly::Disassembly;

pub mod i;
pub use i::*;
use lazy_static::lazy_static;

/// A instruction that can be executed on a RISC-V machine.
pub trait Instruction {

    /// Return a syntax of the instruction
    fn syntax(&self) -> &'static str;

    /// Return `Ok(())` if `format` represents instruction
    /// `Err(Exception::InvalidInstruction)` otherwise.
    fn validate(&self, word: Word) -> Result<(), Exception>;

    /// Return string representaion of an instruction 
    /// or `Err(Exception::InvalidInstruction)` if format does
    /// not represent instruction
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception>;

    /// Execute instruction, return `Ok(true)` if instruction executed successfully
    /// and did not changed program counter. `Ok(false)` signals to the cpu that 
    /// instruction modified program counter and cpu shouldn't increment it.
    /// `Err(Exception)` is returned if any exception occured 
    /// during execution of an instruction.
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception>;
}
impl Debug for dyn Instruction {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}", self.syntax())
    }
}

#[derive(Debug, Default)]
pub struct InstructionSet {
    instructions: Vec<Box<dyn Instruction>>,
}
unsafe impl Sync for InstructionSet {}
impl InstructionSet {
    pub fn append(&mut self, instruction: Box<dyn Instruction>) {
        self.instructions.push(instruction);
    }
    pub fn decode(&self, word: Word) -> Result<&Box<dyn Instruction>, Exception> {
        for inst in self.instructions.iter() {
            if inst.validate(word).is_ok() {
                return Ok(inst);
            }
        }

        Err(Exception::InvalidInstruction)
    }
}

macro_rules! instruction_set {
    ($instruction_set:ident, $($instruction:ident),*) => {
        $(
            $instruction_set.append(Box::new($instruction));
        )*
    };
}

lazy_static! {
    pub static ref INSTRUCTION_SET: InstructionSet = {
        let mut instruction_set = InstructionSet::default();
        
        instruction_set!(instruction_set,
            Add, Sub, Xor, Or, And, Sll, Srl, Sra,
            Slt, Sltu, Addi, Xori, Ori, Andi, Slli,
            Srli, Srai, Slti, Sltiu, Lb, Lh, Lw, Lbu, 
            Lhu, Sb, Sh, Sw, Beq, Bne, Blt, Bge, Bltu, 
            Bgeu, Jal, Jalr, Lui, Auipc, Ecall, Ebreak
        );

        instruction_set
    };
}