<div align="center">

# 🖥️ Ciber Term

**Native ANSI Terminal Control & Formatting**

[![CI](https://github.com/CiberCodes/ciber_term/actions/workflows/ci.yml/badge.svg)](https://github.com/CiberCodes/ciber_term/actions/workflows/ci.yml) [![Crates.io](https://img.shields.io/crates/v/ciber_term.svg)](https://crates.io/crates/ciber_term) [![Docs.rs](https://docs.rs/ciber_term/badge.svg)](https://docs.rs/ciber_term) [![Language](https://img.shields.io/badge/Language-Rust-orange?logo=rust)](https://www.rust-lang.org/) [![License](https://img.shields.io/badge/License-MIT%2FApache-blue.svg)](#-license)

</div>

---

## 📖 Overview

**Ciber Term** is a highly optimized, dependency-free foundational library for terminal manipulation. It provides native ANSI escape sequence generation for colors, text styling, and full cursor control.

Designed to be the ultimate bedrock for CLI tools within the CiberCodes ecosystem, prioritizing zero-cost abstractions and flawless terminal integration.

## 🚀 Quick Start

Add the following to your `Cargo.toml`:

```toml
[dependencies]
ciber_term = "0.1"
```

And then in your code:

```rust
use std::io::stdout;
use ciber_term::{Color, Cursor, Screen, Style, StyleExt};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let mut stdout = stdout();

  // Clear the screen
  Screen::clear()?;

  // Move cursor and print styled text
  Cursor::set_position(&mut stdout, 5, 5)?;
  print!(
    "{} {}Hello, Ciber Term!{}",
    Style::Bold.to_str(),
    Color::Cyan.to_fg_str(),
    Style::Reset.to_str()
  );

  // Use extension traits for fluent styling
  println!("{}", "This text is red and underlined".red().underline());

  // Hide cursor
  Cursor::hide(&mut stdout)?;

  // Move cursor and print again
  Cursor::set_position(&mut stdout, 10, 10)?;
  println!("{}", "Cursor is hidden!".green());

  // Show cursor
  Cursor::show(&mut stdout)?;

  Ok(())
}
```
---

## 🚧 Current Status & Roadmap

### ✅ v0.1.0 — Completed

- [x] Core color manipulation (16 colors, 256 colors, TrueColor RGB).
- [x] Text formatting traits (Bold, Italic, Blink, Dim, etc.).
- [x] Absolute and relative cursor positioning.
- [x] Screen manipulation and clearing.
- [x] **Windows ANSI:** Auto-enable native ANSI support on Windows.
- [x] **Terminal Dimensions:** Methods to query terminal width and height dynamically.
- [x] **Raw Mode Toggle:** Ability to switch the terminal to raw mode for unbuffered, character-by-character input reading.
- [x] **NO_COLOR Standard:** Automatic respect for the `NO_COLOR` environment variable to disable styling dynamically.
- [x] **Async Key Events:** Foundational support for reading non-blocking keyboard and mouse events.

### 🔭 v0.2.0 — Planned

- [ ] **Mouse Tracking Support:** Implement SGR-style mouse reporting (click, drag, scroll).
- [ ] **Modifier Support:** Track Ctrl, Alt, and Shift for complex shortcuts.
- [ ] **Buffered Rendering:** Double buffering to prevent screen flickering.
- [ ] **Event Loop Abstraction:** Helper traits to manage TUI lifecycles easily.
- [ ] **Performance Optimization:** Batching ANSI sequences to reduce syscalls.

---

## 📜 License

Licensed under either of the following, at your option:

* **[MIT License](LICENSE-MIT)**
* **[Apache License 2.0](LICENSE-APACHE)**

---

<div align="center">
  <i>Minimalist by design. Consistent in execution.</i><br>
  <sub>Engineered by CiberCodes</sub>
</div>