/// Cross-platform terminal abstractions.
pub mod terminal;

/// Raw mode toggle for terminal.
pub mod raw;

/// Terminal application settings.
pub mod settings;

/// Windows specific terminal state utilities.
#[cfg(windows)]
pub mod windows;

/// Asynchronous terminal event handling (Keyboard, Mouse).
pub mod event;

use std::{
  io::IsTerminal,
  sync::Once,
};

pub use raw::RawModeGuard;
pub use settings::{
  are_colors_enabled,
  set_colors_enabled,
};
pub use terminal::TerminalSize;
#[cfg(windows)]
pub use windows::enable_ansi_support;

/// Init static once for windows support
static INIT: Once = Once::new();

/// Initializes the terminal for ANSI sequence processing.
///
/// Performs the following checks and setup:
/// 1. Inspects the `NO_COLOR` standard environment
///    variable; if set, disables styling globally.
/// 2. Verifies whether standard output is connected to a
///    terminal (TTY). If redirected to a file or pipe,
///    colors are automatically disabled.
/// 3. On Windows platforms, enables virtual terminal
///    processing (VT100 ANSI escape support).
///
/// Safe to call multiple times; initialization is executed
/// only once.
///
/// # Examples
///
/// ```rust
/// use ciber_term::init_term;
///
/// init_term();
/// ```
pub fn init_term() {
  INIT.call_once(
    || {
      // 1. Check for the NO_COLOR standard
      let no_color_env = std::env::var("NO_COLOR")
        .is_ok_and(
          |v| {
            v.parse::<bool>()
              .unwrap_or(false)
          },
        );

      // 2. Check if stdout is actually a terminal
      let is_tty = std::io::stdout().is_terminal();

      if no_color_env || !is_tty {
        set_colors_enabled(false);
      }
      #[cfg(windows)]
      {
        // Only try to enable Windows ANSI if colors aren't
        // already disabled
        if settings::are_colors_enabled() {
          let _ = windows::enable_ansi_support();
        }
      }
    },
  );
}
