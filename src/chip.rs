// this is implentation of chip8 instruction set
use crate::ram::Ram;
use crate::cpu::Cpu;
use crate::bus::Bus;
pub struct Chip {
    ram: Ram,
    bus: Bus,
    cpu: Cpu,
}

impl Chip {
    pub fn new() -> Chip {
        Chip {
            ram: Ram::new(),
            bus: Bus::new(),
            cpu: Cpu::new(),
        }
    }

    pub fn load_rom(&mut self, data: &Vec<u8>) {
        let offset = 0x200; // does he have an off set if so it needs to be offset + i below
        for i in 0..data.len() {
            self.ram.write_byte((offset + i) as u16, data[i])
        }
    }

    pub fn run_instruction(&mut self) {

    }

   pub fn get_display_buffer(&self) -> &[u8] {
       self.bus.get_display_buffer()
   }

   pub fn set_key_pressed(&mut self, key: Option<u8>) {
       self.bus.set_key_pressed(key);
   }
}