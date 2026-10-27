use std::sync::atomic::{
  AtomicBool,
  Ordering,
};

/// Global flag determining if ANSI colors should be
/// printed. Defaults to `true`, but can be disabled via the
/// `NO_COLOR` environment variable.
static COLORS_ENABLED: AtomicBool = AtomicBool::new(true);

/// Checks if ANSI styling is currently enabled globally.
///
/// Returns `false` if `NO_COLOR` was set, if stdout is not
/// a TTY, or if disabled manually via
/// [`set_colors_enabled`].
///
/// # Examples
///
/// ```rust
/// use ciber_term::are_colors_enabled;
///
/// if are_colors_enabled() {
///   println!("Colors are active!");
/// }
/// ```
#[inline]
pub fn are_colors_enabled() -> bool {
  COLORS_ENABLED.load(Ordering::Relaxed)
}

/// Manually enable or disable colors globally (useful for
/// testing or user overrides).
///
/// # Examples
///
/// ```rust
/// use ciber_term::{are_colors_enabled, set_colors_enabled};
///
/// set_colors_enabled(false);
/// assert!(!are_colors_enabled());
///
/// set_colors_enabled(true);
/// assert!(are_colors_enabled());
/// ```
pub fn set_colors_enabled(enabled: bool) {
  COLORS_ENABLED.store(
    enabled,
    Ordering::Relaxed,
  );
}
