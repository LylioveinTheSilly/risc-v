use crate::{Word, exception::Exception};
use std::{cell::Cell, fmt::Debug, ops::Not};
#[derive(Debug, Clone)]
pub struct Register {
    pub aliases: Vec<String>,
    pub value: Cell<Word>,
}
impl Register {
    pub fn new(aliases: Vec<&str>) -> Self {
        Register {
            aliases: aliases.iter().map(|s| s.to_string()).collect(),
            value: Cell::default(),
        }
    }
    pub fn read(&self) -> Word {
        if self.aliases.iter().find(|a| a == &"zero").is_some() {
            return Word(0);
        }

        return self.value.get();
    }
    pub fn write(&self, word: Word) {
        if self.aliases.iter().find(|a| a == &"zero").is_none() {
            self.value.set(word);
        }
    }
}

macro_rules! registers {
    ($($aliases:expr),*) => {
        [
            $(
                Register::new($aliases),
            )*
        ]
    };
}

#[derive(Clone)]
pub struct RV32IRegisters {
    /// Program counter
    pc: Register,

    /// General purpose RV32I base registers
    base: [Register; 32],
}
impl RV32IRegisters {
    pub fn new() -> Self {
        
        Self {
            pc: Register::new(vec!["pc"]),
            base: registers! {
                vec!["x0",  "zero"],     vec!["x1", "ra"],  vec!["x2", "sp"],   vec!["x3", "gp"],
                vec!["x4",  "tp"],       vec!["x5", "t0"],  vec!["x6", "t1"],   vec!["x7", "t2"],
                vec!["x8",  "s0", "fp"], vec!["x9", "s1"],  vec!["x10", "a0"],  vec!["x11", "a1"],
                vec!["x12", "a2"],       vec!["x13", "a3"], vec!["x14", "a4"],  vec!["x15", "a5"],
                vec!["x16", "a6"],       vec!["x17", "a7"], vec!["x18", "s2"],  vec!["x19", "s3"],
                vec!["x20", "s4"],       vec!["x21", "s5"], vec!["x22", "s6"],  vec!["x23", "s7"],
                vec!["x24", "s8"],       vec!["x25", "s9"], vec!["x26", "s10"], vec!["x27", "s11"],
                vec!["x28", "t3"],       vec!["x29", "t4"], vec!["x30", "t5"],  vec!["x31", "t6"]
            },
        }
    }

    /// Read general purpose integer register; 
    /// Valid indexes are between 0 and 31
    pub fn read_gpr(&self, idx: Word) -> Result<Word, Exception> {
        if (0..31).contains(&(idx.0 as usize)).not() {
            return Err(Exception::InvalidRegister);
        }

        Ok(self.base[idx.0 as usize].read())
    }

    /// Write to general purpose integer register; 
    /// Valid indexes are between 0 and 31
    pub fn write_gpr(&self, idx: Word, word: Word) -> Result<(), Exception> {
        if (0..31).contains(&(idx.0 as usize)).not() {
            return Err(Exception::InvalidRegister);
        }

        if idx.0 == 0 {
            // Writing to x0 has no effect
            return Ok(())
        }

        self.base[idx.0 as usize].write(word);
        Ok(())
    }

    /// Read from register identified by name
    pub fn read(&self, name: &str) -> Result<Word, Exception> {
        if name == "pc" {
            return Ok(self.pc.read());
        }

        if let Some(word) = self.base
            .iter()
            .find(|r| r.aliases.iter().find(|a| a == &name).is_some())
            .map(|reg| reg.read()) 
        {
            return Ok(word);
        }

        Err(Exception::InvalidRegister)
    }

    /// Write to register identified by name
    pub fn write(&self, name: &str, value: Word) -> Result<(), Exception> {
        if name == "pc" {
            self.pc.write(value);
            return Ok(());
        }

        if let Some(reg) = self.base
            .iter()
            .find(|r| r.aliases.iter().find(|a| a == &name).is_some())
        {
            reg.write(value);
            return Ok(());
        }

        Err(Exception::InvalidRegister)
    }

    /// Get a reference to register
    pub fn get(&self, name: &str) -> Option<&Register> {
        if name == "pc" {
            return Some(&self.pc);
        }

        if let Some(reg) = self.base
            .iter()
            .find(|r| r.aliases.iter().find(|a| a == &name).is_some())
        {
            return Some(&reg);
        }

        None
    } 

    /// Get a reference to base register by index
    pub fn get_gpr(&self, idx: Word) -> Option<&Register> {
        if (0..31).contains(&(idx.0 as usize)).not() {
            return None;
        }

        Some(&self.base[idx.0 as usize])
    } 
}

impl Debug for RV32IRegisters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        write!(f, concat!(
            "pc: {:08X}\n",
            "x0  (zero):  {:08X}  x1  (ra): {:08X}  x2  (sp):  {:08X}  x3  (gp):  {:08X}\n",
            "x4  (tp):    {:08X}  x5  (t0): {:08X}  x6  (t1):  {:08X}  x7  (t2):  {:08X}\n",
            "x8  (s0/fp): {:08X}  x9  (s1): {:08X}  x10 (a0):  {:08X}  x11 (a1):  {:08X}\n",
            "x12 (a2):    {:08X}  x13 (a3): {:08X}  x14 (a4):  {:08X}  x15 (a5):  {:08X}\n",
            "x16 (a6):    {:08X}  x17 (a7): {:08X}  x18 (s2):  {:08X}  x19 (s3):  {:08X}\n",
            "x20 (s4):    {:08X}  x21 (s5): {:08X}  x22 (s6):  {:08X}  x23 (s7):  {:08X}\n",
            "x24 (s8):    {:08X}  x25 (s9): {:08X}  x26 (s10): {:08X}  x27 (s11): {:08X}\n",
            "x28 (t3):    {:08X}  x29 (t4): {:08X}  x30 (t5):  {:08X}  x31 (t6):  {:08X}\n",
        ),
            self.read("pc").unwrap().0,
            self.read("x0").unwrap().0,  self.read("x1").unwrap().0,  self.read("x2").unwrap().0,  self.read("x3").unwrap().0,
            self.read("x4").unwrap().0,  self.read("x5").unwrap().0,  self.read("x6").unwrap().0,  self.read("x7").unwrap().0,
            self.read("x8").unwrap().0,  self.read("x9").unwrap().0,  self.read("x10").unwrap().0, self.read("x11").unwrap().0,
            self.read("x12").unwrap().0, self.read("x13").unwrap().0, self.read("x14").unwrap().0, self.read("x15").unwrap().0,
            self.read("x16").unwrap().0, self.read("x17").unwrap().0, self.read("x18").unwrap().0, self.read("x19").unwrap().0,
            self.read("x20").unwrap().0, self.read("x21").unwrap().0, self.read("x22").unwrap().0, self.read("x23").unwrap().0,
            self.read("x24").unwrap().0, self.read("x25").unwrap().0, self.read("x26").unwrap().0, self.read("x27").unwrap().0,
            self.read("x28").unwrap().0, self.read("x29").unwrap().0, self.read("x30").unwrap().0, self.read("x31").unwrap().0,
        )
    }
}