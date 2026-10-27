//! Integration tests for `ciber_term`

use ciber_term::{
  Color,
  Style,
  StyleExt,
};

#[test]
fn test_complex_ansi_composition() {
  let output = format!(
    "{}{}{}{}",
    Style::Bold.to_str(),
    Color::Magenta.to_fg_str(),
    "Warning!",
    Style::Reset.to_str()
  );

  let expected = "\x1b[1m\x1b[35mWarning!\x1b[0m";
  assert_eq!(
    output,
    expected
  );
}

#[test]
fn test_traits_chaining() {
  let styled = "Hello"
    .green()
    .underline()
    .to_string();
  assert_eq!(
    styled,
    "\x1b[4m\x1b[32mHello\x1b[0m"
  );
}
