use crate::Word;
use super::DeviceTrait;
use std::cell::Cell;

pub struct Ram64KiB(Vec<Cell<u8>>);
impl Ram64KiB {
    pub fn new() -> Self { 
        Self(vec![Cell::new(0); 1024 * 64]) 
    }

    pub fn load(&mut self, data: &[u8]) {
        assert!(data.len() >= 1024 * 64, "Failed to load RAM; data can have only 64 KiB of size");

        for (idx, byte) in data.iter().enumerate() {
            self.0[idx].set(*byte);
        }
    }
}
impl DeviceTrait for Ram64KiB {
    fn name(&self) -> String {
        "RAM 64 KiB".into()
    }
    fn read(&self, address: Word) -> Word {
        let ram_address = (address & Word(0xFFFF)).0 as usize;
        return Word(self.0[ram_address].get() as u32);
    }
    fn write(&self, address: Word, word: Word) {
        let ram_address = (address & Word(0xFFFF)).0 as usize;
        self.0[ram_address].set((word & Word(0xFF)).0 as u8);
    }
    fn tick(&self) {
        // Do nothing
    }
}
