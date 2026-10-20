use chip8_core::{DISPLAY_HEIGHT, DISPLAY_WIDTH, pixel_count};
use chip8_core::{MEMORY_SIZE, PROGRAM_START};
fn main() {
    println!("CHIP-8 emulator starting...");
    println!(
        "Screen: {}x{} pixels = {} pixels",
        DISPLAY_WIDTH,
        DISPLAY_HEIGHT,
        pixel_count()
    );
    println!(
        "Memory: {} bytes, Program start:{:#X}",
        MEMORY_SIZE, PROGRAM_START
    );
}
