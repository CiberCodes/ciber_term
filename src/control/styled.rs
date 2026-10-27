use std::fmt;

use super::{
  Color,
  Style,
};
use crate::state::are_colors_enabled;

/// Bit flags for text styles.
const STYLE_BOLD: u8 = 1 << 0;
const STYLE_DIM: u8 = 1 << 1;
const STYLE_ITALIC: u8 = 1 << 2;
const STYLE_UNDERLINE: u8 = 1 << 3;
const STYLE_BLINK: u8 = 1 << 4;
const STYLE_REVERSE: u8 = 1 << 5;
const STYLE_HIDDEN: u8 = 1 << 6;
const STYLE_STRIKETHROUGH: u8 = 1 << 7;

/// A deferred evaluation builder for ANSI terminal styles.
///
/// Instead of allocating strings immediately, `StyledText`
/// stores the requested styles and injects the ANSI escape
/// codes directly into the buffer when `std::fmt::Display`
/// is invoked. This ensures maximum performance and
/// prevents duplicate reset codes.
///
/// # Examples
///
/// ```rust
/// use ciber_term::{Color, StyledText};
///
/// let styled = StyledText::new("Success")
///   .color(Color::Green)
///   .bold();
///
/// println!("{styled}");
/// ```
#[derive(Debug)]
pub struct StyledText<T> {
  /// The underlying text or data to be styled.
  pub text: T,

  /// The foreground color to be applied, if any.
  pub fg: Option<Color>,

  /// The background color to be applied, if any.
  pub bg: Option<Color>,

  /// A list of text formatting styles to be applied.
  pub style_mask: u8,
}

impl<T> StyledText<T> {
  /// Creates a new `StyledText` instance wrapping the
  /// provided text.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyledText;
  ///
  /// let text = StyledText::new("Hello, World!");
  /// assert_eq!(text.to_string(), "Hello, World!\x1b[0m");
  /// ```
  #[must_use]
  pub const fn new(text: T) -> Self {
    Self {
      text,
      fg: None,
      bg: None,
      style_mask: 0,
    }
  }

  /// Sets the foreground color to the specified `Color`.
  #[must_use]
  pub const fn color(
    mut self,
    color: Color,
  ) -> Self {
    self.fg = Some(color);
    self
  }

  /// Sets the background color to the specified `Color`.
  #[must_use]
  pub const fn bg(
    mut self,
    color: Color,
  ) -> Self {
    self.bg = Some(color);
    self
  }

  /// Sets the foreground color using an 8-bit (256-color)
  /// palette index.
  #[must_use]
  pub const fn fixed(
    mut self,
    n: u8,
  ) -> Self {
    self.fg = Some(Color::Fixed(n));
    self
  }

  /// Sets the foreground color using an RGB `TrueColor`
  /// value.
  #[must_use]
  pub const fn rgb(
    mut self,
    r: u8,
    g: u8,
    b: u8,
  ) -> Self {
    self.fg = Some(
      Color::Rgb(
        r, g, b,
      ),
    );
    self
  }

  /// Sets the background color using an 8-bit (256-color)
  /// palette index.
  #[must_use]
  pub const fn bg_fixed(
    mut self,
    n: u8,
  ) -> Self {
    self.bg = Some(Color::Fixed(n));
    self
  }

  /// Sets the background color using an RGB `TrueColor`
  /// value.
  #[must_use]
  pub const fn bg_rgb(
    mut self,
    r: u8,
    g: u8,
    b: u8,
  ) -> Self {
    self.bg = Some(
      Color::Rgb(
        r, g, b,
      ),
    );
    self
  }

  /// Clears all applied styles, foreground, and background
  /// colors.
  #[must_use]
  pub const fn reset(mut self) -> Self {
    self.fg = Some(Color::Reset);
    self.bg = Some(Color::Reset);
    self.style_mask = 0;
    self
  }

  /// Sets the foreground color to black.
  #[must_use]
  pub const fn black(mut self) -> Self {
    self.fg = Some(Color::Black);
    self
  }

  /// Sets the foreground color to bright black (gray).
  #[must_use]
  pub const fn bright_black(mut self) -> Self {
    self.fg = Some(Color::BrightBlack);
    self
  }

  /// Sets the foreground color to bright red.
  #[must_use]
  pub const fn bright_red(mut self) -> Self {
    self.fg = Some(Color::BrightRed);
    self
  }

  /// Sets the foreground color to bright green.
  #[must_use]
  pub const fn bright_green(mut self) -> Self {
    self.fg = Some(Color::BrightGreen);
    self
  }

  /// Sets the foreground color to bright yellow.
  #[must_use]
  pub const fn bright_yellow(mut self) -> Self {
    self.fg = Some(Color::BrightYellow);
    self
  }

  /// Sets the foreground color to bright blue.
  #[must_use]
  pub const fn bright_blue(mut self) -> Self {
    self.fg = Some(Color::BrightBlue);
    self
  }

  /// Sets the foreground color to bright magenta.
  #[must_use]
  pub const fn bright_magenta(mut self) -> Self {
    self.fg = Some(Color::BrightMagenta);
    self
  }

  /// Sets the foreground color to bright cyan.
  #[must_use]
  pub const fn bright_cyan(mut self) -> Self {
    self.fg = Some(Color::BrightCyan);
    self
  }

  /// Sets the foreground color to bright white.
  #[must_use]
  pub const fn bright_white(mut self) -> Self {
    self.fg = Some(Color::BrightWhite);
    self
  }

  /// Sets the foreground color to red.
  #[must_use]
  pub const fn red(mut self) -> Self {
    self.fg = Some(Color::Red);
    self
  }

  /// Sets the foreground color to green.
  #[must_use]
  pub const fn green(mut self) -> Self {
    self.fg = Some(Color::Green);
    self
  }

  /// Sets the foreground color to yellow.
  #[must_use]
  pub const fn yellow(mut self) -> Self {
    self.fg = Some(Color::Yellow);
    self
  }

  /// Sets the foreground color to blue.
  #[must_use]
  pub const fn blue(mut self) -> Self {
    self.fg = Some(Color::Blue);
    self
  }

  /// Sets the foreground color to magenta.
  #[must_use]
  pub const fn magenta(mut self) -> Self {
    self.fg = Some(Color::Magenta);
    self
  }

  /// Sets the foreground color to cyan.
  #[must_use]
  pub const fn cyan(mut self) -> Self {
    self.fg = Some(Color::Cyan);
    self
  }

  /// Sets the foreground color to white.
  #[must_use]
  pub const fn white(mut self) -> Self {
    self.fg = Some(Color::White);
    self
  }

  /// Sets the background color to black.
  #[must_use]
  pub const fn bg_black(mut self) -> Self {
    self.bg = Some(Color::Black);
    self
  }

  /// Sets the background color to bright black (gray).
  #[must_use]
  pub const fn bg_bright_black(mut self) -> Self {
    self.bg = Some(Color::BrightBlack);
    self
  }

  /// Sets the background color to bright red.
  #[must_use]
  pub const fn bg_bright_red(mut self) -> Self {
    self.bg = Some(Color::BrightRed);
    self
  }

  /// Sets the background color to bright green.
  #[must_use]
  pub const fn bg_bright_green(mut self) -> Self {
    self.bg = Some(Color::BrightGreen);
    self
  }

  /// Sets the background color to bright yellow.
  #[must_use]
  pub const fn bg_bright_yellow(mut self) -> Self {
    self.bg = Some(Color::BrightYellow);
    self
  }

  /// Sets the background color to bright blue.
  #[must_use]
  pub const fn bg_bright_blue(mut self) -> Self {
    self.bg = Some(Color::BrightBlue);
    self
  }

  /// Sets the background color to bright magenta.
  #[must_use]
  pub const fn bg_bright_magenta(mut self) -> Self {
    self.bg = Some(Color::BrightMagenta);
    self
  }

  /// Sets the background color to bright cyan.
  #[must_use]
  pub const fn bg_bright_cyan(mut self) -> Self {
    self.bg = Some(Color::BrightCyan);
    self
  }

  /// Sets the background color to bright white.
  #[must_use]
  pub const fn bg_bright_white(mut self) -> Self {
    self.bg = Some(Color::BrightWhite);
    self
  }

  /// Sets the background color to red.
  #[must_use]
  pub const fn bg_red(mut self) -> Self {
    self.bg = Some(Color::Red);
    self
  }

  /// Sets the background color to green.
  #[must_use]
  pub const fn bg_green(mut self) -> Self {
    self.bg = Some(Color::Green);
    self
  }

  /// Sets the background color to yellow.
  #[must_use]
  pub const fn bg_yellow(mut self) -> Self {
    self.bg = Some(Color::Yellow);
    self
  }

  /// Sets the background color to blue.
  #[must_use]
  pub const fn bg_blue(mut self) -> Self {
    self.bg = Some(Color::Blue);
    self
  }

  /// Sets the background color to magenta.
  #[must_use]
  pub const fn bg_magenta(mut self) -> Self {
    self.bg = Some(Color::Magenta);
    self
  }

  /// Sets the background color to cyan.
  #[must_use]
  pub const fn bg_cyan(mut self) -> Self {
    self.bg = Some(Color::Cyan);
    self
  }

  /// Sets the background color to white.
  #[must_use]
  pub const fn bg_white(mut self) -> Self {
    self.bg = Some(Color::White);
    self
  }

  /// Adds the underline style to the text.
  #[must_use]
  pub const fn underline(mut self) -> Self {
    self.style_mask |= STYLE_UNDERLINE;
    self
  }

  /// Decreases the intensity of the text color.
  #[must_use]
  pub const fn dim(mut self) -> Self {
    self.style_mask |= STYLE_DIM;
    self
  }

  /// Adds the italic style to the text.
  #[must_use]
  pub const fn italic(mut self) -> Self {
    self.style_mask |= STYLE_ITALIC;
    self
  }

  /// Adds the blink style to the text.
  #[must_use]
  pub const fn blink(mut self) -> Self {
    self.style_mask |= STYLE_BLINK;
    self
  }

  /// Reverses the foreground and background colors.
  #[must_use]
  pub const fn reverse(mut self) -> Self {
    self.style_mask |= STYLE_REVERSE;
    self
  }

  /// Hides the text completely.
  #[must_use]
  pub const fn hidden(mut self) -> Self {
    self.style_mask |= STYLE_HIDDEN;
    self
  }

  /// Adds a strikethrough line to the text.
  #[must_use]
  pub const fn strikethrough(mut self) -> Self {
    self.style_mask |= STYLE_STRIKETHROUGH;
    self
  }

  /// Adds the bold style to the text.
  #[must_use]
  pub const fn bold(mut self) -> Self {
    self.style_mask |= STYLE_BOLD;
    self
  }
}

// Display implementation ensuring a single Reset at the end
impl<T: fmt::Display> fmt::Display for StyledText<T> {
  fn fmt(
    &self,
    f: &mut fmt::Formatter<'_>,
  ) -> fmt::Result {
    // THE MAGIC: If NO_COLOR is set, bypass all styling
    // logic!
    if !are_colors_enabled() {
      return write!(
        f,
        "{}",
        self.text
      );
    }

    if self.style_mask & STYLE_BOLD != 0 {
      write!(
        f,
        "{}",
        Style::Bold.to_str()
      )?;
    }
    if self.style_mask & STYLE_DIM != 0 {
      write!(
        f,
        "{}",
        Style::Dim.to_str()
      )?;
    }
    if self.style_mask & STYLE_ITALIC != 0 {
      write!(
        f,
        "{}",
        Style::Italic.to_str()
      )?;
    }
    if self.style_mask & STYLE_UNDERLINE != 0 {
      write!(
        f,
        "{}",
        Style::Underline.to_str()
      )?;
    }
    if self.style_mask & STYLE_BLINK != 0 {
      write!(
        f,
        "{}",
        Style::Blink.to_str()
      )?;
    }
    if self.style_mask & STYLE_REVERSE != 0 {
      write!(
        f,
        "{}",
        Style::Reverse.to_str()
      )?;
    }
    if self.style_mask & STYLE_HIDDEN != 0 {
      write!(
        f,
        "{}",
        Style::Hidden.to_str()
      )?;
    }
    if self.style_mask & STYLE_STRIKETHROUGH != 0 {
      write!(
        f,
        "{}",
        Style::Strikethrough.to_str()
      )?;
    }

    if let Some(fg) = &self.fg {
      fg.write_fg(f)?;
    }
    if let Some(bg) = &self.bg {
      bg.write_bg(f)?;
    }

    write!(
      f,
      "{}",
      self.text
    )?;
    write!(
      f,
      "{}",
      Style::Reset.to_str()
    )
  }
}
