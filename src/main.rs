//use minifb::{Key, KeyRepeat, Window, WindowOptions};
use std::fs::File;
use std::io::Read;
use std::time::{Duration, Instant};
use std::env;

mod cpu;
mod ram;
mod keyboard;
mod chip;
mod bus;
mod display;

use crate::chip::Chip;

// let set up are screen size
//const WIDTH : usize = 640;
//const HEIGHT: usize = 320;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = match args.len() {
        0 | 1 => "/plex/Nextcloud/code/rust/zemulator/Data/add",
        _ => args.get(1).unwrap(),
    };
    let mut file = File::open(file_name).unwrap();
    let mut data = Vec::<u8>::new();
    file.read_to_end(&mut data).expect("No file found");

    //ARGB buffer
    //let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    /*
    let mut window = Window::new(
        "Bearz Emulator of Zachs instruction set",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("Widnow Create failed: {:?}", e);
    });
    */

    let mut chip = Chip::new();
    chip.load_rom(&data);

    loop{
        chip.run_instruction();
        //chip.print_reg_values();
    }
    // this is here for debuging
    println!("Loop has ened");
}
