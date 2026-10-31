//! The CHIP-8 emulator core. Knows nothing about windows, keyboards, or files.

pub mod font;
pub mod opcode;

mod machine;
pub use machine::{Chip8, LoadError};

/// Width of the CHIP-8 display in pixels.
pub const DISPLAY_WIDTH: usize = 64;

/// Height of the CHIP-8 display in pixels.
pub const DISPLAY_HEIGHT: usize = 32;

/// Total number of pixels in the CHIP-8 display
pub fn pixel_count() -> usize {
    DISPLAY_WIDTH * DISPLAY_HEIGHT
}

/// CHIP-8 has 4KB of memory
pub const MEMORY_SIZE: usize = 4096;

/// Address where CHIP-8 programs are loaded into memory.
pub const PROGRAM_START: usize = 0x200;

/// Largest ROM that fits between `PROGRAM_START` and the end of memory.
pub const MAX_ROM_SIZE: usize = MEMORY_SIZE - PROGRAM_START;

/// Number of general-purpose registers (V0–VF).
pub const REGISTER_COUNT: usize = 16;

/// Maximum depth of nested subroutine calls.
pub const STACK_SIZE: usize = 16;

/// Number of keys on the CHIP-8 keypad (0–F).
pub const KEY_COUNT: usize = 16;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_has_2048_pixels() {
        assert_eq!(pixel_count(), 2048);
    }
    #[test]
    fn program_start_is_inside_memory() {
        const { assert!(PROGRAM_START < MEMORY_SIZE) };
    }
    #[test]
    fn max_rom_size_is_3584() {
        assert_eq!(MAX_ROM_SIZE, 3584);
    }
}
