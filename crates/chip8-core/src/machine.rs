//! The CHIP-8 machine: all of its state, and loading programs into it.

use std::fmt;

use crate::font::{FONT, FONT_START};
use crate::{
    DISPLAY_HEIGHT, DISPLAY_WIDTH, KEY_COUNT, MAX_ROM_SIZE, MEMORY_SIZE, PROGRAM_START,
    REGISTER_COUNT, STACK_SIZE,
};

/// The complete state of a CHIP-8 machine.
#[derive(Debug, Clone)]
pub struct Chip8 {
    /// 4 KB of RAM. Font at 0x050, programs from 0x200.
    memory: [u8; MEMORY_SIZE],
    /// General-purpose registers V0–VF. VF doubles as the flag register.
    v: [u8; REGISTER_COUNT],
    /// Index register: points at memory (sprites, fonts, load/store).
    i: u16,
    /// Program counter: address of the next instruction.
    pc: u16,
    /// Return addresses for subroutine calls.
    stack: [u16; STACK_SIZE],
    /// Stack pointer: how many entries of `stack` are in use.
    sp: usize,
    /// Counts down at 60 Hz while above zero.
    delay_timer: u8,
    /// Counts down at 60 Hz; the beeper sounds while above zero.
    sound_timer: u8,
    /// 64×32 pixels, row by row. `true` = pixel on.
    display: [bool; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    /// Which of the 16 keys are held down.
    keys: [bool; KEY_COUNT],
}

impl Chip8 {
    /// Creates a freshly powered-on machine: font loaded, everything else zero, PC at 0x200.
    pub fn new() -> Self {
        let mut memory = [0; MEMORY_SIZE];
        memory[FONT_START..FONT_START + FONT.len()].copy_from_slice(&FONT);

        Self {
            memory,
            v: [0; REGISTER_COUNT],
            i: 0,
            pc: PROGRAM_START as u16,
            stack: [0; STACK_SIZE],
            sp: 0,
            delay_timer: 0,
            sound_timer: 0,
            display: [false; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            keys: [false; KEY_COUNT],
        }
    }

    /// Copies a program into memory, starting at 0x200.
    pub fn load_rom(&mut self, rom: &[u8]) -> Result<(), LoadError> {
        if rom.is_empty() {
            return Err(LoadError::Empty);
        }
        if rom.len() > MAX_ROM_SIZE {
            return Err(LoadError::TooLarge {
                size: rom.len(),
                max: MAX_ROM_SIZE,
            });
        }

        let end = PROGRAM_START + rom.len();
        self.memory[PROGRAM_START..end].copy_from_slice(rom);
        Ok(())
    }

    /// Address of the next instruction.
    pub fn pc(&self) -> u16 {
        self.pc
    }

    /// The index register.
    pub fn i(&self) -> u16 {
        self.i
    }

    /// Registers V0–VF.
    pub fn registers(&self) -> &[u8; REGISTER_COUNT] {
        &self.v
    }

    /// All 4096 bytes of memory (read-only).
    pub fn memory(&self) -> &[u8] {
        &self.memory
    }

    /// Return addresses currently on the stack, oldest first.
    pub fn stack(&self) -> &[u16] {
        &self.stack[..self.sp]
    }

    /// Current value of the delay timer.
    pub fn delay_timer(&self) -> u8 {
        self.delay_timer
    }

    /// Current value of the sound timer.
    pub fn sound_timer(&self) -> u8 {
        self.sound_timer
    }

    /// The 64×32 screen, row by row. `true` = pixel on.
    pub fn display(&self) -> &[bool] {
        &self.display
    }

    /// Whether key `0x0`–`0xF` is held down. Panics if `key` is 16 or more.
    pub fn is_key_pressed(&self, key: usize) -> bool {
        self.keys[key]
    }
}

impl Default for Chip8 {
    fn default() -> Self {
        Self::new()
    }
}

/// Why a ROM couldn't be loaded.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LoadError {
    /// The ROM has no bytes at all.
    Empty,
    /// The ROM doesn't fit between 0x200 and the end of memory.
    TooLarge { size: usize, max: usize },
}

impl fmt::Display for LoadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LoadError::Empty => write!(f, "ROM file is empty"),
            LoadError::TooLarge { size, max } => {
                write!(f, "ROM is {size} bytes, but the maximum is {max} bytes")
            }
        }
    }
}

impl std::error::Error for LoadError {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::font::BYTES_PER_CHAR;

    #[test]
    fn new_machine_has_documented_starting_state() {
        let chip8 = Chip8::new();
        assert_eq!(chip8.pc(), 0x200);
        assert_eq!(chip8.i(), 0);
        assert_eq!(chip8.registers(), &[0; REGISTER_COUNT]);
        assert!(chip8.stack().is_empty());
        assert_eq!(chip8.delay_timer(), 0);
        assert_eq!(chip8.sound_timer(), 0);
        assert!(!chip8.display().contains(&true));
        for key in 0..KEY_COUNT {
            assert!(!chip8.is_key_pressed(key));
        }
    }

    #[test]
    fn font_is_loaded_at_font_start() {
        let chip8 = Chip8::new();
        assert_eq!(
            &chip8.memory()[FONT_START..FONT_START + FONT.len()],
            &FONT[..]
        );
    }

    #[test]
    fn load_rom_copies_bytes_to_program_start() {
        let mut chip8 = Chip8::new();
        let rom = [0x00, 0xE0, 0x12, 0x00];

        assert_eq!(chip8.load_rom(&rom), Ok(()));
        assert_eq!(&chip8.memory()[PROGRAM_START..PROGRAM_START + 4], &rom[..]);
        assert_eq!(chip8.memory()[PROGRAM_START + 4], 0);
    }

    #[test]
    fn load_rom_rejects_an_empty_rom() {
        let mut chip8 = Chip8::new();
        assert_eq!(chip8.load_rom(&[]), Err(LoadError::Empty));
    }

    #[test]
    fn load_rom_accepts_the_largest_possible_rom() {
        let mut chip8 = Chip8::new();
        let rom = [0xAB; MAX_ROM_SIZE];

        assert_eq!(chip8.load_rom(&rom), Ok(()));
        assert_eq!(chip8.memory()[MEMORY_SIZE - 1], 0xAB);
    }

    #[test]
    fn load_rom_rejects_rom_one_byte_too_large() {
        let mut chip8 = Chip8::new();
        let rom = [0; MAX_ROM_SIZE + 1];

        assert_eq!(
            chip8.load_rom(&rom),
            Err(LoadError::TooLarge {
                size: 3585,
                max: 3584,
            })
        );
    }

    #[test]
    fn load_error_messages() {
        assert_eq!(LoadError::Empty.to_string(), "ROM file is empty");
        assert_eq!(
            LoadError::TooLarge {
                size: 3585,
                max: 3584,
            }
            .to_string(),
            "ROM is 3585 bytes, but the maximum is 3584 bytes"
        );
    }

    #[test]
    fn letter_a_is_in_memory() {
        let chip8 = Chip8::new();
        // A is character 10: 0x050 + 10 × 5 = 0x050 + 0x032 = 0x082
        let a_start = FONT_START + 0xA * BYTES_PER_CHAR;

        assert_eq!(a_start, 0x082);
        assert_eq!(
            &chip8.memory()[a_start..a_start + BYTES_PER_CHAR],
            &[0xF0, 0x90, 0xF0, 0x90, 0x90]
        );
    }
}
