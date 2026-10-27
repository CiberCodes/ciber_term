/// Represents ANSI text formatting styles.
///
/// # Examples
///
/// ```rust
/// use ciber_term::Style;
///
/// assert_eq!(Style::Bold.to_str(), "\x1b[1m");
/// assert_eq!(Style::Reset.to_str(), "\x1b[0m");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Style {
  /// Makes the text bold or increased intensity.
  Bold,

  /// Decreases the intensity of the text.
  Dim,

  /// Makes the text italic (not supported in all
  /// terminals).
  Italic,

  /// Underlines the text (not supported in all terminals).
  Underline,

  /// Makes the text blink (not supported in all terminals).
  Blink,

  /// Reverses the foreground and background colors.
  Reverse,

  /// Hides the text (not supported in all terminals).
  Hidden,

  /// Puts a line through the text (not supported in all
  /// terminals).
  Strikethrough,

  /// Resets all styles to default.
  Reset,
}

impl Style {
  /// Returns the ANSI escape sequence for the text style.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::Style;
  ///
  /// assert_eq!(Style::Bold.to_str(), "\x1b[1m");
  /// ```
  #[must_use]
  pub const fn to_str(&self) -> &'static str {
    match self {
      | Self::Bold => "\x1b[1m",
      | Self::Dim => "\x1b[2m",
      | Self::Italic => "\x1b[3m",
      | Self::Underline => "\x1b[4m",
      | Self::Blink => "\x1b[5m",
      | Self::Reverse => "\x1b[7m",
      | Self::Hidden => "\x1b[8m",
      | Self::Strikethrough => "\x1b[9m",
      | Self::Reset => "\x1b[0m",
    }
  }
}

impl std::fmt::Display for Style {
  #[inline]
  fn fmt(
    &self,
    f: &mut std::fmt::Formatter<'_>,
  ) -> std::fmt::Result {
    f.write_str(self.to_str())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_style_codes() {
    assert_eq!(
      Style::Bold.to_str(),
      "\x1b[1m"
    );
    assert_eq!(
      Style::Reset.to_str(),
      "\x1b[0m"
    );
    assert_eq!(
      Style::Dim.to_str(),
      "\x1b[2m"
    );
  }
}
