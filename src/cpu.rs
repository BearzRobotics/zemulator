use crate::bus::Bus;
use std::fmt;
use rand;
use rand::distributions::{IndependentSample, Range};

pub const PROGRAM_START: u16 = 0x200; // Maybe

pub struct Cpu{
    vx:         [u8; 5], // number of regesters
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

        match (instruction & 0xF000) >> 12 {
            0x00 => {

            }
            0x01 => {

            }
            0x02 => {
                
            }
            0x03 => {
                
            }
            0x04 => {
                
            }
            0x05 => {
                
            }
            0x06 => {
                
            }
            0x07 => {
                
            }
            0x08 => {
                
            }
            0x09 => {

            }
            0x0a => {

            }
            0x0b => {
                
            }
            0x0c => {
                
            }
            0x0d => {
                
            }
            0x0e => {
                
            }
            0x0f => {
                
            }
            0x10 => {
                
            }
            0x11 => {
                
            }
            0x12 => {

            }
            0x13 => {

            }
            0x14 => {
                
            }
            0x15 => {
                
            }
            0x16 => {
                
            }
            0x17 => {
                
            }
            0x18 => {
                
            }
            0x19 => {
                
            }
            0x1a => {
                
            }
            0x1b => {
                
            }
            0x1c => {
                
            }
            0x1d => {
                
            }
            0x1e => {
                
            }
            0x1f => {
                
            }
            _ => {
                panic!("Not a valid instruction");
            }

        }
    }

    fn debug_draw_sprite() {

    }

    pub fn write_reg_vx(){

    }

    pub fn read_reg_vx(){

    }
}