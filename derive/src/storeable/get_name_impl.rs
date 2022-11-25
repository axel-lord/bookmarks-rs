use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::any_field::AnyField;

fn gen_field_access(line: &syn::Ident, store_fields: &[Box<dyn AnyField>]) -> TokenStream2 {
    let field_access = store_fields.iter().map(|f| f.get_field_methods(line));

    quote! {
        #(
            #field_access
        )*
    }
}

fn gen_new(
    line: &syn::Ident,
    store_fields: &[Box<dyn AnyField>],
    params: &Vec<TokenStream2>,
) -> TokenStream2 {
    let new_fields = store_fields.iter().map(|f| f.get_new_init(line));

    quote! {
        pub fn new<'a>(#(#params)*) -> Self {
            let mut #line = bookmark_storage::content_string::ContentString::new();
            Self {
                #(#new_fields)*
                #line,
            }
        }
    }
}

fn gen_create_line(
    store_fields: &Vec<Box<dyn AnyField>>,
    params: &Vec<TokenStream2>,
) -> TokenStream2 {
    let format_params = store_fields
        .iter()
        .map(|f| f.get_create_line_format_param());

    let format_string = std::iter::repeat("{}")
        .take(store_fields.len() * 2)
        .collect::<Vec<_>>()
        .join(" ");

    quote! {
        pub fn create_line<'a>(#(#params)*) -> String {
            format!(#format_string, #(#format_params)*)
        }
    }
}

pub fn gen_name_impl(
    name: &syn::Ident,
    line: &syn::Ident,
    store_fields: &Vec<Box<dyn AnyField>>,
) -> TokenStream2 {
    let params = store_fields
        .iter()
        .map(|f| f.get_create_line_param())
        .collect::<Vec<_>>();

    let create_line_fn = gen_create_line(store_fields, &params);
    let new_fn = gen_new(line, store_fields, &params);
    let field_access = gen_field_access(line, store_fields);

    quote! {
        impl #name {
            #create_line_fn
            #new_fn
            #field_access
        }
    }
}
