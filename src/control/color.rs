use std::{
  borrow::Cow,
  fmt,
};

/// Represents ANSI terminal foreground and background
/// colors.
///
/// Supports standard 16 ANSI colors, 256-color palette
/// (8-bit), and 24-bit `TrueColor` RGB.
///
/// # Examples
///
/// ```rust
/// use ciber_term::Color;
///
/// let red = Color::Red;
/// assert_eq!(red.to_fg_str(), "\x1b[31m");
///
/// let rgb = Color::Rgb(255, 100, 50);
/// assert_eq!(rgb.to_fg_str(), "\x1b[38;2;255;100;50m");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Color {
  /// Standard ANSI black (`\x1b[30m`).
  Black,

  /// Standard ANSI red (`\x1b[31m`).
  Red,

  /// Standard ANSI green (`\x1b[32m`).
  Green,

  /// Standard ANSI yellow (`\x1b[33m`).
  Yellow,

  /// Standard ANSI blue (`\x1b[34m`).
  Blue,

  /// Standard ANSI magenta (`\x1b[35m`).
  Magenta,

  /// Standard ANSI cyan (`\x1b[36m`).
  Cyan,

  /// Standard ANSI white (`\x1b[37m`).
  White,

  /// Standard ANSI bright black (`\x1b[90m`).
  BrightBlack,

  /// Standard ANSI bright red (`\x1b[91m`).
  BrightRed,

  /// Standard ANSI bright green (`\x1b[92m`).
  BrightGreen,

  /// Standard ANSI bright yellow (`\x1b[93m`).
  BrightYellow,

  /// Standard ANSI bright blue (`\x1b[94m`).
  BrightBlue,

  /// Standard ANSI bright magenta (`\x1b[95m`).
  BrightMagenta,

  /// Standard ANSI bright cyan (`\x1b[96m`).
  BrightCyan,

  /// Standard ANSI bright white (`\x1b[97m`).
  BrightWhite,

  /// RGB `TrueColor` representation.
  Rgb(
    u8,
    u8,
    u8,
  ),

  /// 256-color palette (8-bit fixed colors).
  Fixed(u8),

  /// Reset all colors and styles to default.
  Reset,
}

impl Color {
  /// Returns the ANSI escape sequence for the foreground
  /// color.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::Color;
  ///
  /// assert_eq!(Color::Red.to_fg_str(), "\x1b[31m");
  /// ```
  #[must_use]
  #[inline]
  pub fn to_fg_str(&self) -> Cow<'static, str> {
    match self {
      | Self::Black => Cow::Borrowed("\x1b[30m"),
      | Self::Red => Cow::Borrowed("\x1b[31m"),
      | Self::Green => Cow::Borrowed("\x1b[32m"),
      | Self::Yellow => Cow::Borrowed("\x1b[33m"),
      | Self::Blue => Cow::Borrowed("\x1b[34m"),
      | Self::Magenta => Cow::Borrowed("\x1b[35m"),
      | Self::Cyan => Cow::Borrowed("\x1b[36m"),
      | Self::White => Cow::Borrowed("\x1b[37m"),
      | Self::BrightBlack => Cow::Borrowed("\x1b[90m"),
      | Self::BrightRed => Cow::Borrowed("\x1b[91m"),
      | Self::BrightGreen => Cow::Borrowed("\x1b[92m"),
      | Self::BrightYellow => Cow::Borrowed("\x1b[93m"),
      | Self::BrightBlue => Cow::Borrowed("\x1b[94m"),
      | Self::BrightMagenta => Cow::Borrowed("\x1b[95m"),
      | Self::BrightCyan => Cow::Borrowed("\x1b[96m"),
      | Self::BrightWhite => Cow::Borrowed("\x1b[97m"),
      // Apenas RGB e Fixed pagam o preço da alocação
      // (Owned)
      | Self::Rgb(r, g, b) => {
        Cow::Owned(format!("\x1b[38;2;{r};{g};{b}m"))
      },
      | Self::Fixed(n) => {
        Cow::Owned(format!("\x1b[38;5;{n}m"))
      },
      | Self::Reset => Cow::Borrowed("\x1b[39m"),
    }
  }

  /// Returns the ANSI escape sequence for the background
  /// color.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::Color;
  ///
  /// assert_eq!(Color::Green.to_bg_str(), "\x1b[42m");
  /// ```
  #[must_use]
  #[inline]
  pub fn to_bg_str(&self) -> Cow<'static, str> {
    match self {
      | Self::Black => Cow::Borrowed("\x1b[40m"),
      | Self::Red => Cow::Borrowed("\x1b[41m"),
      | Self::Green => Cow::Borrowed("\x1b[42m"),
      | Self::Yellow => Cow::Borrowed("\x1b[43m"),
      | Self::Blue => Cow::Borrowed("\x1b[44m"),
      | Self::Magenta => Cow::Borrowed("\x1b[45m"),
      | Self::Cyan => Cow::Borrowed("\x1b[46m"),
      | Self::White => Cow::Borrowed("\x1b[47m"),
      | Self::BrightBlack => Cow::Borrowed("\x1b[100m"),
      | Self::BrightRed => Cow::Borrowed("\x1b[101m"),
      | Self::BrightGreen => Cow::Borrowed("\x1b[102m"),
      | Self::BrightYellow => Cow::Borrowed("\x1b[103m"),
      | Self::BrightBlue => Cow::Borrowed("\x1b[104m"),
      | Self::BrightMagenta => Cow::Borrowed("\x1b[105m"),
      | Self::BrightCyan => Cow::Borrowed("\x1b[106m"),
      | Self::BrightWhite => Cow::Borrowed("\x1b[107m"),
      | Self::Rgb(r, g, b) => {
        Cow::Owned(format!("\x1b[48;2;{r};{g};{b}m"))
      },
      | Self::Fixed(n) => {
        Cow::Owned(format!("\x1b[48;5;{n}m"))
      },
      | Self::Reset => Cow::Borrowed("\x1b[49m"),
    }
  }

  /// Writes the foreground ANSI escape sequence directly to
  /// the formatter.
  ///
  /// This method avoids the allocation of `to_fg_str` by
  /// formatting the escape sequence directly into the
  /// formatter.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::Color;
  ///
  /// let mut f = String::new();
  /// Color::Red.write_fg(&mut f);
  /// assert_eq!(f, "\x1b[31m");
  /// ```
  ///
  /// # Errors
  /// Returns an `std::fmt::Result` if writing to the
  /// formatter fails.
  #[inline]
  pub fn write_fg<W: fmt::Write>(
    self,
    f: &mut W,
  ) -> fmt::Result {
    match self {
      | Self::Rgb(r, g, b) => write!(
        f,
        "\x1b[38;2;{r};{g};{b}m"
      ),
      | Self::Fixed(n) => write!(
        f,
        "\x1b[38;5;{n}m"
      ),
      | _ => write!(
        f,
        "{}",
        self.to_fg_str()
      ),
    }
  }

  /// Writes the background ANSI escape sequence directly to
  /// the formatter.
  ///
  /// This method avoids the allocation of `to_bg_str` by
  /// formatting the escape sequence directly into the
  /// formatter.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::Color;
  ///
  /// let mut f = String::new();
  /// Color::Red.write_bg(&mut f);
  /// assert_eq!(f, "\x1b[41m");
  /// ```
  ///
  /// # Errors
  /// Returns an `std::fmt::Result` if writing to the
  /// formatter fails.
  #[inline]
  pub fn write_bg<W: fmt::Write>(
    self,
    f: &mut W,
  ) -> fmt::Result {
    match self {
      | Self::Rgb(r, g, b) => write!(
        f,
        "\x1b[48;2;{r};{g};{b}m"
      ),
      | Self::Fixed(n) => write!(
        f,
        "\x1b[48;5;{n}m"
      ),
      | _ => write!(
        f,
        "{}",
        self.to_bg_str()
      ),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_standard_colors() {
    assert_eq!(
      Color::Red.to_fg_str(),
      "\x1b[31m"
    );
    assert_eq!(
      Color::Green.to_bg_str(),
      "\x1b[42m"
    );
  }

  #[test]
  fn test_rgb_colors() {
    // Adapt according to your RGB struct/methods
    let rgb_color = Color::Rgb(
      255,
      100,
      50,
    );
    assert_eq!(
      rgb_color.to_fg_str(),
      "\x1b[38;2;255;100;50m"
    );
  }
}
