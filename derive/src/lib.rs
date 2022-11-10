use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(BuildCommand)]
pub fn build_command_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_build_command(&ast)
}

fn impl_build_command(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let syn::Data::Struct(data_struct) = &ast.data else {
        panic!("only works for stucts");
    };

    let fields: Vec<_> = data_struct.fields.iter().collect();
    let field_idents: Vec<_> = data_struct
        .fields
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect();

    let gen = quote! {
        impl #name {
             pub fn build(#(#fields),*) -> Box<Self>{
                Box::new(Self { #(#field_idents),* })
             }
        }
    };

    gen.into()
}
