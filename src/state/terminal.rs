use std::io;

/// Represents the dimensions of the terminal window.
///
/// # Examples
///
/// ```rust,no_run
/// use ciber_term::TerminalSize;
///
/// if let Ok(size) = TerminalSize::get() {
///   println!("Terminal: {}x{}", size.width(), size.height());
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TerminalSize {
  width:  u16,
  height: u16,
}

impl TerminalSize {
  /// Queries the terminal size.
  ///
  /// # Safety
  ///
  /// This function is safe to call because the underlying
  /// FFI calls (`ioctl` on Unix or
  /// `GetConsoleScreenBufferInfo` on Windows)
  /// are standard system calls that do not rely on
  /// undefined state provided by the caller.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::TerminalSize;
  ///
  /// let size = TerminalSize::get()?;
  /// println!("Width: {}, Height: {}", size.width(), size.height());
  /// # Ok::<(), std::io::Error>(())
  /// ```
  ///
  /// # Errors
  /// Returns an `io::Result` if the terminal size cannot be
  /// determined.
  #[inline]
  pub fn get() -> io::Result<Self> {
    #[cfg(windows)]
    {
      use windows_sys::Win32::System::Console::{
        CONSOLE_SCREEN_BUFFER_INFO,
        GetConsoleScreenBufferInfo,
        GetStdHandle,
        STD_OUTPUT_HANDLE,
      };
      #[allow(unsafe_code)]
      unsafe {
        let handle = GetStdHandle(STD_OUTPUT_HANDLE);
        let mut info: CONSOLE_SCREEN_BUFFER_INFO =
          std::mem::zeroed();
        if GetConsoleScreenBufferInfo(
          handle,
          &mut info,
        ) == 0
        {
          return Err(io::Error::last_os_error());
        }
        Ok(
          Self {
            width:  (info
              .srWindow
              .Right -
              info
                .srWindow
                .Left +
              1) as u16,
            height: (info
              .srWindow
              .Bottom -
              info
                .srWindow
                .Top +
              1) as u16,
          },
        )
      }
    }

    #[cfg(unix)]
    {
      use std::mem::MaybeUninit;

      use libc::{
        STDOUT_FILENO,
        TIOCGWINSZ,
        ioctl,
        winsize,
      };

      let mut w = MaybeUninit::<winsize>::uninit();
      #[allow(unsafe_code)]
      let res = unsafe {
        ioctl(
          STDOUT_FILENO,
          TIOCGWINSZ,
          w.as_mut_ptr(),
        )
      };
      if res == 0 {
        #[allow(unsafe_code)]
        let size = unsafe { w.assume_init() };
        Ok(
          Self {
            width:  size.ws_col,
            height: size.ws_row,
          },
        )
      }
      else {
        Err(io::Error::last_os_error())
      }
    }
  }

  /// Returns the width of the terminal window in columns.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::TerminalSize;
  ///
  /// let size = TerminalSize::get()?;
  /// println!("Columns: {}", size.width());
  /// # Ok::<(), std::io::Error>(())
  /// ```
  #[must_use]
  #[inline]
  pub const fn width(&self) -> u16 { self.width }

  /// Returns the height of the terminal window in rows.
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use ciber_term::TerminalSize;
  ///
  /// let size = TerminalSize::get()?;
  /// println!("Rows: {}", size.height());
  /// # Ok::<(), std::io::Error>(())
  /// ```
  #[must_use]
  #[inline]
  pub const fn height(&self) -> u16 { self.height }
}
