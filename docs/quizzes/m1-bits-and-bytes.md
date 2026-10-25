# M1 Quiz — Bits & bytes

Answers explained as simply as possible.

---

## 1. What is `0xD123 >> 4`?

**Answer: `0x0D12`**

Normal numbers first: chop the last digit off **1234** and you get **123**.

`>> 4` does the same with hex digits. Each hex digit is 4 bits, so `>> 4` means
**"slide everything one hex digit to the right, and the last digit falls off"**.

```
0xD123
   >> 4
0x0D12     ← the "3" fell off, a 0 came in on the left
```

Rule of thumb:

| Shift | Hex digits dropped |
|---|---|
| `>> 4` | 1 |
| `>> 8` | 2 |
| `>> 12` | 3 |

That's why `family()` uses `>> 12`: it drops three digits, so only the `D` is left.

---

## 2. What is `200u8.wrapping_add(100)`, and why?

**Answer: `44`**

A `u8` is a small box that only holds **0 to 255**. 200 + 100 = 300, which **doesn't fit**.

Think of a **clock**. 11 o'clock + 3 hours isn't 14 o'clock. You go past 12, start again, and land on 2.

A `u8` is a clock with 256 positions (0 to 255):

```
200 + 100 = 300
300 − 256 = 44     ← went past 255, started again from 0
```

`wrapping_add` = **"if you go past the top, wrap around to the bottom, like a clock."**
Real hardware works this way, so the emulator uses it.

---

## 3. Which CHIP-8 instruction needs `overflowing_add`, and what is the `bool` for?

**Answer: `8XY4` (add VY to VX). The `bool` becomes the carry flag in VF.**

Remember adding on paper in school:

```
  7
+ 5
----
 12   ← doesn't fit in one column, so you "carry the 1"
```

`8XY4` does the same with bytes. When the answer is too big for a `u8`,
the CPU has to remember that it **carried**.

`overflowing_add` gives you **two things at once**:

| Part | Example | Where it goes |
|---|---|---|
| the wrapped answer | `44` | into **VX** |
| "did it go past 255?" | `true` / `false` | into **VF** as `1` / `0` |

VF is like the little "carry the 1" mark you write above the column.

---

## 4. What is `0x1FF as u8`?

**Answer: `0xFF` (255)**

`0x1FF` needs 3 hex digits, but a `u8` only has room for **2**.
`as u8` **cuts off everything except the last 2 hex digits**:

```
0x1FF
   ↓ keep only the last 2 digits
 0xFF
```

Like writing a phone number on a sticky note that only fits the last two digits:
the rest is just lost. **No error, no warning.** That's why `as` can be dangerous.

---

## 5. Why does `x()` return `usize` while `nn()` returns `u8`?

**Answer: they do different jobs.**

Think of a row of 16 lockers (the registers V0–VF):

- **X is the locker number**: *which* locker to open. Rust's rule is that positions
  in a list (`registers[x]`) must be `usize`, so `x()` gives back a `usize`, ready to use.
- **NN is the thing you put inside**: a *value* like 42. Each locker holds one byte,
  and a byte is a `u8`, so `nn()` gives back a `u8`, ready to store.

Different kinds of numbers, so they get different types.

---

## 6. `(value & 0x00FF) as u8` and `value as u8` give the same result. Why write the mask?

**Answer: so a human reading the code instantly knows what you meant.**

- `value as u8` relies on the sticky-note cutting from question 4. A reader has to stop and
  wonder: *"Is losing data here on purpose, or is it a bug?"*
- `(value & 0x00FF) as u8` clearly says **"I want only the low byte, on purpose."**
  The cast then loses nothing, because the mask already removed the top part.

Like a recipe saying "use the egg" vs. "use **only the yolk**": the second leaves no doubt.

**Code is read far more often than it's written**, so make your intention obvious.
