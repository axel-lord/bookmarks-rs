use super::{any_field::AnyField, field_single::FieldSingle};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

fn gen_simple_display(store_fields: &[Box<dyn AnyField>]) -> TokenStream2 {
    let simple_displays = store_fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.get_simple_display(i));

    quote! {
        #(
            #simple_displays
        )*
    }
}

fn gen_alternate_display(
    display_fields: &[Box<dyn AnyField>],
    title_field: &FieldSingle,
) -> TokenStream2 {
    let alternate_displays = display_fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.get_fancy_display(i));

    let title_display = title_field.get_title_display();

    quote! {
        #title_display

        #(
            #alternate_displays
        )*
    }
}

fn gen_display(
    name: &syn::Ident,
    store_fields: &[Box<dyn AnyField>],
    display_fields: &[Box<dyn AnyField>],
    title_field: &FieldSingle,
) -> TokenStream2 {
    let simple_display = gen_simple_display(store_fields);
    let alternate_display = gen_alternate_display(display_fields, title_field);

    quote! {
        impl std::fmt::Display for #name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if !f.alternate() {
                    #simple_display
                } else {
                    #alternate_display
                }
                Ok(())
            }
        }
    }
}

pub fn display_implementation(
    name: &syn::Ident,
    store_fields: &[Box<dyn AnyField>],
    display_fields: &[Box<dyn AnyField>],
    title_field: &Option<FieldSingle>,
) -> TokenStream2 {
    if let Some(title_field) = title_field {
        gen_display(name, store_fields, display_fields, title_field)
    } else {
        TokenStream2::default()
    }
}
