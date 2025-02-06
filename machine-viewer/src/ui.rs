use ratatui::{
    layout::{Layout, Direction, Constraint},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::{app::App, widgets::memory_view};
use crate::widgets::{InstructionView, RegisterView, MemoryView, ValueFormat};

pub fn render(app: &mut App, frame: &mut Frame) {
    use Constraint::Percentage;

    let layout = Layout::default()
        .margin(1)
        .direction(Direction::Horizontal)
        .constraints([
            Percentage(30),
            Percentage(70)
        ])
        .split(frame.size());

    let (instruction_view_block, memory_register_blocks) = (layout[0], layout[1]);

    let block = Block::default()
        .title(" Instructions ")
        .border_type(BorderType::Plain)
        .borders(Borders::all());

    let instruction_view = InstructionView::new(&app.cpu)
        .block(block)
        .display_immediates(ValueFormat::DecimalSigned);
    frame.render_widget(instruction_view, instruction_view_block);

    let memory_register_space = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(memory_register_blocks);
    let (register_space, memory_space) = (memory_register_space[0], memory_register_space[1]);

    let register_block = Block::default()
        .title(" Registers ")
        .border_type(BorderType::Plain)
        .borders(Borders::all());
    let register_view = RegisterView::new(&app.cpu)
        .block(register_block)
        .display_values(ValueFormat::DecimalSigned)
        .display_aliases(true);
    frame.render_stateful_widget(
        register_view, register_space, 
        &mut app.register_view_state
    );

    let memory_block = Block::default()
        .title(" Memory ")
        .border_type(BorderType::Plain)
        .borders(Borders::all());
    let memory_view = MemoryView::new(&app.cpu)
        .block(memory_block);
    frame.render_widget(memory_view, memory_space);
}