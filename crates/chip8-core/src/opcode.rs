//! Helpers for pulling apart 16-bit CHIP-8 opcodes.
//!
//! Every instruction is 2 bytes = 4 hex digits (nibbles):
//!
//! ```text
//!   0xD123
//!     │││└─ N    (lowest nibble)
//!     ││└── Y
//!     │└─── X
//!     └──── family
//!   NN  = low byte      (0x23)
//!   NNN = low 3 nibbles (0x123)
//! ```

/// Combines two bytes into one 16-bit value, high byte first (big-endian).
pub fn combine_bytes(high: u8, low: u8) -> u16 {
    (u16::from(high) << 8) | u16::from(low)
}

/// Splits a 16-bit value into `(high byte, low byte)`.
pub fn split_u16(value: u16) -> (u8, u8) {
    ((value >> 8) as u8, (value & 0x00FF) as u8)
}

/// The upper 4 bits of a byte: `0xAB` → `0xA`.
pub fn high_nibble(byte: u8) -> u8 {
    byte >> 4
}

/// The lower 4 bits of a byte: `0xAB` → `0xB`.
pub fn low_nibble(byte: u8) -> u8 {
    byte & 0x0F
}

/// A raw 16-bit CHIP-8 instruction, with helpers to read each field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Opcode(pub u16);

impl Opcode {
    /// Top nibble: which instruction family this is (0x0–0xF).
    pub fn family(self) -> u8 {
        (self.0 >> 12) as u8
    }

    /// Second nibble: register index X (0–15).
    /// Returned as `usize` because it's used to index the register array.
    pub fn x(self) -> usize {
        ((self.0 & 0x0F00) >> 8) as usize
    }

    /// Third nibble: register index Y (0–15).
    /// Returned as `usize` because it's used to index the register array.
    pub fn y(self) -> usize {
        ((self.0 & 0x00F0) >> 4) as usize
    }

    /// Lowest nibble: a 4-bit number (0–15), e.g. sprite height in `DXYN`.
    pub fn n(self) -> u8 {
        (self.0 & 0x000F) as u8
    }

    /// Low byte: an 8-bit number (0–255), e.g. the value in `6XNN`.
    pub fn nn(self) -> u8 {
        (self.0 & 0x00FF) as u8
    }

    /// Low 12 bits: a memory address (0x000–0xFFF), e.g. the target in `1NNN`.
    pub fn nnn(self) -> u16 {
        self.0 & 0x0FFF
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn combine_bytes_puts_high_byte_first() {
        assert_eq!(combine_bytes(0xD1, 0x23), 0xD123);
    }

    #[test]
    fn split_u16_returns_high_then_low() {
        assert_eq!(split_u16(0xD123), (0xD1, 0x23));
    }

    #[test]
    fn combine_and_split_round_trip() {
        for value in [0x0000, 0x00FF, 0xFF00, 0xABCD, 0xFFFF] {
            let (high, low) = split_u16(value);
            assert_eq!(combine_bytes(high, low), value);
        }
    }

    #[test]
    fn nibbles_of_a_byte() {
        assert_eq!(high_nibble(0xAB), 0xA);
        assert_eq!(low_nibble(0xAB), 0xB);
    }

    #[test]
    fn opcode_family_and_x() {
        let op = Opcode(0xD123);
        assert_eq!(op.family(), 0xD);
        assert_eq!(op.x(), 1);
    }

    #[test]
    fn opcode_fields_of_d123() {
        let op = Opcode(0xD123);

        assert_eq!(op.family(), 0xD);
        assert_eq!(op.x(), 1);
        assert_eq!(op.y(), 2);
        assert_eq!(op.n(), 3);
        assert_eq!(op.nn(), 0x23);
        assert_eq!(op.nnn(), 0x123);
    }

    #[test]
    fn opcode_fields_of_8ab4() {
        let op = Opcode(0x8AB4);

        assert_eq!(op.family(), 0x8);
        assert_eq!(op.x(), 0xA);
        assert_eq!(op.y(), 0xB);
        assert_eq!(op.n(), 4);
        assert_eq!(op.nn(), 0xB4);
        assert_eq!(op.nnn(), 0xAB4);
    }

    #[test]
    fn fields_of_ffff_are_all_max() {
        let op = Opcode(0xFFFF);

        assert_eq!(op.family(), 15);
        assert_eq!(op.x(), 15);
        assert_eq!(op.y(), 15);
        assert_eq!(op.n(), 15);
        assert_eq!(op.nn(), 255);
        assert_eq!(op.nnn(), 0xFFF);
    }
}
