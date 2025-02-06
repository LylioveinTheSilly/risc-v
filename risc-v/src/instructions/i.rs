//! 32-bit Base Integer Instruction Set 

use super::Instruction;
use crate::{Exception, Word, RV32};
use crate::disassembly::{Disassembly, Operand::{Immediate, Register, Offset, RegisterOffset, RegisterUnsigned}};

/// Add registers
pub struct Add;
impl Instruction for Add {
    fn syntax(&self) -> &'static str { "add rd, rs1, rs2" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.funct7()) {
            (Word(0b_0110011), Word(0x0), Word(0x00)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("add", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Register(word.rs2())
        ]))
    }

    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (rs2, rs1, rd) = (word.rs2(), word.rs1(), word.rd());

        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;
        cpu.reg.write_gpr(rd, rs1v + rs2v)?;

        Ok(true)
    }
}

/// Subtract registers
pub struct Sub;
impl Instruction for Sub {
    fn syntax(&self) -> &'static str { "sub rd, rs1, rs2" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.funct7()) {
            (Word(0b_0110011), Word(0x0), Word(0x20)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("sub", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Register(word.rs2())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (rs2, rs1, rd) = (word.rs2(), word.rs1(), word.rd());

        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;
        cpu.reg.write_gpr(rd, rs1v - rs2v)?;

        Ok(true)
    }
}

/// Exclusive or registers
pub struct Xor;
impl Instruction for Xor {
    fn syntax(&self) -> &'static str { "xor rd, rs1, rs2" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.funct7()) {
            (Word(0b_0110011), Word(0x4), Word(0x00)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("xor", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Register(word.rs2())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (rs2, rs1, rd) = (word.rs2(), word.rs1(), word.rd());

        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;
        cpu.reg.write_gpr(rd, rs1v ^ rs2v)?;

        Ok(true)
    }
}

/// Or registers
pub struct Or;
impl Instruction for Or {
    fn syntax(&self) -> &'static str { "or rd, rs1, rs2" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.funct7()) {
            (Word(0b_0110011), Word(0x6), Word(0x00)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("or", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Register(word.rs2())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (rs2, rs1, rd) = (word.rs2(), word.rs1(), word.rd());

        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;
        cpu.reg.write_gpr(rd, rs1v | rs2v)?;

        Ok(true)
    }
}

/// And registers
pub struct And;
impl Instruction for And {
    fn syntax(&self) -> &'static str { "and rd, rs1, rs2" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.funct7()) {
            (Word(0b_0110011), Word(0x7), Word(0x00)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("and", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Register(word.rs2())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (rs2, rs1, rd) = (word.rs2(), word.rs1(), word.rd());

        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;
        cpu.reg.write_gpr(rd, rs1v & rs2v)?;

        Ok(true)
    }
}

/// Shift Left Logical
pub struct Sll;
impl Instruction for Sll {
    fn syntax(&self) -> &'static str { "sll rd, rs1, rs2" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.funct7()) {
            (Word(0b_0110011), Word(0x1), Word(0x00)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("sll", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Register(word.rs2())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (rs2, rs1, rd) = (word.rs2(), word.rs1(), word.rd());

        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;
        cpu.reg.write_gpr(rd, rs1v << rs2v)?;

        Ok(true)
    }
}

/// Shift Right Logical
pub struct Srl;
impl Instruction for Srl {
    fn syntax(&self) -> &'static str { "srl rd, rs1, rs2" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.funct7()) {
            (Word(0b_0110011), Word(0x5), Word(0x00)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("srl", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Register(word.rs2())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (rs2, rs1, rd) = (word.rs2(), word.rs1(), word.rd());

        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;
        cpu.reg.write_gpr(rd, rs1v >> rs2v)?;

        Ok(true)
    }
}

/// Shift Right Arithmetical
pub struct Sra;
impl Instruction for Sra {
    fn syntax(&self) -> &'static str { "sra rd, rs1, rs2" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.funct7()) {
            (Word(0b_0110011), Word(0x5), Word(0x20)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("sra", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Register(word.rs2())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (rs2, rs1, rd) = (word.rs2(), word.rs1(), word.rd());

        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;

        // To sign extend the shifted value, we can shift right value 0xFFFFFFFF
        // by the `rs2_val` amount and negate in to get sign extention, then "or" it 
        // with `rs1_val >> rs2_val`
        //
        // Example:
        //
        // sra xd, rs1, rs2
        //
        // rs1            = 0xF000_1234
        // rs2            = 0x0000_0004
        // 
        // sign_extention = 0xFFFF_FFFF >> rs2
        //                = 0x0FFF_FFFF !
        //                = 0xF000_0000
        //
        // rd             = 0xF000_1234 >> rs2
        //                = 0x0F00_0123 | sign_extention
        //                = 0xFF00_0123
        //
        // rd has value 0xFF00_0123 and it preserved its sign

        // Chech if number in register is signed
        let msb_extension = match rs1v & Word(0x80000000) {

            // Last bit is `1`, we need to preserve it
            Word(0x80000000) => !(Word::MAX >> rs2v),
                
            // No need to preserve sign
            Word(0x00000000) => Word(0),

            // Last bit can only be `0` or `1` (because its a bit :P)
            _ => unreachable!(),
        };

        cpu.reg.write_gpr(rd, (rs1v >> rs2v) | msb_extension)?;

        Ok(true)
    }
}

/// Set Less Than
pub struct Slt;
impl Instruction for Slt {
    fn syntax(&self) -> &'static str { "slt rd, rs1, rs2" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.funct7()) {
            (Word(0b_0110011), Word(0x2), Word(0x00)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("slt", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Register(word.rs2())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (rs2, rs1, rd) = (word.rs2(), word.rs1(), word.rd());

        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;

        let rs1sv = rs1v.0 as i32;
        let rs2sv = rs2v.0 as i32;

        let dstv = match rs1sv < rs2sv {
            true  => Word(1),
            false => Word(0)
        };

        cpu.reg.write_gpr(rd, dstv)?;

        Ok(true)
    }
}

/// Set Less Than Unsigned
pub struct Sltu;
impl Instruction for Sltu {
    fn syntax(&self) -> &'static str { "sltu rd, rs1, rs2" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.funct7()) {
            (Word(0b_0110011), Word(0x3), Word(0x00)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("sltu", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Register(word.rs2())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (rs2, rs1, rd) = (word.rs2(), word.rs1(), word.rd());

        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;

        let dstv = match rs1v < rs2v {
            true  => Word(1),
            false => Word(0)
        };

        cpu.reg.write_gpr(rd, dstv)?;

        Ok(true)
    }
}

/// Add immediate to register
pub struct Addi;
impl Instruction for Addi {
    fn syntax(&self) -> &'static str { "addi rd, rs1, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0010011), Word(0x0)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("addi", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Immediate(word.i_type_immediate())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.i_type_immediate(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        cpu.reg.write_gpr(rd, rs1v + imm)?;

        Ok(true)
    }
}

/// Xor immediate with register
pub struct Xori;
impl Instruction for Xori {
    fn syntax(&self) -> &'static str { "xori rd, rs1, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0010011), Word(0x4)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("xori", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Immediate(word.i_type_immediate())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.i_type_immediate(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        cpu.reg.write_gpr(rd, rs1v ^ imm)?;

        Ok(true)
    }
}

/// Or immediate with register
pub struct Ori;
impl Instruction for Ori {
    fn syntax(&self) -> &'static str { "ori rd, rs1, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0010011), Word(0x6)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("ori", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Immediate(word.i_type_immediate())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.i_type_immediate(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        cpu.reg.write_gpr(rd, rs1v | imm)?;

        Ok(true)
    }
}

/// And immediate with register
pub struct Andi;
impl Instruction for Andi {
    fn syntax(&self) -> &'static str { "andi rd, rs1, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0010011), Word(0x7)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("andi", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Immediate(word.i_type_immediate())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.i_type_immediate(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        cpu.reg.write_gpr(rd, rs1v & imm)?;

        Ok(true)
    }
}

/// Shift Left Logical Immediate
pub struct Slli;
impl Instruction for Slli {
    fn syntax(&self) -> &'static str { "slli rd, rs1, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0010011), Word(0x1)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("slli", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Immediate(word.shift_imm_amount())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.shift_imm_amount(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        cpu.reg.write_gpr(rd, rs1v << imm)?;

        Ok(true)
    }
}

/// Shift Right Logical Immediate
pub struct Srli;
impl Instruction for Srli {
    fn syntax(&self) -> &'static str { "srli rd, rs1, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.funct7()) {
            (Word(0b_0010011), Word(0x5), Word(0x00)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("srli", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Immediate(word.shift_imm_amount())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.shift_imm_amount(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        cpu.reg.write_gpr(rd, rs1v >> imm)?;

        Ok(true)
    }
}

/// Shift Right Arithmetical Immediate
pub struct Srai;
impl Instruction for Srai {
    fn syntax(&self) -> &'static str { "srai rd, rs1, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.funct7()) {
            (Word(0b_0010011), Word(0x5), Word(0x20)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("srai", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Immediate(word.shift_imm_amount())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.shift_imm_amount(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        
        // The explanation of what is going on here is in `sra`
        // instruction implementation.     
        let msb_extension = match rs1v & Word(0x80000000) {
            Word(0x80000000) => !(Word::MAX >> imm),
            Word(0x00000000) => Word(0),
            _ => unreachable!(),
        };

        cpu.reg.write_gpr(rd, (rs1v >> imm) | msb_extension)?;

        Ok(true)
    }
}

/// Set Less Than Immediate
pub struct Slti;
impl Instruction for Slti {
    fn syntax(&self) -> &'static str { "slti rd, rs1, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0010011), Word(0x2)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("slti", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Immediate(word.shift_imm_amount())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.i_type_immediate(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;

        let rs1vs = rs1v.0 as i32;
        let imms  = imm.0 as i32;

        let dstv = match rs1vs < imms {
            true  => Word(1),
            false => Word(0)
        };

        cpu.reg.write_gpr(rd, dstv)?;

        Ok(true)
    }
}

/// Set Less Than Immediate Unsigned
pub struct Sltiu;
impl Instruction for Sltiu {
    fn syntax(&self) -> &'static str { "sltiu rd, rs1, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0010011), Word(0x3)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("sltiu", vec![
            Register(word.rd()), 
            Register(word.rs1()), 
            Immediate(word.shift_imm_amount())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.i_type_immediate(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;

        let dstv = match rs1v < imm {
            true  => Word(1),
            false => Word(0)
        };

        cpu.reg.write_gpr(rd, dstv)?;

        Ok(true)
    }
}

/// Load Byte
pub struct Lb;
impl Instruction for Lb {
    fn syntax(&self) -> &'static str { "lb rd, imm(rs1)" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0000011), Word(0x0)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("lb", vec![
            Register(word.rd()), 
            Offset(word.i_type_immediate()),
            RegisterOffset(word.rs1()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.i_type_immediate(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;

        let address = rs1v + imm;
        let byte = cpu.bus.read(address);

        // sign extention
        let word = match byte & Word(0x80) {
            Word(0x00) => byte,
            Word(0x80) => Word(0xFFFFFF00) | byte,
            _ => unreachable!(),
        };

        cpu.reg.write_gpr(rd, word)?;

        Ok(true)
    }
}

/// Load Halfword
pub struct Lh;
impl Instruction for Lh {
    fn syntax(&self) -> &'static str { "lh rd, imm(rs1)" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0000011), Word(0x1)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("lh", vec![
            Register(word.rd()), 
            Offset(word.i_type_immediate()),
            RegisterOffset(word.rs1()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.i_type_immediate(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;

        let address = rs1v + imm;

        let (byte1, byte2) = (
            cpu.bus.read(address + Word(0)),
            cpu.bus.read(address + Word(1))
        );

        let halfword = (byte1 << Word(8)) | byte2;

        // sign extention
        let word = match halfword & Word(0x8000) {
            Word(0x0000) => halfword,
            Word(0x8000) => Word(0xFFFF0000) | halfword,
            _ => unreachable!(),
        };

        cpu.reg.write_gpr(rd, word)?;

        Ok(true)
    }
}

/// Load Word
pub struct Lw;
impl Instruction for Lw {
    fn syntax(&self) -> &'static str { "lw rd, imm(rs1)" }
    
    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0000011), Word(0x2)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("lw", vec![
            Register(word.rd()), 
            Offset(word.i_type_immediate()),
            RegisterOffset(word.rs1()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.i_type_immediate(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;

        let address = rs1v + imm;

        let (byte1, byte2, byte3, byte4) = (
            cpu.bus.read(address + Word(0)),
            cpu.bus.read(address + Word(1)),
            cpu.bus.read(address + Word(2)),
            cpu.bus.read(address + Word(3)),
        );

        let word = (byte1 << Word(24)) | (byte2 << Word(16)) | (byte3 << Word(8)) | byte4;
        cpu.reg.write_gpr(rd, word)?;

        Ok(true)
    }
}

/// Load Byte Unsigned
pub struct Lbu;
impl Instruction for Lbu {
    fn syntax(&self) -> &'static str { "lbu rd, imm(rs1)" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0000011), Word(0x4)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("lbu", vec![
            Register(word.rd()), 
            Offset(word.i_type_immediate()),
            RegisterOffset(word.rs1()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.i_type_immediate(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;

        let address = rs1v + imm;
        let word = cpu.bus.read(address);

        cpu.reg.write_gpr(rd, word)?;

        Ok(true)
    }
}

/// Load Halfword Unsigned
pub struct Lhu;
impl Instruction for Lhu {
    fn syntax(&self) -> &'static str { "lhu rd, imm(rs1)" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0000011), Word(0x5)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("lhu", vec![
            Register(word.rd()), 
            Offset(word.i_type_immediate()),
            RegisterOffset(word.rs1()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rd) = (word.i_type_immediate(), word.rs1(), word.rd());
        let rs1v = cpu.reg.read_gpr(rs1)?;

        let address = rs1v + imm;

        let (byte1, byte2) = (
            cpu.bus.read(address + Word(0)),
            cpu.bus.read(address + Word(1))
        );

        let word = (byte1 << Word(8)) | byte2;

        cpu.reg.write_gpr(rd, word)?;

        Ok(true)
    }
}

/// Store byte
pub struct Sb;
impl Instruction for Sb {
    fn syntax(&self) -> &'static str { "sb rs2, imm(rs1)" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0100011), Word(0x0)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("sb", vec![
            Register(word.rs2()), 
            Offset(word.s_type_immediate()),
            RegisterOffset(word.rs1()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rs2) = (word.s_type_immediate(), word.rs1(), word.rs2());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;

        let address = rs1v + imm;
        let word = rs2v & Word(0xFF);
        
        cpu.bus.write(address, word);

        Ok(true)
    }
}

/// Store Halfword
pub struct Sh;
impl Instruction for Sh {
    fn syntax(&self) -> &'static str { "sh rs2, imm(rs1)" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0100011), Word(0x1)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("sh", vec![
            Register(word.rs2()), 
            Offset(word.s_type_immediate()),
            RegisterOffset(word.rs1()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rs2) = (word.s_type_immediate(), word.rs1(), word.rs2());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;

        let address = rs1v + imm;
        let (byte1, byte2) = (
            (rs2v & Word(0xFF00)) >> Word(8),
            (rs2v & Word(0x00FF)),
        );
        
        cpu.bus.write(address + Word(0), byte1);
        cpu.bus.write(address + Word(1), byte2);

        Ok(true)
    }
}

/// Store Word
pub struct Sw;
impl Instruction for Sw {
    fn syntax(&self) -> &'static str { "sw rs2, imm(rs1)" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_0100011), Word(0x2)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("sw", vec![
            Register(word.rs2()), 
            Offset(word.s_type_immediate()),
            RegisterOffset(word.rs1()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rs2) = (word.s_type_immediate(), word.rs1(), word.rs2());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;

        let address = rs1v + imm;
        let (byte1, byte2, byte3, byte4) = (
            (rs2v & Word(0xFF000000)) >> Word(24),
            (rs2v & Word(0x00FF0000)) >> Word(16),
            (rs2v & Word(0x0000FF00)) >> Word(8),
            (rs2v & Word(0x000000FF)),
        );

        cpu.bus.write(address + Word(0), byte1);
        cpu.bus.write(address + Word(1), byte2);
        cpu.bus.write(address + Word(2), byte3);
        cpu.bus.write(address + Word(3), byte4);

        Ok(true)
    }
}

/// Break if EQual
pub struct Beq;
impl Instruction for Beq {
    fn syntax(&self) -> &'static str { "beq rs1, rs2, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_1100011), Word(0x0)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("beq", vec![
            Register(word.rs1()), 
            Register(word.rs2()),
            Offset(word.b_type_immediate()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rs2) = (word.b_type_immediate(), word.rs1(), word.rs2());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;

        if rs1v == rs2v {
            let pc = cpu.reg.read("pc")?;
            cpu.reg.write("pc", pc + imm)?;

            // We changed program counter, don't increment it
            // after execution of this instruction
            return Ok(false);

        } else {
            return Ok(true);
        }
    }
}

/// Break if Not Equal
pub struct Bne;
impl Instruction for Bne {
    fn syntax(&self) -> &'static str { "bne rs1, rs2, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_1100011), Word(0x1)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("bne", vec![
            Register(word.rs1()), 
            Register(word.rs2()),
            Offset(word.b_type_immediate()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rs2) = (word.b_type_immediate(), word.rs1(), word.rs2());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;

        if rs1v != rs2v {
            let pc = cpu.reg.read("pc")?;
            cpu.reg.write("pc", pc + imm)?;

            // We changed program counter, don't increment it
            // after execution of this instruction
            return Ok(false);

        } else {
            return Ok(true);
        }
    }
}

/// Break if Less Than
pub struct Blt;
impl Instruction for Blt {
    fn syntax(&self) -> &'static str { "blt rs1, rs2, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_1100011), Word(0x4)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("blt", vec![
            Register(word.rs1()), 
            Register(word.rs2()),
            Offset(word.b_type_immediate()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rs2) = (word.b_type_immediate(), word.rs1(), word.rs2());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;

        let rs1vs = rs1v.0 as i32;
        let rs2vs = rs2v.0 as i32;

        if rs1vs < rs2vs {
            let pc = cpu.reg.read("pc")?;
            cpu.reg.write("pc", pc + imm)?;

            // We changed program counter, don't increment it
            // after execution of this instruction
            return Ok(false);

        } else {
            return Ok(true);
        }
    }
}

/// Break if Greater or Equal than
pub struct Bge;
impl Instruction for Bge {
    fn syntax(&self) -> &'static str { "bge rs1, rs2, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_1100011), Word(0x5)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("bge", vec![
            Register(word.rs1()), 
            Register(word.rs2()),
            Offset(word.b_type_immediate()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rs2) = (word.b_type_immediate(), word.rs1(), word.rs2());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;

        let rs1vs = rs1v.0 as i32;
        let rs2vs = rs2v.0 as i32;

        if rs1vs >= rs2vs {
            let pc = cpu.reg.read("pc")?;
            cpu.reg.write("pc", pc + imm)?;

            // We changed program counter, don't increment it
            // after execution of this instruction
            return Ok(false);

        } else {
            return Ok(true);
        }
    }
}

/// Break if Less Than Unsigned
pub struct Bltu;
impl Instruction for Bltu {
    fn syntax(&self) -> &'static str { "bltu rs1, rs2, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_1100011), Word(0x6)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("bltu", vec![
            RegisterUnsigned(word.rs1()), 
            RegisterUnsigned(word.rs2()),
            Offset(word.b_type_immediate()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rs2) = (word.b_type_immediate(), word.rs1(), word.rs2());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;

        if rs1v < rs2v {
            let pc = cpu.reg.read("pc")?;
            cpu.reg.write("pc", pc + imm)?;

            // We changed program counter, don't increment it
            // after execution of this instruction
            return Ok(false);

        } else {
            return Ok(true);
        }
    }
}

/// Break if Greater or Equal than Unsigned
pub struct Bgeu;
impl Instruction for Bgeu {
    fn syntax(&self) -> &'static str { "bgeu rs1, rs2, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_1100011), Word(0x7)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("bgeu", vec![
            RegisterUnsigned(word.rs1()), 
            RegisterUnsigned(word.rs2()),
            Offset(word.b_type_immediate()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rs1, rs2) = (word.b_type_immediate(), word.rs1(), word.rs2());
        let rs1v = cpu.reg.read_gpr(rs1)?;
        let rs2v = cpu.reg.read_gpr(rs2)?;

        if rs1v >= rs2v {
            let pc = cpu.reg.read("pc")?;
            cpu.reg.write("pc", pc + imm)?;

            // We changed program counter, don't increment it
            // after execution of this instruction
            return Ok(false);

        } else {
            return Ok(true);
        }
    }
}

/// Jump And Link
pub struct Jal;
impl Instruction for Jal {
    fn syntax(&self) -> &'static str { "jal rd, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match word.opcode() {
            Word(0b_1101111) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("jal", vec![
            Register(word.rd()),
            Offset(word.j_type_immediate()),
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rd) = (word.j_type_immediate(), word.rd());

        let pc = cpu.reg.read("pc")?;
        cpu.reg.write("pc", pc + imm)?;
        cpu.reg.write_gpr(rd, pc + Word(4))?;

        // We changed program counter, don't increment it
        // after execution of this instruction
        return Ok(false);
    }
}

/// Jump And Link Register
pub struct Jalr;
impl Instruction for Jalr {
    fn syntax(&self) -> &'static str { "jalr rd, imm(rs1)" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3()) {
            (Word(0b_1100111), Word(0x0)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("jalr", vec![
            Register(word.rd()),
            Offset(word.i_type_immediate()),
            RegisterOffset(word.rs1())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rd, rs1) = (word.i_type_immediate(), word.rd(), word.rs1());
        let rs1v = cpu.reg.read_gpr(rs1)?;

        let pc = cpu.reg.read("pc")?;
        cpu.reg.write("pc", rs1v + imm)?;
        cpu.reg.write_gpr(rd, pc + Word(4))?;

        // We changed program counter, don't increment it
        // after execution of this instruction
        return Ok(false);
    }
}

/// Load Upper Immediate 
pub struct Lui;
impl Instruction for Lui {
    fn syntax(&self) -> &'static str { "lui rd, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match word.opcode() {
            Word(0b_0110111) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("lui", vec![
            Register(word.rd()),
            Immediate(word.u_type_immediate())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rd) = (word.u_type_immediate(), word.rd());

        cpu.reg.write_gpr(rd, imm)?;

        return Ok(true);
    }
}

/// Add Upper Immediate with Program Counter
pub struct Auipc;
impl Instruction for Auipc {
    fn syntax(&self) -> &'static str { "auipc rd, imm" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match word.opcode() {
            Word(0b_0010111) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("auipc", vec![
            Register(word.rd()),
            Immediate(word.u_type_immediate())
        ]))
    }
    fn execute(&self, word: Word, cpu: &RV32) -> Result<bool, Exception> {
        let (imm, rd) = (word.u_type_immediate(), word.rd());
        let pc = cpu.reg.read("pc")?;

        cpu.reg.write_gpr(rd, pc + imm)?;
        
        return Ok(true);
    }
}

/// Environment call
pub struct Ecall;
impl Instruction for Ecall {
    fn syntax(&self) -> &'static str { "ecall" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.i_type_immediate()) {
            (Word(0b_1110011), Word(0x0), Word(0x0)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("ecall", vec![]))
    }
    fn execute(&self, _word: Word, _cpu: &RV32) -> Result<bool, Exception> {
        Err(Exception::EnvironmentCall)
    }
}

/// Environment break
pub struct Ebreak;
impl Instruction for Ebreak {
    fn syntax(&self) -> &'static str { "ebreak" }

    fn validate(&self, word: Word) -> Result<(), Exception> {
        match (word.opcode(), word.funct3(), word.i_type_immediate()) {
            (Word(0b_1110011), Word(0x0), Word(0x1)) => Ok(()),
            _ => Err(Exception::InvalidInstruction)
        }
    }
    fn disassemble(&self, word: Word) -> Result<Disassembly, Exception> {
        self.validate(word)?;
        Ok(Disassembly::new("ebreak", vec![]))
    }
    fn execute(&self, _word: Word, _cpu: &RV32) -> Result<bool, Exception> {
        Err(Exception::EnvironmentBreak)
    }
}
