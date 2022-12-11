//! Crate used for definitions of items needed to serialize and deserialize bookmarks,
//! categories and info.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    rustdoc::missing_crate_level_docs
)]

mod content_string;
mod field;
mod list_field;
mod load;
mod parse_err;
mod property;
mod save;
mod section;
mod storeable;

/// Helpers for pattern matching.
pub mod pattern_match;

/// Constants used for saving and loading.
pub mod token;

pub use bookmark_derive::Storeable;
pub use content_string::ContentString;
pub use field::Field;
pub use list_field::ListField;
pub use load::load;
pub use load::load_from;
pub use parse_err::ParseErr;
pub use property::{Property, PropertyErr};
pub use save::save;
pub use section::Section;
pub use storeable::Storeable;

/// Trait for types that imlement both [Section] ans [Storeable].
pub trait Listed: Storeable + Section {}

impl<T> Listed for T where T: Storeable + Section {}
