use super::{any_field::AnyField, field_single::FieldSingle};
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn display_implementation(
    name: &syn::Ident,
    store_fields: &Vec<Box<dyn AnyField>>,
    display_fields: &Vec<Box<dyn AnyField>>,
    title_field: &Option<FieldSingle>,
) -> TokenStream2 {
    if let Some(title_field) = title_field {
        let simple_displays = store_fields
            .iter()
            .enumerate()
            .map(|(i, f)| f.get_simple_display(i))
            .collect::<Vec<_>>();

        let fancy_displays = display_fields
            .iter()
            .enumerate()
            .map(|(i, f)| f.get_fancy_display(i))
            .collect::<Vec<_>>();

        let title_display = title_field.get_title_display();

        quote! {
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    if !f.alternate() {
                        #(#simple_displays)*
                    } else {
                        #title_display

                        #(#fancy_displays)*
                    }
                    Ok(())
                }
            }
        }
    } else {
        Default::default()
    }
}
