//! Learning tests: small experiments that pin down how Rust integers behave.
//! They don't test the emulator. They test my understanding.

use std::hint::black_box;

#[test]
fn literals_can_be_written_in_any_base() {
    assert_eq!(0xFF, 255);
    assert_eq!(0b1111_1111, 255);
    assert_eq!(0x200, 512);
}

#[test]
fn formatting_shows_numbers_in_hex_and_binary() {
    let value: u8 = 0xAB;
    assert_eq!(format!("{value}"), "171");
    assert_eq!(format!("{value:X}"), "AB");
    assert_eq!(format!("{value:#04X}"), "0xAB");
    assert_eq!(format!("{value:08b}"), "10101011");
}

#[test]
#[should_panic(expected = "attempt to add with overflow")]
fn plain_addition_panics_on_overflow_in_debug_builds() {
    let a: u8 = black_box(255);
    let b: u8 = black_box(1);
    let _sum = a + b;
}

#[test]
fn wrapping_add_rolls_over_like_hardware() {
    assert_eq!(255u8.wrapping_add(1), 0);
    assert_eq!(200u8.wrapping_add(100), 44);
    assert_eq!(0u8.wrapping_sub(1), 255);
}

#[test]
fn overflowing_add_also_reports_the_carry() {
    assert_eq!(200u8.overflowing_add(100), (44, true));
    assert_eq!(10u8.overflowing_add(20), (30, false));
}

#[test]
fn checked_add_returns_none_on_overflow() {
    assert_eq!(250u8.checked_add(5), Some(255));
    assert_eq!(250u8.checked_add(6), None);
}

#[test]
fn widening_casts_never_lose_data() {
    let byte: u8 = 0xAB;
    assert_eq!(u16::from(byte), 0x00AB);
    assert_eq!(byte as u16, 0x00AB);
}

#[test]
fn narrowing_casts_keep_only_the_low_bits() {
    let word: u16 = 0x1FF;
    assert_eq!(word as u8, 0xFF);
}

#[test]
fn shift_binds_tighter_than_bitwise_and() {
    // Reads as 0x1234 & (0x0F00 >> 8). Probably NOT what you meant!
    assert_eq!(0x1234 & 0x0F00 >> 8, 0x4);
    // With parentheses: mask first, then shift.
    assert_eq!((0x1234 & 0x0F00) >> 8, 0x2);
}

#[test]
fn xor_flips_bits_and_flipping_twice_restores() {
    let screen: u8 = 0b1100_1100;
    let sprite: u8 = 0b1010_1010;
    let drawn = screen ^ sprite;
    assert_eq!(drawn, 0b0110_0110);
    assert_eq!(drawn ^ sprite, screen);
}

#[test]
fn reading_individual_bits() {
    let byte: u8 = 0b1000_0001;
    assert_eq!((byte >> 7) & 1, 1); // leftmost bit
    assert_eq!((byte >> 6) & 1, 0);
    assert_eq!(byte & 1, 1); // rightmost bit
}
