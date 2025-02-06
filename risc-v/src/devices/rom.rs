use crate::Word;
use super::DeviceTrait;

pub struct Rom64KiB([u8; 1024 * 64]);
impl Rom64KiB {
    pub fn new() -> Self { 
        Self([0; 1024 * 64]) 
    }

    pub fn load(&mut self, data: &[u8]) {
        assert!(data.len() < 1024 * 64, "Failed to load ROM; data can have only 64 KiB of size, data size: {} bytes", data.len());

        for (idx, byte) in data.iter().enumerate() {
            self.0[idx] = *byte;
        }
    }

    pub fn from_bytes(data: &[u8]) -> Self {
        let mut rom = Self::new();
        rom.load(data);
        rom
    }
}
impl DeviceTrait for Rom64KiB {
    fn name(&self) -> String {
        "ROM 64 KiB".into()
    }
    fn read(&self, address: Word) -> Word {
        let rom_address = (address & Word(0xFFFF)).0 as usize;
        return Word(self.0[rom_address] as u32);
    }
    fn write(&self, _: Word, _: Word) {
        // ROM is not writable
    }
    fn tick(&self) {
        // Do nothing
    }
}