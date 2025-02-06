use crate::Word;
use crate::devices::Device;

pub struct Bus {
    devices: Vec<Device>,
}
impl Bus {
    pub fn new() -> Self {
        Self { devices: vec![] }
    }

    pub fn connect(&mut self, device: Device) -> Result<(), ()> {
        for dev in self.devices.iter() {
            if dev.range.intersects(device.range) {
                return Err(());
            }
        }

        self.devices.push(device);
        Ok(())
    } 

    pub fn read(&self, address: Word) -> Word {
        for device in self.devices.iter() {
            if device.range.contains(address) {
                return device.device.read(address);
            }
        }

        // FIXME: What happens when CPU reads open bus?
        Word(0)
    }

    pub fn write(&self, address: Word, word: Word) {
        for device in self.devices.iter() {
            if device.range.contains(address) {
                device.device.write(address, word);
            }
        }
    }

    pub fn load(&self, offset: Word, data: &[u8]) {
        for (i, byte) in data.iter().enumerate() {
            self.write(offset + Word(i as u32), Word(*byte as u32));
        }
    }

    pub fn read_le_word(&self, offset: Word) -> Word {
        let (b1, b2, b3, b4) = (
            self.read(offset + Word(0)),
            self.read(offset + Word(1)),
            self.read(offset + Word(2)),
            self.read(offset + Word(3)),
        );
    
        let le_word = Word::from_le_bytes([
            b1.0 as u8, 
            b2.0 as u8, 
            b3.0 as u8, 
            b4.0 as u8
        ]);

        le_word
    }

    pub fn tick(&self) {
        self.devices.iter().for_each(|d| d.device.tick());
    }
}