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
