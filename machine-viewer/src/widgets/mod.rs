pub mod instruction_view;
pub use instruction_view::InstructionView;

pub mod register_view;
pub use register_view::{RegisterView, RegisterViewState};

pub mod memory_view;
pub use memory_view::MemoryView;

#[derive(Debug, Clone, Copy, Default)]
pub enum ValueFormat {
    #[default]
    DecimalSigned,
    DecimalUnsigned,
    Hexadecimal,
}