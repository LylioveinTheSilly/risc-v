pub mod rom;
pub mod ram;

#[cfg(feature = "multimedia")]
pub mod multimedia;

use crate::{Word, MemoryRange};

pub struct Device {
    pub range: MemoryRange,
    pub device: Box<dyn DeviceTrait>,
}
impl Device {
    pub fn new(range: MemoryRange, device: Box<dyn DeviceTrait>) -> Self {
        Self { range, device }
    }
    pub fn read(&self, address: Word) -> Word {
        self.device.read(address)
    }
    pub fn write(&self, address: Word, word: Word) {
        self.device.write(address, word);
    }
}

pub trait DeviceTrait {
    fn name(&self) -> String;
    fn read(&self, address: Word) -> Word;
    fn write(&self, address: Word, word: Word);
    fn tick(&self);
}