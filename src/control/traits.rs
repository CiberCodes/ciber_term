use std::fmt::Display;

use super::{
  color::Color,
  styled::StyledText,
};

/// Extension trait providing an elegant builder API for
/// terminal styling.
///
/// This trait is implemented for all types that implement
/// `std::fmt::Display`, allowing for zero-overhead chained
/// formatting.
///
/// # Examples
///
/// ```rust
/// use ciber_term::StyleExt;
///
/// let styled_string = "Success".green().bold();
/// assert_eq!(styled_string.to_string(), "\x1b[1m\x1b[32mSuccess\x1b[0m");
/// ```
pub trait StyleExt: Sized {
  /// Applies a foreground color to the text.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::{Color, StyleExt};
  ///
  /// println!("{}", "Warning".color(Color::Red));
  /// ```
  fn color(
    self,
    color: Color,
  ) -> StyledText<Self>;

  /// Applies a background color to the text.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::{Color, StyleExt};
  ///
  /// println!("{}", "Highlight".bg(Color::Yellow));
  /// ```
  fn bg(
    self,
    color: Color,
  ) -> StyledText<Self>;

  /// Formats the text with a black foreground.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".black());
  /// ```
  fn black(self) -> StyledText<Self>;

  /// Formats the text with a red foreground.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".red());
  /// ```
  fn red(self) -> StyledText<Self>;

  /// Formats the text with a green foreground.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".green());
  /// ```
  fn green(self) -> StyledText<Self>;

  /// Formats the text with a yellow foreground.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".yellow());
  /// ```
  fn yellow(self) -> StyledText<Self>;

  /// Formats the text with a blue foreground.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".blue());
  /// ```
  fn blue(self) -> StyledText<Self>;

  /// Formats the text with a magenta foreground.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".magenta());
  /// ```
  fn magenta(self) -> StyledText<Self>;

  /// Formats the text with a cyan foreground.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".cyan());
  /// ```
  fn cyan(self) -> StyledText<Self>;

  /// Formats the text with a white foreground.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".white());
  /// ```
  fn white(self) -> StyledText<Self>;

  /// Formats the text with a custom RGB foreground color.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".rgb(255, 0, 0));
  /// ```
  fn rgb(
    self,
    r: u8,
    g: u8,
    b: u8,
  ) -> StyledText<Self>;

  /// Formats the text with a bold style.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".bold());
  /// ```
  fn bold(self) -> StyledText<Self>;

  /// Formats the text with an italic style.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".italic());
  /// ```
  fn italic(self) -> StyledText<Self>;

  /// Formats the text with an underline style.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".underline());
  /// ```
  fn underline(self) -> StyledText<Self>;

  /// Formats the text with a blink style.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".blink());
  /// ```
  fn blink(self) -> StyledText<Self>;

  /// Formats the text with a strikethrough style.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".strikethrough());
  /// ```
  fn strikethrough(self) -> StyledText<Self>;

  /// Formats the text with a dim style.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".dim());
  /// ```
  fn dim(self) -> StyledText<Self>;

  /// Resets the text styles.
  ///
  /// # Examples
  ///
  /// ```rust
  /// use ciber_term::StyleExt;
  /// println!("{}", "Everything is OK".reset());
  /// ```
  fn reset(self) -> StyledText<Self>;
}

// The Magic: We implement it for ALL types T that implement
// Display
impl<T: Display> StyleExt for T {
  fn color(
    self,
    color: Color,
  ) -> StyledText<Self> {
    StyledText::new(self).color(color)
  }

  fn bg(
    self,
    color: Color,
  ) -> StyledText<Self> {
    StyledText::new(self).bg(color)
  }

  fn black(self) -> StyledText<Self> {
    StyledText::new(self).black()
  }

  fn red(self) -> StyledText<Self> {
    StyledText::new(self).red()
  }

  fn green(self) -> StyledText<Self> {
    StyledText::new(self).green()
  }

  fn yellow(self) -> StyledText<Self> {
    StyledText::new(self).yellow()
  }

  fn blue(self) -> StyledText<Self> {
    StyledText::new(self).blue()
  }

  fn magenta(self) -> StyledText<Self> {
    StyledText::new(self).magenta()
  }

  fn cyan(self) -> StyledText<Self> {
    StyledText::new(self).cyan()
  }

  fn white(self) -> StyledText<Self> {
    StyledText::new(self).white()
  }

  fn rgb(
    self,
    r: u8,
    g: u8,
    b: u8,
  ) -> StyledText<Self> {
    StyledText::new(self).rgb(
      r, g, b,
    )
  }

  fn bold(self) -> StyledText<Self> {
    StyledText::new(self).bold()
  }

  fn italic(self) -> StyledText<Self> {
    StyledText::new(self).italic()
  }

  fn underline(self) -> StyledText<Self> {
    StyledText::new(self).underline()
  }

  fn blink(self) -> StyledText<Self> {
    StyledText::new(self).blink()
  }

  fn strikethrough(self) -> StyledText<Self> {
    StyledText::new(self).strikethrough()
  }

  fn dim(self) -> StyledText<Self> {
    StyledText::new(self).dim()
  }

  fn reset(self) -> StyledText<Self> {
    StyledText::new(self).reset()
  }
}
