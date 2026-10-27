use std::io::{
  self,
  Write,
};

/// A utility struct for manipulating the terminal screen.
///
/// Provides methods to clear the terminal window, the
/// current line, or specific portions of the screen using
/// ANSI escape sequences.
///
/// # Examples
///
/// ```rust,no_run
/// use ciber_term::Screen;
///
/// let _ = Screen::clear();
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Screen;

impl Screen {
  /// Clears the entire screen and moves the cursor to the
  /// home position (top-left).
  ///
  /// This method writes to standard output and immediately
  /// flushes the buffer.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Screen;
  ///
  /// let _ = Screen::clear();
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn clear() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(b"\x1b[2J\x1b[H")?;
    stdout.flush()
  }

  /// Clears the screen from the current cursor position to
  /// the end of the display.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Screen;
  ///
  /// let _ = Screen::clear_to_end();
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn clear_to_end() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(b"\x1b[J")?;
    stdout.flush()
  }

  /// Clears the entire current line without moving the
  /// cursor.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Screen;
  ///
  /// let _ = Screen::clear_line();
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn clear_line() -> io::Result<()> {
    let mut stdout = io::stdout();
    stdout.write_all(b"\x1b[2K")?;
    stdout.flush()
  }

  /// Clears the entire screen and moves the cursor to the
  /// home position (top-left) on the provided writer.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Screen;
  /// use std::io::stdout;
  ///
  /// let mut stdout = stdout();
  /// let _ = Screen::clear_on(&mut stdout);
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn clear_on<W: Write>(
    writer: &mut W
  ) -> io::Result<()> {
    writer.write_all(b"\x1b[2J\x1b[H")?;
    writer.flush()
  }
}
