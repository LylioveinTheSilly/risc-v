use risc_v::{RV32, Word};
use risc_v::register::{RV32IRegisters, Register};
use ratatui::prelude::*;
use ratatui::widgets::{Widget, StatefulWidget, Block};
use super::ValueFormat;

const VALUE_MAX_WIDTH: u16 = 10;
const ALIAS_MAX_WIDTH: u16 = 4;

#[derive(Debug, Clone)]
struct RegisterState {
    pub changed: bool,
    pub register: Register,
}
impl RegisterState {
    pub fn name(&self) -> &String {
        &self.register.aliases[0]
    }
    pub fn aliases(&self) -> &Vec<String> {
        &self.register.aliases
    }
    pub fn value(&self) -> Word {
        self.register.read()
    }
}

pub struct RegisterViewState {
    pub stepped: bool,
    pub current_registers: RV32IRegisters,
    pub registers_snapshot: RV32IRegisters,
}
impl RegisterViewState {
    pub fn reset(&mut self, cpu: &RV32) {
        self.current_registers = cpu.reg.clone();
        self.registers_snapshot = cpu.reg.clone();
        self.stepped = false;
    }
    
    /// Return differences between current and snapshotted registers. Returns `Ok()` with differences 
    /// and `Err(&str)` if any of the supplied register aliases wasn't found.
    pub(self) fn difference<'a>(&mut self, cpu: &RV32, registers: &Vec<String>) -> Result<Vec<RegisterState>, String> {
        if self.stepped == true {
            self.registers_snapshot = self.current_registers.clone();
            self.current_registers = cpu.reg.clone();
            self.stepped = false;
        }

        let mut diffs = vec![];

        for reg_alias in registers {
            let old_reg_value = self.registers_snapshot.read(reg_alias).or(Err(reg_alias));
            let new_reg_value = self.current_registers.read(reg_alias).or(Err(reg_alias));

            diffs.push(RegisterState {
                changed: old_reg_value != new_reg_value, 
                register: cpu.reg
                    .get(&reg_alias)
                    .ok_or(reg_alias)?
                    .clone()
            })
        }

        Ok(diffs)
    }
}
impl RegisterViewState {
    pub fn new(cpu: &RV32) -> Self {
        Self {
            stepped: false,
            current_registers: cpu.reg.clone(),
            registers_snapshot: cpu.reg.clone(),
        }
    }
}

pub struct RegisterView<'a> {
    cpu: &'a RV32,
    block: Option<Block<'a>>,
    registers: Vec<String>,
    direction: Direction,
    display_values: ValueFormat,
    display_aliases: bool,
    align: Alignment,
}
impl<'a> RegisterView<'a> {
    pub fn new(cpu: &'a RV32) -> Self {
        Self { 
            cpu, block: None, registers: vec![], 
            display_values: ValueFormat::default(), 
            direction: Direction::Horizontal,
            display_aliases: true, align: Alignment::Left,
        }
    }
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }
    pub fn align(mut self, align: Alignment) -> Self {
        self.align = align;
        self
    }
    pub fn registers(mut self, registers: Vec<String>) -> Self {
        self.registers = registers;
        self
    }
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }
    pub fn display_values(mut self, display_values: ValueFormat) -> Self {
        self.display_values = display_values;
        self
    }
    pub fn display_aliases(mut self, display_aliases: bool) -> Self {
        self.display_aliases = display_aliases;
        self
    }
}
impl<'a> StatefulWidget for RegisterView<'a> {
    type State = RegisterViewState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        if self.registers.is_empty() {
            return;
        }

        let mut area = match self.block.clone() {
            Some(block) => {
                let mut inner_area = block.inner(area);
                block.render(area, buf);
                inner_area
            },
            None => area,
        };

        let register_widgets: Vec<RegisterWidget> = state
            .difference(self.cpu, &self.registers)
            .unwrap()
            .into_iter()
            .map(|s| RegisterWidget::new(s)
                .display_value(self.display_values)
                .display_alias(self.display_aliases)
            )
            .collect();

        // Calculate areas for each widgets
        // All widgets have the same size
        let widget_width = register_widgets[0].width();

        // Calculate the maximum amount of registers that can be displayed on the screen
        // along the main and cross axises; terminology (main and cross axis) 
        // taken from CSS flexbox layout model.
        let (main_axis_count, cross_axis_count) = match self.direction {
            Direction::Horizontal => {
                //   ╔══ main ═══┄┄┄
                //   ║
                //  cross
                //   ║
                //   ┊

                let main  = area.width / widget_width;
                let cross = area.height;
                (main, cross)
            },
            Direction::Vertical => {
                //   ╔══ cross ═══┄┄┄
                //   ║
                //  main
                //   ║
                //   ┊

                let main  = area.height;
                let cross = area.width / widget_width;
                (main, cross)
            }
        };
        
    }

    
}

#[derive(Debug, Clone)]
struct RegisterWidget {
    register: RegisterState,
    display_value: ValueFormat,
    display_alias: bool,
    value_max_width: u16,
    alias_max_width: u16,
}
impl RegisterWidget {
    pub fn new(register: RegisterState) -> Self {
        Self { 
            register,
            display_value: ValueFormat::default(),
            display_alias: true,
            value_max_width: VALUE_MAX_WIDTH,
            alias_max_width: ALIAS_MAX_WIDTH,
        }
    }
    pub fn width(&self) -> u16 {
        self.alias_max_width + 2 + self.value_max_width
    }
    pub fn display_value(mut self, display_value: ValueFormat) -> Self {
        self.display_value = display_value;
        self
    }
    pub fn display_alias(mut self, display_alias: bool) -> Self {
        self.display_alias = display_alias;
        self
    }
}
impl Widget for RegisterWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use ValueFormat::{DecimalSigned, DecimalUnsigned, Hexadecimal};
        
        let reg_name = match self.register.aliases().iter().next() {
            Some(alias) => format!("{0:.1$}: ", alias, self.alias_max_width as usize),
            None => format!("{0:.1$}: ", self.register.name(), self.alias_max_width as usize)
        };

        // Offset from the beginnig of the area to where value will be drawn
        let value_offset = self.alias_max_width as usize + 2;
        let reg_name = format!("{reg_name:<0$}", value_offset);

        let value = match self.display_value {
            DecimalSigned => format!("{}", self.register.register.read().signed()),
            DecimalUnsigned => format!("{}", self.register.register.read().unsigned()),
            Hexadecimal => format!("{:08X}", self.register.register.read().unsigned()),
        };
        let value = format!("{value:.0$}", self.value_max_width as usize);


        let value_style = match self.register.changed {
            true => Style::default().light_red(),
            false => Style::default(),
        };

        let line = Line::from(vec![
            Span::raw(reg_name), Span::styled(value, value_style)  
        ]);

        buf.set_line(area.x, area.y, &line, self.width());
    }
}