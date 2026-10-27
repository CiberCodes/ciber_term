//! # Example Usage: `ciber_term`
//!
//! This example demonstrates how to initialize the library,
//! manipulate the cursor, clear the screen, and apply
//! styles using the fluid, performant builder API.
//!
//! ## Requirements
//! Ensure your `Cargo.toml` is configured with the
//! necessary dependencies.

use std::io::{
  Write,
  stdout,
};

use ciber_term::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  // 1. Terminal Initialization
  // Mandatory for enabling ANSI support on Windows.
  // On Linux/macOS, this is a no-op (native support).
  init_term();

  // 2. Fluid Styling
  // The `StyleExt` trait allows chaining styles directly
  // onto any type that implements `Display`. This avoids
  // unnecessary allocations and improves
  // code readability.
  println!(
    "{}",
    "Hello, Ciber Term!"
      .cyan()
      .bold()
  );

  // 3. Cursor Manipulation
  // We use a handle to `stdout` to ensure escape sequences
  // are written directly to the system buffer.
  let mut handle = stdout();

  Cursor::down(
    &mut handle,
    2,
  )?;

  Cursor::right(
    &mut handle,
    5,
  )?;

  // 4. Explicitly write and flush to ensure the text
  //    appears at the
  // exact position before further operations.
  write!(
    handle,
    "I am here!"
  )?;
  handle.flush()?;

  // 5. Cursor Visibility
  // Hiding the cursor is essential for TUI (Terminal User
  // Interface) applications to prevent flickering and
  // visual clutter.
  Cursor::hide(&mut handle)?;
  println!("\nCursor hidden for 1 second...");
  std::thread::sleep(std::time::Duration::from_secs(1));

  Cursor::show(&mut handle)?;
  println!("Cursor restored.");

  Ok(())
}
