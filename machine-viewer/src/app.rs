use std::error;
use risc_v::RV32;
use crate::widgets::RegisterViewState;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
pub struct App {
    pub running: bool,
    pub cpu: RV32,
    pub register_view_state: RegisterViewState,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new(cpu: RV32) -> Self {
        let register_view_state = RegisterViewState::new(&cpu);
        Self { running: true, cpu, register_view_state }
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Step the CPU
    pub fn step_cpu(&mut self) {
        self.cpu.step();
        self.register_view_state.stepped = true;
    }

    /// Run the CPU until exception is hit
    pub fn run_cpu(&mut self) {
        loop {
            match self.cpu.step() {
                Ok(_) => (),
                Err(_) => break,
            }

        }
         
        self.register_view_state.reset(&self.cpu);
    }

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.running = false;
    }
}
