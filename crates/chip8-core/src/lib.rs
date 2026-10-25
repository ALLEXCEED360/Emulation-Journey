//! The CHIP-8 emulator core. Knows nothing about windows, keyboards, or files.

pub mod opcode;

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
pub const PROGRAM_START: usize = 0x200;

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
}
