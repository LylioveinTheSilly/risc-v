use risc_v::Word;
use risc_v::bus::{Device, MemoryRange};
use device::TermScreen;
mod device;

fn main() {
    let mut cpu = risc_v::RV32::new();
    cpu.bus.connect(Device::new(
        MemoryRange::new(Word(0x8000), Word(0x1000)).unwrap(),
        TermScreen::new().boxed()
    ));
    
    println!("Hello, world!");
}
