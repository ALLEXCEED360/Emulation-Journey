# Dev Log

## 2026-10-19 — M0: Setup

- Did: installed Rust, created workspace (chip8-core lib + chip8-desktop bin)
- Learned: Setup of rust, its test and printing
- Next: M1 — Rust bits & bytes

## 2026-10-25 — M1: Bits & bytes

- Did: learning tests for integer behavior (`tests/integer_basics.rs`); opcode module with byte/nibble helpers and `Opcode` field extractors (family, x, y, n, nn, nnn)
- Learned: masking with `&` then shifting with `>>` pulls fields out of an opcode; `wrapping_add` vs `overflowing_add` vs plain `+`; `mod opcode;` goes in `lib.rs`, not in `opcode.rs`; check the test count goes up, not just "ok"
- Next: M2 — the Chip8 machine state + ROM loading
