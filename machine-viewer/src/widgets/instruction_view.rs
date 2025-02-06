use risc_v::{RV32, Word};
use risc_v::instructions::{INSTRUCTION_SET, Instruction};
use risc_v::disassembly::Operand::{Immediate, Register, RegisterUnsigned, Offset, RegisterOffset};
use ratatui::prelude::*;
use ratatui::widgets::{Widget, Block};
use ratatui::style::Color;

use super::ValueFormat;

pub struct InstructionView<'a> {
    cpu: &'a RV32,
    block: Option<Block<'a>>,
    
    offset: Option<Word>,
    display_immediates: ValueFormat,
    display_offsets: ValueFormat,
    display_regisers_aliases: bool,
}
impl<'a> InstructionView<'a> {
    pub fn new(cpu: &'a RV32) -> Self {
        Self { 
            cpu, block: None, offset: None,
            display_immediates: ValueFormat::default(),
            display_offsets: ValueFormat::default(),
            display_regisers_aliases: true,
        }
    }
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
    pub fn offset(mut self, offset: Word) -> Self {
        self.offset = Some(offset);
        self
    }
    pub fn display_immediates(mut self, display_immediates: ValueFormat) -> Self {
        self.display_immediates = display_immediates;
        self
    }
    pub fn display_offsets(mut self, display_offsets: ValueFormat) -> Self {
        self.display_offsets = display_offsets;
        self
    }
    pub fn display_regisers_aliases(mut self, display_regisers_aliases: bool) -> Self {
        self.display_regisers_aliases = display_regisers_aliases;
        self
    }
    fn format_register_name(&self, register_idx: Word) -> String {
        match self.display_regisers_aliases {
            false => format!("x{:.3}" , register_idx.0),
            true => {
                if let Some(reg) = self.cpu.reg.get_gpr(register_idx) {
                    reg.aliases.iter()
                        .next()
                        .map(|a| format!("{a:.4}"))
                        .unwrap_or(format!("x{:.3}" , register_idx.0))
                } else {
                    format!("x{:.3}" , register_idx.0)
                }
            }
        }
    }
    fn format_instruction(&self, inst: &Box<dyn Instruction>, word: Word) -> String {
        let disasm = match inst.disassemble(word) {
            Ok(disasm) => disasm,
            Err(_) => return format!("invalid inst."),
        };

        let mnemonic = disasm.mnemonic;
        let mut iter = disasm.operands.iter();
        let operands = (iter.next(), iter.next(), iter.next());

        match operands {
            (None, None, None) => format!("{mnemonic}"),
            (Some(Register(r1)), Some(Register(r2)), Some(Register(r3))) => {
                format!("{mnemonic:6} {}, {}, {}", 
                    self.format_register_name(*r1), 
                    self.format_register_name(*r2), 
                    self.format_register_name(*r3),
                )
            },
            (Some(Register(r1)), Some(Register(r2)), Some(Immediate(imm))) => {
                use ValueFormat::*;
                
                let imm_string = match self.display_immediates {
                    DecimalSigned => format!("{}", imm.signed()),
                    DecimalUnsigned => format!("{}", imm.unsigned()),
                    Hexadecimal => format!("{:08X}", imm.unsigned()),
                };

                format!("{mnemonic:6} {}, {}, {imm_string}", 
                    self.format_register_name(*r1), 
                    self.format_register_name(*r2), 
                )
            },
            (Some(Register(r1)), Some(Offset(off)), Some(RegisterOffset(ro))) => {
                use ValueFormat::*;

                let off_string = match self.display_offsets {
                    DecimalSigned => format!("{}", off.signed()),
                    DecimalUnsigned => format!("{}", off.unsigned()),
                    Hexadecimal => format!("{:08X}", off.unsigned()),
                };

                format!("{mnemonic:6} {}, {off_string}({})", 
                    self.format_register_name(*r1), 
                    self.format_register_name(*ro), 
                )
            },
            (Some(Register(r1)), Some(Register(r2)), Some(Offset(off))) => {
                use ValueFormat::*;

                let off_string = match self.display_offsets {
                    DecimalSigned => format!("{}", off.signed()),
                    DecimalUnsigned => format!("{}", off.unsigned()),
                    Hexadecimal => format!("{:08X}", off.unsigned()),
                };

                format!("{mnemonic:6} {}, {}, {off_string}", 
                    self.format_register_name(*r1), 
                    self.format_register_name(*r2),
                )
            },
            (Some(RegisterUnsigned(r1)), Some(RegisterUnsigned(r2)), Some(Offset(off))) => {
                use ValueFormat::*;

                let off_string = match self.display_offsets {
                    DecimalSigned => format!("{}", off.signed()),
                    DecimalUnsigned => format!("{}", off.unsigned()),
                    Hexadecimal => format!("{:08X}", off.unsigned()),
                };

                format!("{mnemonic:6} {}, {}, {off_string}", 
                    self.format_register_name(*r1), 
                    self.format_register_name(*r2), 
                )
            },
            (Some(Register(r1)), Some(Offset(off)), None) => {
                use ValueFormat::*;

                let off_string = match self.display_offsets {
                    DecimalSigned => format!("{}", off.signed()),
                    DecimalUnsigned => format!("{}", off.unsigned()),
                    Hexadecimal => format!("{:08X}", off.unsigned()),
                };

                format!("{mnemonic:6} {}, {off_string}", 
                    self.format_register_name(*r1),
                )
            },
            (Some(Register(r1)), Some(Immediate(imm)), None) => {
                use ValueFormat::*;
                
                let imm_string = match self.display_immediates {
                    DecimalSigned => format!("{}", imm.signed()),
                    DecimalUnsigned => format!("{}", imm.unsigned()),
                    Hexadecimal => format!("{:08X}", imm.unsigned()),
                };

                format!("{mnemonic:6} {}, {imm_string}", 
                    self.format_register_name(*r1),
                )
            },
            _ => format!("unknown inst. format")
        }
    }
}
impl<'a> Widget for InstructionView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = match self.block.clone() {
            Some(block) => {
                let inner_area = block.inner(area);
                block.render(area, buf);
                inner_area
            },
            None => area,
        };

        let current_inst_offset = match self.offset {
            Some(offset) => offset,
            None => self.cpu.reg.read("pc").unwrap()
        };

        let top_instruction_offset = current_inst_offset - Word((area.height as u32 / 2) * 4);

        for line_number in 0..area.height {
            let offset = top_instruction_offset + Word(line_number as u32 * 4);
            
            let inst_word = self.cpu.bus.read_le_word(offset);

            let line = match INSTRUCTION_SET.decode(inst_word) {
                Ok(inst) if offset == current_inst_offset => {
                    let inst_string = self.format_instruction(inst, inst_word);
                    
                    Line::from(vec![
                        Span::from(format!("{:08X}: ", offset.unsigned())),
                        Span::styled(inst_string, Style::default().light_cyan())
                    ])
                },
                Ok(inst) => {
                    let inst_string = self.format_instruction(inst, inst_word);
                    
                    Line::from(vec![
                        Span::from(format!("{:08X}: ", offset.unsigned())),
                        Span::styled(inst_string, Style::default().fg(Color::Gray))
                    ])
                },
                Err(_) => {
                    Line::from(vec![
                        Span::from(format!("{:08X}: ", offset.unsigned())),
                        Span::styled("invalid instruction", Style::default().light_red())
                    ])
                },
            };

            buf.set_line(area.x, area.y + line_number, &line, area.width);
        }
    }

}
