use risc_v::Word;
use risc_v::bus::DeviceTrait;

use std::{io::{stdout, Stdout}, thread, time::Duration};
use crossterm::{
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::*,
    widgets::{canvas::*, *},
};

pub struct TermScreen {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}
impl TermScreen {
    pub fn new() -> Self {
        enable_raw_mode().unwrap();
        stdout().execute(EnterAlternateScreen).unwrap();
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout())).unwrap();
        
        terminal.draw(|frame| {
            frame.render_widget(Canvas::default()
                .block(Block::default().title("Canvas").borders(Borders::ALL))
                .x_bounds([-180.0, 180.0])
                .y_bounds([-90.0, 90.0])
                .paint(|ctx| {
                    ctx.draw(&Map {
                        resolution: MapResolution::High,
                        color: Color::White,
                    });
                    ctx.layer();
                    ctx.draw(&Rectangle {
                        x: 10.0,
                        y: 20.0,
                        width: 10.0,
                        height: 10.0,
                        color: Color::Red,
                    }), frame.size())
            }, frame.size()s) 
        }).unwrap();

        thread::sleep(Duration::from_millis(3000));
        Self { terminal }
    }
    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
    }
}
impl Drop for TermScreen {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        stdout().execute(LeaveAlternateScreen).unwrap();
    }
}
impl DeviceTrait for TermScreen {
    fn name(&self) -> String {
        "TermScreen".to_string()
    }
    fn read(&self, address: Word) -> Word { Word(0) }
    fn write(&self, address: Word, word: Word) {}
    fn tick(&self) {}
}