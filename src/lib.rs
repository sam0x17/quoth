//! Quoth is a scannerless parsing library focused on ergonomic, developer-friendly parsing of
//! DSLs and programming languages without a separate lexing step.
//!
//! # Feature flags
//! - `std` *(default)*: enables filesystem/path-aware functionality (e.g. `Source::from_file`,
//!   diagnostics that display file paths) and uses the standard library.
//! - `--no-default-features`: builds Quoth in a `no_std` + `alloc` context. Filesystem/path
//!   helpers are unavailable in this mode, and diagnostics fall back to the provided context name
//!   instead of displaying file paths.
//!
//! All parsers and core data structures are available in both configurations.
#![cfg_attr(not(feature = "std"), no_std)]
//#![deny(missing_docs)]

extern crate alloc;

pub(crate) use alloc::{
    format,
    rc::Rc,
    string::{String, ToString},
    vec::Vec,
};

mod source;
pub use source::*;
mod span;
pub use span::*;
mod diagnostic;
pub use diagnostic::*;
mod parsing;
pub use parsing::*;
pub mod parsable;
pub use quoth_macros::*;
pub use safe_string::*;
