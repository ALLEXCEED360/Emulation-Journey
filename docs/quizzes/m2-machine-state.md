# M2 Quiz — Machine state & ROM loading

Answers explained as simply as possible.

---

## 1. Why are `Chip8`'s fields private, and how can its tests still see them?

**Answer: so nothing outside can break the machine by accident. Tests can see them because they live in the same file.**

Think of a **vending machine**. You can press buttons and look through the glass,
but you can't open it up and move the snacks around yourself. Only the machine's
own mechanism can do that.

- **Private fields** = the inside of the machine. Only `Chip8`'s own methods
  (like `load_rom`) can change `memory`, `pc` and so on.
- **Getters** like `pc()` and `memory()` = the glass window. You can *look*, but not touch.

Why it matters: if any code could write `chip8.pc = 5000`, the program counter would point
outside memory and the emulator would crash later, far away from the real mistake.
Keeping fields private means the only way to change the machine is through methods
that check things properly.

**Why tests can still see them:** in Rust, "private" means *"private to this module"*.
The `tests` module is written **inside** `machine.rs`, so it's part of the family and
is allowed to look inside.

---

## 2. What's the difference between `&self` and `&mut self`?

**Answer: `&self` = "let me look". `&mut self` = "let me change".**

Think of a **library book**:

| | Like… | How many at once? | Example from M2 |
|---|---|---|---|
| `&self` | reading the book | as many readers as you like | `pc()`, `memory()`, `display()` |
| `&mut self` | writing notes in the book | only one writer, and nobody else can read while they write | `load_rom()` |

`load_rom` copies bytes **into** memory, so it changes the machine and needs `&mut self`.
`pc()` only **reports** a number, so `&self` is enough.

Rust checks this rule when it compiles your code. That means you can never have one part of
the program reading memory while another part is halfway through changing it.

---

## 3. Why does `load_rom` take `&[u8]` instead of a file path?

**Answer: because the core must not know about files. The desktop app reads the file and hands over just the bytes.**

Remember the rule from M0: the core is the **console**, and the desktop app is the **TV and controller**.

A real console doesn't care whether the game came from a shop, a friend or the internet.
You just put the cartridge in. Same here:

```
chip8-desktop:  "roms/2-ibm-logo.ch8"  →  fs::read  →  [0x00, 0xE0, 0xA2, ...]
                                                              │
chip8-core:                                          load_rom(&bytes)
```

Bonus benefits:

- **Tests are easy.** We can write `load_rom(&[0x00, 0xE0])` with no file at all.
- **A web version can work later.** A browser has no file paths, but it can still give us bytes.

---

## 4. At what memory address does the font character "A" start?

**Answer: `0x082`**

Three facts:

1. The font starts at **`0x050`**.
2. Each character takes **5 bytes**.
3. Characters are stored in order: 0, 1, 2 … 9, A, B … so **A is character number 10**
   (counting from 0).

So we skip past 10 characters:

```
start of A = 0x050 + (10 × 5)
           = 0x050 + 50
           = 0x050 + 0x032      (50 in hex is 0x32)
           = 0x082
```

It's like houses on a street where every house is 5 metres wide. House number 10 starts
50 metres after the first one.

This exact sum becomes instruction `FX29` later: "point I at the font sprite for the digit in VX".

---

## 5. In plain words, what does the `?` operator do?

**Answer: "If this failed, stop right here and pass the error up. If it worked, give me the value and carry on."**

Think of a **checklist for leaving the house**:

```rust
let path = get_argument()?;     // no argument?       → stop, report it
let rom  = fs::read(&path)?;    // file missing?      → stop, report it
chip8.load_rom(&rom)?;          // ROM empty/too big? → stop, report it
println!("Loaded!");            // only reached if ALL of the above worked
```

At each step: if something's wrong, you stop and say *why*. You don't keep going with a
missing file. Without `?`, every one of those lines would need its own `match` with an
`Ok` branch and an `Err` branch, which is a lot of repetitive code.

`?` only works inside a function that itself returns a `Result`. That's why `main.rs`
has a separate `run()` function.

---

## 6. Why is `LoadError` an `enum` rather than just a text message?

**Answer: so the code can tell *which* error happened, and the compiler makes sure every case is handled.**

Imagine a doctor's form:

- A **text message** is like the patient writing *"I feel bad"*. A human can read it,
  but a program can't easily act on it.
- An **enum** is like ticking a box: ☐ `Empty` ☐ `TooLarge`. A program can check
  exactly which box is ticked.

With an enum:

1. **Code can react differently** to each error. A future menu could say "pick a different file"
   for `Empty`, but show the size for `TooLarge`.
2. **Extra details come along.** `TooLarge { size, max }` carries the actual numbers.
3. **The compiler checks you.** `match` must handle every variant. If we add a third error later
   and forget to handle it somewhere, the code won't compile.
4. **Tests can compare exactly:** `assert_eq!(result, Err(LoadError::Empty))`.

And humans still get nice text, thanks to `impl Display`, which turns each variant into a sentence.
