use risc_v::{RV32, Word};
use ratatui::prelude::*;
use ratatui::widgets::{Widget, Block};

#[derive(Debug, Clone, Copy, Default)]
pub enum SegmentMemoryView {
    #[default]
    PerByte,
    PerHalfword,
    PerWord,
}

pub struct MemoryView<'a> {
    cpu: &'a RV32,
    show_segments: SegmentMemoryView,
    block: Option<Block<'a>>,
    offset: Word,
}
impl<'a> MemoryView<'a> {
    pub fn new(cpu: &'a RV32) -> Self {
        Self { cpu, 
            show_segments: SegmentMemoryView::default(),
            block: None, offset: Word(0),
        }
    }
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
    pub fn offset(mut self, offset: Word) -> Self {
        self.offset = offset;
        self
    }
    pub fn show_segments(mut self, segments: SegmentMemoryView) -> Self {
        self.show_segments = segments;
        self
    }
}
impl<'a> Widget for MemoryView<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = match self.block.clone() {
            Some(block) => {
                let mut inner_area = block.inner(area);
                block.render(area, buf);
                inner_area.x += 1;
                inner_area.width -= 2;
                inner_area
            },
            None => area,
        };

        let mem_per_line = 0x10;
        for row in 0..area.height {
            let mut line = Line::default();

            let offset = mem_per_line * row;
            line.spans.push(Span::raw(format!("{offset:08X}: ")));

            for byte_offset in 0..mem_per_line {
                let address = (offset + byte_offset).into();
                let byte = self.cpu.bus.read(address).0 as u8;

                let byte_style = match (address / Word(4)) == (self.cpu.reg.read("pc").unwrap() / Word(4)) {
                    true    => Style::default().fg(Color::LightCyan),
                    false   => Style::default(),
                };

                line.spans.push(Span::styled(format!("{byte:02X} "), byte_style));
            }
            
            buf.set_line(area.x, area.y + row, &line, area.width);
        }
    }
}