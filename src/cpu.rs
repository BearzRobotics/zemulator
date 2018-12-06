use crate::bus::Bus;
use std::fmt;
use rand;
use rand::distributions::{IndependentSample, Range};

pub const PROGRAM_START: u16 = 0x1F; // Maybe

/// This arch has 5 registors
/// aluout, = read only.
/// bus,    = the comptuer bus
/// A,      = General registor
/// B,      = General registor
/// C,      = General registor
pub struct Cpu{
    vx:         [u8; 3], // number of general regesters
    pc:         u16,
    alu:        u8,
    ret_stack:  Vec<u16>,
    rng:        rand::ThreadRng,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu{
            vx: [0; 3], // vx[1] = a, vx[2] = b, vx[3] = c
            pc: PROGRAM_START,
            alu: 0,
            ret_stack: Vec::<u16>::new(),
            rng: rand::thread_rng(),
        }

    }

    pub fn run_instruction(&mut self, bus: &mut Bus) {
        let hi = bus.ram_read_byte(self.pc) as u16;
        let lo = bus.ram_read_byte(self.pc + 1) as u16;
        let instruction: u16 = (hi << 8) | lo;

        println!("Intruction Read: {:#X}: lo:{:#X} hi:{:#X}", instruction, lo, hi );

        
        match (instruction & 0xF000) >> 12 {
            0x00 => {
                // HLT 

            }
            0x01 => {
                //CLRR

            }
            0x02 => {
                //STRA
                
            }
            0x03 => {
                //STRB
                
            }
            0x04 => {
                //PSSA
                
            }
            0x05 => {
                //PSSB
                
            }
            0x06 => {
                //JMP
                
            }
            0x07 => {
                //JMPC
                
            }
            0x08 => {
                //JMPZ

            }
            0x09 => {
                //JMPF

            }
            0x0a => {
                //NOTA

            }
            0x0b => {
                //NOTB
                           
            }
            0x0c => {
                //AND    
            }
            0x0d => {
                //OR
                
            }
            0x0e => {
                //XOR
                
            }
            0x0f => {
                //ADD

            }
            0x10 => {
                //SUB

            }
            0x11 => {
                //ROL

            }
            0x12 => {
                //ROR

            }
            0x13 => {
                //ZRO

            }
            0x14 => {
                //ALL

            }
            0x15 => {
                //ONE

            }
            0x16 => {
                //SDTS

            }
            0x17 => {
                //SRMS

            }
            0x18 => {
                //SALS

            }
            0x19 => {
                //STRC

            }
            0x1a => {
                //JMPH

            }
            0x1b => {
                //STRM

            }
            0x1c => {
                //RDRM
                
            }
            0x1d => {
                //OUT

            }
            0x1e => {
                //INP

            }
            0x1f => {
                //HLD

            }
            _ => {
                // this will panic if it catches an instruction not
                // delt with above.
                panic!("Not a valid instruction");
            }

        }
    }

    // arch has no IO functions yet.
    fn debug_draw_sprite() {

    }

    pub fn write_reg_vx(&mut self, index: u8, value: u8){
        self.vx[index as usize] = value;
    }

    pub fn read_reg_vx(&mut self, index: u8) -> u8{
        self.vx[index as usize]
    }

    pub fn print_reg_values(&mut self, bus: &mut Bus) {
        println!("Alu Out: {}", self.alu);
        println!("Bus: {}", bus.get_reg_bus());
        println!("RegA: {}", self.vx[0]);
        println!("RegB: {}", self.vx[1]);
        println!("RegC: {}", self.vx[2]);
    }
}