//! Derive macros used by project.

#![warn(
    missing_copy_implementations,
    missing_docs,
    clippy::missing_errors_doc,
    clippy::missing_panics_doc,
    clippy::missing_safety_doc,
    clippy::unwrap_used,
    clippy::pedantic,
    rustdoc::missing_crate_level_docs
)]

use proc_macro::TokenStream;

mod build_command;
mod storeable;

#[proc_macro_derive(Command)]
/// Derive a Command build by passing the struct fields as arguments.
///
/// # Panics
/// If the struct is malformed.
pub fn build_command_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("could not parse struct");

    build_command::implementation(&ast)
}

/// Derive a storeable implementation for a macro.
///
/// # Panics
/// If the struct is malformed.
#[proc_macro_derive(Storeable, attributes(line, string, composite, token, title))]
pub fn storeable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).expect("could not parse struct");

    storeable::implementation(&ast)
}
