use std::error::Error;
use std::{env, fs, process};

use chip8_core::Chip8;

fn main() {
    if let Err(error) = run() {
        eprintln!("Error: {error}");
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let path = env::args()
        .nth(1)
        .ok_or("usage: chip8-desktop <rom-file>")?;
    let rom = fs::read(&path)?;

    let mut chip8 = Chip8::new();
    chip8.load_rom(&rom)?;

    println!("Loaded {path} ({} bytes)", rom.len());
    print!("First bytes:");
    for byte in rom.iter().take(8) {
        print!(" {byte:02X}");
    }
    println!();
    println!("PC starts at {:#05X}", chip8.pc());

    Ok(())
}
