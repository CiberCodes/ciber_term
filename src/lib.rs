//! # Ciber Term
//!
//! A high-performance, zero-allocation terminal styling and
//! manipulation library designed as the foundational
//! bedrock for the `CiberCodes` ecosystem.
//!
//! This crate provides an intuitive and elegant API for
//! ANSI terminal colors, text styles, cursor movement, and
//! screen clearing.
//!
//! ## Quick Start
//!
//! Simply import the `StyleExt` trait to seamlessly style
//! any type that implements `std::fmt::Display`.
//!
//! ```rust
//! use ciber_term::StyleExt;
//!
//! // Style strings natively
//! println!("{}", "Warning: High memory usage!".yellow().bold().blink());
//!
//! // Style any type seamlessly
//! let age: i32 = 25;
//! println!("You are {} years old.", age.green().underline());
//! ```

// Enables docs.rs features to show tags like "Only on
// Windows"
#![cfg_attr(
  docsrs,
  feature(doc_cfg)
)]
// Ensures all public items are documented (Essential for
// crates.io)
#![warn(missing_docs)]
// Prevents accidental unsafe code in the entire crate
#![deny(unsafe_code)]

/// ANSI control structures (Colors, Styles, .etc.).
pub mod control;

/// Terminal lifecycle and environment utilities.
pub mod state;

/// Re-export all public items.
pub mod prelude;

pub use prelude::*;
