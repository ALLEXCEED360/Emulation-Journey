# M0 Quiz — Setup

Answers explained as simply as possible.

---

## 1. Why does `pixel_count` have no `;` on its last line?

**Answer: because in Rust, the last line without a `;` is what the function gives back.**

```rust
fn pixel_count() -> usize {
    DISPLAY_WIDTH * DISPLAY_HEIGHT    // no ; → this number is returned
}
```

Think of a function like a vending machine. The last line without a `;` is the snack that drops out.

If you add a `;`, you're saying "do this calculation, then throw the result away". Now the machine gives you **nothing**, and Rust complains:

```
expected `usize`, found `()`
```

`()` is Rust's way of writing "nothing".

> You *can* write `return DISPLAY_WIDTH * DISPLAY_HEIGHT;` and it works too.
> But Rust programmers only use `return` to leave a function **early**.

---

## 2. Why `chip8_core` in `use`, but `chip8-core` in `Cargo.toml`?

**Answer: Rust code can't have `-` in names, so Cargo swaps it for `_`.**

- In `Cargo.toml`, `chip8-core` is just a **label**. Hyphens are fine there.
- In Rust code, a `-` means **minus**. So `chip8-core` would mean "chip8 take away core", which makes no sense.

So Cargo quietly turns the hyphen into an underscore for you:

```
Cargo.toml:   chip8-core
Rust code:    chip8_core
```

---

## 3. Why are window and keyboard code banned from `chip8-core`?

**Answer: so the emulator's "brain" works on its own, without needing a screen.**

Think of a game console and a TV.

- The **console** (`chip8-core`) runs the game.
- The **TV and controller** (`chip8-desktop`) show the picture and read the buttons.

You can plug the same console into any TV. Keeping them separate gives us:

1. **Fast tests.** We can test the brain thousands of times per second without opening a window.
2. **Easy swaps.** We'll change the window library later (minifb → egui → maybe a web version), and the core won't change at all.
3. **Easy bug hunting.** Wrong picture? If the core's pixels are right, the bug is in the TV part, and vice versa.

```
chip8-desktop  (TV + controller)        chip8-core  (console)
───────────────────────────────         ─────────────────────
reads ROM file   ── bytes ──────────▶   load_rom()
reads keyboard   ── key presses ────▶   set_key()
draws window     ◀── pixels ─────────   display()
plays beep       ◀── "beep on?" ─────   sound timer
```

---

## 4. What is `0x200` in decimal?

**Answer: 512.**

Hex counts in **16s** instead of 10s. Each position is worth 16 times the one to its right:

```
0x200 = 2 × 256  +  0 × 16  +  0 × 1
      = 512
```

Same idea as normal numbers: `200` in decimal = 2 × 100 + 0 × 10 + 0 × 1.

**Why programs start at 512:** on the original 1977 computer, the CHIP-8 program itself took up the first 512 bytes of memory. Games had to start after it. We'll put the **font** (the shapes for 0–F) in that space instead.

So the biggest possible game is **4096 − 512 = 3584 bytes**.

**Why programmers love hex:** 1 hex digit = exactly 4 bits, 2 hex digits = exactly 1 byte. So `0x6A05` can be read digit by digit: `6` = "set register", `A` = register A, `05` = the number 5. In normal decimal (27141) you can't see any of that.
