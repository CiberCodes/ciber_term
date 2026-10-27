#[cfg(windows)]
pub use crate::state::windows::enable_ansi_support;
pub use crate::{
  control::{
    color::Color,
    cursor::Cursor,
    screen::Screen,
    style::Style,
    styled::StyledText,
    traits::StyleExt,
  },
  state::{
    event::{
      Event,
      KeyCode,
      KeyEvent,
      poll,
      read,
    },
    init_term,
    raw::RawModeGuard,
    settings::{
      are_colors_enabled,
      set_colors_enabled,
    },
    terminal::TerminalSize,
  },
};
