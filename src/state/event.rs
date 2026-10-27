use std::{
  io::{
    self,
    Read,
  },
  time::Duration,
};

/// Represents terminal events.
///
/// # Examples
///
/// ```rust
/// use ciber_term::prelude::*;
///
/// let event = Event::Key(KeyEvent { code: KeyCode::Enter });
/// match event {
///   Event::Key(k) => println!("Key: {:?}", k.code),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event {
  /// A key event.
  Key(KeyEvent),
}

/// Represents keyboard events.
///
/// # Examples
///
/// ```rust
/// use ciber_term::prelude::*;
///
/// let key = KeyEvent { code: KeyCode::Char('q') };
/// assert_eq!(key.code, KeyCode::Char('q'));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
  /// The key code.
  pub code: KeyCode,
}

/// Represents keyboard key codes.
///
/// # Examples
///
/// ```rust
/// use ciber_term::prelude::*;
///
/// let code = KeyCode::Up;
/// match code {
///   KeyCode::Up => println!("Up arrow!"),
///   _ => {},
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyCode {
  /// Character key.
  Char(char),
  /// Enter key.
  Enter,
  /// Backspace key.
  Backspace,
  /// Escape key.
  Esc,
  /// Up arrow key.
  Up,
  /// Down arrow key.
  Down,
  /// Left arrow key.
  Left,
  /// Right arrow key.
  Right,
  /// Unknown key.
  Unknown,
}

/// Checks if there is an event waiting to be read.
/// Returns `true` if input is available, or `false` if the
/// timeout expires.
///
/// # Examples
///
/// ```rust,no_run
/// use ciber_term::prelude::*;
/// use std::time::Duration;
///
/// if poll(Duration::from_millis(50))? {
///   let event = read()?;
///   println!("Event: {:?}", event);
/// }
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// # Errors
///
/// Returns an `io::Result` if an error occurs while polling
/// for an event.
pub fn poll(timeout: Duration) -> io::Result<bool> {
  #[cfg(unix)]
  {
    use libc::{
      POLLIN,
      STDIN_FILENO,
      poll,
      pollfd,
    };

    let mut pfd = pollfd {
      fd:      STDIN_FILENO,
      events:  POLLIN,
      revents: 0,
    };

    let timeout_ms = i32::try_from(timeout.as_millis())
      .map_err(
        |_| {
          io::Error::new(
            io::ErrorKind::InvalidInput,
            "Timeout too large",
          )
        },
      )?;

    #[allow(unsafe_code)]
    // SAFETY: poll is a safe system call when provided with
    // a valid polled struct.
    let ret = unsafe {
      poll(
        &raw mut pfd,
        1,
        timeout_ms,
      )
    };

    if ret < 0 {
      Err(io::Error::last_os_error())
    }
    else {
      Ok(ret > 0)
    }
  }

  #[cfg(windows)]
  {
    use windows_sys::Win32::{
      Foundation::{
        WAIT_OBJECT_0,
        WAIT_TIMEOUT,
      },
      System::{
        Console::{
          GetStdHandle,
          STD_INPUT_HANDLE,
        },
        Threading::WaitForSingleObject,
      },
    };

    let timeout_ms = timeout.as_millis() as u32;

    #[allow(unsafe_code)]
    // SAFETY: We are getting the standard input handle and
    // waiting on it.
    let res = unsafe {
      let handle = GetStdHandle(STD_INPUT_HANDLE);
      WaitForSingleObject(
        handle,
        timeout_ms,
      )
    };

    match res {
      | WAIT_OBJECT_0 => Ok(true),
      | WAIT_TIMEOUT => Ok(false),
      | _ => Err(io::Error::last_os_error()),
    }
  }
}

/// Reads an event from `stdin`.
/// This function blocks, so it should only be called AFTER
/// `poll()` returns `true`.
///
/// # Examples
///
/// ```rust,no_run
/// use ciber_term::prelude::*;
/// use std::time::Duration;
///
/// if poll(Duration::from_millis(50))? {
///   let event = read()?;
///   match event {
///     Event::Key(k) => println!("Key: {:?}", k.code),
///   }
/// }
/// # Ok::<(), std::io::Error>(())
/// ```
///
/// # Errors
///
/// Returns an `io::Result` if an error occurs while reading
/// from `stdin`.
pub fn read() -> io::Result<Event> {
  let mut stdin = io::stdin();
  let mut buffer = [0u8; 1];

  // Read exactly 1 byte
  stdin.read_exact(&mut buffer)?;

  match buffer[0] {
    | b'\x1b' => {
      let mut seq = [0u8; 2];
      // Try to read 2 more bytes (ANSI escape sequence)
      if stdin
        .read_exact(&mut seq)
        .is_err()
      {
        return Ok(
          Event::Key(
            KeyEvent {
              code: KeyCode::Esc,
            },
          ),
        );
      }

      if seq[0] == b'[' {
        match seq[1] {
          | b'A' => Ok(
            Event::Key(
              KeyEvent {
                code: KeyCode::Up,
              },
            ),
          ),
          | b'B' => Ok(
            Event::Key(
              KeyEvent {
                code: KeyCode::Down,
              },
            ),
          ),
          | b'C' => Ok(
            Event::Key(
              KeyEvent {
                code: KeyCode::Right,
              },
            ),
          ),
          | b'D' => Ok(
            Event::Key(
              KeyEvent {
                code: KeyCode::Left,
              },
            ),
          ),
          | _ => Ok(
            Event::Key(
              KeyEvent {
                code: KeyCode::Unknown,
              },
            ),
          ),
        }
      }
      else {
        Ok(
          Event::Key(
            KeyEvent {
              code: KeyCode::Esc,
            },
          ),
        )
      }
    },
    | b'\n' | b'\r' => Ok(
      Event::Key(
        KeyEvent {
          code: KeyCode::Enter,
        },
      ),
    ),
    | 127 | 8 => Ok(
      Event::Key(
        KeyEvent {
          code: KeyCode::Backspace,
        },
      ),
    ),
    | c => Ok(
      Event::Key(
        KeyEvent {
          code: KeyCode::Char(c as char),
        },
      ),
    ),
  }
}
