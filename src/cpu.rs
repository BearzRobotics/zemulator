use crate::bus::Bus;
use std::fmt;
use rand;
use rand::distributions::{IndependentSample, Range};

pub const PROGRAM_START: u16 = 0x200; // Maybe

pub struct Cpu{
    vx:         [u8; 4], // number of regesters
    pc:         u16,
    i:          u16,
    ret_stack:  Vec<u16>,
    rng:        rand::ThreadRng,
}
impl Cpu {
    pub fn new() -> Cpu {
        Cpu{
            vx: [0; 5],
            pc: PROGRAM_START,
            i: 0,
            ret_stack: Vec::<u16>::new(),
            rng: rand::thread_rng(),
        }

    }

    pub fn run_instruction(&mut self, bus: &mut Bus) {
        let hi = bus.ram_read_byte(self.pc) as u16;
        let lo = bus.ram_read_byte(self.pc + 1) as u16;
        let instruction: u16 = (hi << 8) | lo;
    }

    fn debug_draw_sprite() {

    }

    pub fn write_reg_vx(){

    }

    pub fn read_reg_vx(){

    }
}