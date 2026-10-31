# Dev Log

## 2026-10-19 — M0: Setup

- Did: installed Rust, created workspace (chip8-core lib + chip8-desktop bin)
- Learned: Setup of rust, its test and printing
- Next: M1 — Rust bits & bytes

## 2026-10-25 — M1: Bits & bytes

- Did: learning tests for integer behavior (`tests/integer_basics.rs`); opcode module with byte/nibble helpers and `Opcode` field extractors (family, x, y, n, nn, nnn)
- Learned: masking with `&` then shifting with `>>` pulls fields out of an opcode; `wrapping_add` vs `overflowing_add` vs plain `+`; `mod opcode;` goes in `lib.rs`, not in `opcode.rs`; check the test count goes up, not just "ok"
- Next: M2 — the Chip8 machine state + ROM loading

## 2026-10-31 — M2: Machine state & ROM loading

- Did: built-in font (`font.rs`); `Chip8` struct with memory, registers, I, PC, stack, timers, display and keys (`machine.rs`); `load_rom` with a `LoadError` enum; desktop app reads a ROM file from the command line; first real ROM (IBM logo, 132 bytes, starts with `00 E0`) loads
- Learned: private fields + read-only getters protect the machine's state; `&self` = look, `&mut self` = change; slices (`&memory[a..b]`) are views without copying; errors are values (`Result`), and `?` passes them up; `match` must handle every enum variant
- Next: M3 — fetch and decode instructions
