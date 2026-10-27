/// ANSI color definitions and representations.
pub mod color;

/// Utilities for moving and manipulating the terminal
/// cursor.
pub mod cursor;

/// Utilities for clearing the terminal screen or lines.
pub mod screen;

/// ANSI text formatting styles (e.g., bold, italic).
pub mod style;

/// Deferred formatting structures for optimal string
/// styling.
pub mod styled;

/// Extension traits for adding styling methods to types.
pub mod traits;

pub use color::Color;
pub use cursor::Cursor;
pub use screen::Screen;
pub use style::Style;
pub use styled::StyledText;
pub use traits::StyleExt;
