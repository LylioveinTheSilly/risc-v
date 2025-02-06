use machine_viewer::app::{App, AppResult};
use machine_viewer::event::{Event, EventHandler};
use machine_viewer::handler::handle_key_events;
use machine_viewer::tui::Tui;
use ratatui::backend::CrosstermBackend;
use risc_v::{RV32, Word};
use ratatui::Terminal;
use std::io;

const PROGRAM_ROM: &[u8] = include_bytes!("rom.bin");

fn main() -> AppResult<()> {
    let cpu = RV32::new();
    cpu.bus.load(Word(0), PROGRAM_ROM);

    // Create an application.
    let mut app = App::new(cpu);

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
