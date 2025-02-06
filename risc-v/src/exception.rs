
#[derive(Debug, Clone, Copy)]
pub enum Exception {
    InvalidInstruction,
    InvalidRegister,
    MisalignedAddress,
    EnvironmentCall,
    EnvironmentBreak,
}