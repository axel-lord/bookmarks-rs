use proc_macro::TokenStream;

mod build_command;
mod storeable;

#[proc_macro_derive(Command)]
pub fn build_command_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    build_command::impl_build_command(&ast)
}

#[proc_macro_derive(Storeable, attributes(line, string, composite, token, title))]
pub fn storeable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    storeable::impl_storeable(&ast)
}
