//! This example demonstrates how to use the library to
//! create a simple, non-blocking interactive terminal
//! application using the event polling engine.

use std::{
  io,
  time::Duration,
};

use ciber_term::prelude::*;

fn main() -> io::Result<()> {
  // 1. Enable raw mode and set up the initial terminal
  //    state
  let _raw = RawModeGuard::enable()?;
  let mut stdout = io::stdout();

  Cursor::hide(&mut stdout)?;
  Screen::clear()?;
  Cursor::set_position(
    &mut stdout,
    1,
    1,
  )?;

  // Note: The '\r' is required in Raw Mode to return the
  // cursor to the start of the line
  println!(
    "Interactive Mode! Press the Arrow keys to test, or \
     'q' to quit.\r"
  );

  // 2. The non-blocking event loop
  loop {
    // Poll for events with a 50ms timeout (loop runs at ~20
    // iterations per second if idle)
    if poll(Duration::from_millis(50))? {
      // An event is ready, so `read()` will process it
      // immediately without blocking
      match read()? {
        | Event::Key(key_event) => match key_event.code {
          | KeyCode::Up => println!("Up Arrow detected!\r"),
          | KeyCode::Down => {
            println!("Down Arrow detected!\r");
          },
          | KeyCode::Left => {
            println!("Left Arrow detected!\r");
          },
          | KeyCode::Right => {
            println!("Right Arrow detected!\r");
          },
          | KeyCode::Enter => println!("Enter pressed!\r"),
          | KeyCode::Char('q') => {
            println!("Exiting system...\r");
            break; // Break the loop to trigger cleanup
          },
          | KeyCode::Char(c) => {
            println!("You typed: {c}\r");
          },
          | _ => {}, // Ignore unknown or unhandled keys
        },
      }
    }
    else {
      // 3. Background Processing
      // This block executes whenever the poll timeout
      // expires with no input. Put background tasks
      // here (e.g., animations, network calls, state
      // updates).
    }
  }

  // 4. Cleanup
  // Restore the cursor before exiting the application.
  // Raw mode is automatically disabled here when the `_raw`
  // guard goes out of scope.
  Cursor::show(&mut stdout)?;

  Ok(())
}
