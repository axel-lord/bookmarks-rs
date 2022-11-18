use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

use super::any_field::AnyField;

pub fn gen_name_impl(
    name: &syn::Ident,
    line: &syn::Ident,
    store_fields: &Vec<Box<dyn AnyField>>,
) -> TokenStream2 {
    let new_fields = store_fields
        .iter()
        .map(|f| f.get_new_init(&line))
        .collect::<Vec<_>>();

    let create_line_params = store_fields
        .iter()
        .map(|f| f.get_create_line_param())
        .collect::<Vec<_>>();

    let create_line_format_params = store_fields
        .iter()
        .map(|f| f.get_create_line_format_param())
        .collect::<Vec<_>>();

    let create_line_format_string = std::iter::repeat("{}")
        .take(store_fields.len() * 2)
        .collect::<Vec<_>>()
        .join(" ");

    let field_access = store_fields
        .iter()
        .map(|f| f.get_field_methods(&line))
        .collect::<Vec<_>>();

    quote! {
        impl #name {

            pub fn create_line<'a>(#(#create_line_params)*) -> String {
                format!(#create_line_format_string, #(#create_line_format_params)*)
            }

            pub fn new<'a>(#(#create_line_params)*) -> Self {
                let mut #line = bookmark_storage::content_string::ContentString::new();
                Self {
                    #(#new_fields)*
                    #line,
                }
            }

            #(
                #field_access
            )*
        }
    }
}
