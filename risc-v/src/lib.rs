pub mod exception;
pub mod register;
pub mod disassembly;
pub mod instructions;
pub mod devices;
pub mod word;
pub mod bus;

use instructions::INSTRUCTION_SET;
pub use word::Word;

use bus::Bus;
use register::RV32IRegisters;
use exception::Exception;

#[derive(Debug, Clone, Copy)]
pub struct MemoryRange {
    base: Word,
    offset: Word,
}
impl MemoryRange {
    pub fn new(base: Word, offset: Word) -> Self {
        return Self { base, offset };
    }

    pub fn contains(&self, address: Word) -> bool {
        let end = match self.base.checked_add(self.offset.0) {
            None => Word::MAX,
            Some(n) => Word(n),
        };

        address < self.base && address > end
    }

    pub fn intersects(&self, other: Self) -> bool {
        use std::cmp::Ordering::{Greater, Less};

        let self_end = match self.base.checked_add(self.offset.0) {
            None => Word::MAX,
            Some(n) => Word(n),
        };

        let other_end = match other.base.checked_add(self.offset.0) {
            None => Word::MAX,
            Some(n) => Word(n),
        };

        let (c1, c2, c3, c4) = (
            self.base.cmp(&other.base),
            self.base.cmp(&other_end),
            self_end.cmp(&other.base),
            self_end.cmp(&other_end),  
        );

        match (c1, c2, c3, c4) {
            (Less, Less, Less, Less) |
            (Greater, Greater, Greater, Greater) => {
                return true;
            }
            _ => {
                return false;
            }
        }
    }
}

pub struct RV32 {
    /// RV32 registers
    pub reg: RV32IRegisters,

    /// Address bus with devices
    pub bus: Bus,
}
impl RV32 {
    pub fn new() -> Self {
        let bus = Bus::new();
        Self { reg: RV32IRegisters::new(), bus }
    }

    /// fetch next word pointed by program counter
    pub fn fetch(&self) -> Result<Word, Exception> {
        let pc = self.reg.read("pc").unwrap();
        let inst_word = self.bus.read_le_word(pc);
        Ok(inst_word)
    }

    /// Instrement Program Counter
    pub fn increment_pc(&self) {
        let pc = self.reg.read("pc").unwrap();
        self.reg.write("pc", pc + Word(4)).unwrap();
    }

    /// Fetch and execute one instruction and clock the bus once
    pub fn step(&self) -> Result<(), Exception> {
        let word = self.fetch()?;
        let instruction = INSTRUCTION_SET.decode(word)?;
        let increment_pc = instruction.execute(word, self)?;

        if increment_pc {
            self.increment_pc()
        }

        self.bus.tick();
        Ok(())
    }
}

