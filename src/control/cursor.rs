use std::io::{
  self,
  Write,
};

/// A utility struct for manipulating the terminal cursor.
///
/// Provides methods to move the cursor, change its
/// visibility, and save/restore its position using ANSI
/// escape sequences.
///
/// # Examples
///
/// ```rust,no_run
/// use ciber_term::Cursor;
/// use std::io::stdout;
///
/// let mut writer = stdout();
/// let _ = Cursor::set_position(&mut writer, 10, 5);
/// let _ = Cursor::hide(&mut writer);
/// let _ = Cursor::flush(&mut writer);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Cursor;

impl Cursor {
  /// Moves the cursor up by the specified number of lines.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Cursor;
  /// use std::io::stdout;
  ///
  /// let mut stdout = stdout();
  /// let _ = Cursor::up(&mut stdout, 2);
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn up<W: Write>(
    writer: &mut W,
    n: u16,
  ) -> io::Result<()> {
    write!(
      writer,
      "\x1b[{n}A"
    )
  }

  /// Moves the cursor down by the specified number of
  /// lines.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Cursor;
  /// use std::io::stdout;
  ///
  /// let mut stdout = stdout();
  /// let _ = Cursor::down(&mut stdout, 3);
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn down<W: Write>(
    writer: &mut W,
    n: u16,
  ) -> io::Result<()> {
    write!(
      writer,
      "\x1b[{n}B"
    )
  }

  /// Moves the cursor right by the specified number of
  /// columns.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Cursor;
  /// use std::io::stdout;
  ///
  /// let mut stdout = stdout();
  /// let _ = Cursor::right(&mut stdout, 5);
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn right<W: Write>(
    writer: &mut W,
    n: u16,
  ) -> io::Result<()> {
    write!(
      writer,
      "\x1b[{n}C"
    )
  }

  /// Moves the cursor left by the specified number of
  /// columns.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Cursor;
  /// use std::io::stdout;
  ///
  /// let mut stdout = stdout();
  /// let _ = Cursor::left(&mut stdout, 4);
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn left<W: Write>(
    writer: &mut W,
    n: u16,
  ) -> io::Result<()> {
    write!(
      writer,
      "\x1b[{n}D"
    )
  }

  /// Sets the absolute position of the cursor.
  ///
  /// `x` represents the column (1-based) and `y` represents
  /// the row (1-based).
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Cursor;
  /// use std::io::stdout;
  ///
  /// let mut stdout = stdout();
  /// let _ = Cursor::set_position(&mut stdout, 10, 5);
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn set_position<W: Write>(
    writer: &mut W,
    x: u16,
    y: u16,
  ) -> io::Result<()> {
    write!(
      writer,
      "\x1b[{y};{x}H"
    )
  }

  /// Hides the terminal cursor.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Cursor;
  /// use std::io::stdout;
  ///
  /// let mut stdout = stdout();
  /// let _ = Cursor::hide(&mut stdout);
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn hide<W: Write>(writer: &mut W) -> io::Result<()> {
    writer.write_all(b"\x1b[?25l")
  }

  /// Shows the terminal cursor.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Cursor;
  /// use std::io::stdout;
  ///
  /// let mut stdout = stdout();
  /// let _ = Cursor::show(&mut stdout);
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn show<W: Write>(writer: &mut W) -> io::Result<()> {
    writer.write_all(b"\x1b[?25h")
  }

  /// Saves the current cursor position.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Cursor;
  /// use std::io::stdout;
  ///
  /// let mut stdout = stdout();
  /// let _ = Cursor::save_position(&mut stdout);
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn save_position<W: Write>(
    writer: &mut W
  ) -> io::Result<()> {
    writer.write_all(b"\x1b[s")
  }

  /// Restores the cursor to the last saved position.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Cursor;
  /// use std::io::stdout;
  ///
  /// let mut stdout = stdout();
  /// let _ = Cursor::restore_position(&mut stdout);
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn restore_position<W: Write>(
    writer: &mut W
  ) -> io::Result<()> {
    writer.write_all(b"\x1b[u")
  }

  /// Flushes the standard output buffer.
  ///
  /// Terminal escape sequences are buffered by default.
  /// Calling this ensures that all cursor movements are
  /// executed immediately.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::Cursor;
  /// use std::io::stdout;
  ///
  /// let mut stdout = stdout();
  /// let _ = Cursor::flush(&mut stdout);
  /// ```
  ///
  /// # Errors
  /// Returns an `std::io::Result` if writing to the
  /// terminal fails.
  pub fn flush<W: Write>(writer: &mut W) -> io::Result<()> {
    writer.flush()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_cursor_movements_generate_correct_ansi()
  -> Result<(), Box<dyn std::error::Error>> {
    let mut buffer = Vec::new();

    // Testamos se a string ANSI gerada está correta
    Cursor::set_position(
      &mut buffer,
      10,
      5,
    )?;
    assert_eq!(
      buffer,
      b"\x1b[5;10H"
    );

    buffer.clear(); // Limpa o buffer para o próximo teste

    Cursor::up(
      &mut buffer,
      2,
    )?;
    // Nota: O flush() vai acontecer dentro da função se
    // você o adicionou lá!
    assert_eq!(
      buffer,
      b"\x1b[2A"
    );
    Ok(())
  }
}
