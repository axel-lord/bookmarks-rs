use proc_macro::TokenStream;
use quote::quote;

pub mod any_field;
pub mod conversion_boilerplate;
pub mod display_implementation;
pub mod field_list;
pub mod field_single;
pub mod get_name_impl;
pub mod parse_attr;
pub mod parse_field;
pub mod parse_struct;
pub mod storeable_impl;

use conversion_boilerplate::conversion_boilerplate;
use display_implementation::display_implementation;
use get_name_impl::gen_name_impl;
use parse_struct::{parse_struct, StructInfo};
use storeable_impl::gen_storeable_impl;

pub fn impl_storeable(ast: &syn::DeriveInput) -> TokenStream {
    let StructInfo {
        name,
        line,
        store_fields,
        display_fields,
        title_field,
    } = parse_struct(ast);

    let conversions = conversion_boilerplate(&name);

    let display_implementation =
        display_implementation(&name, &store_fields, &display_fields, &title_field);

    let storeable_impl = gen_storeable_impl(&name, &line, &store_fields);

    let name_impl = gen_name_impl(&name, &line, &store_fields);

    quote! {
        #conversions
        #display_implementation
        #name_impl
        #storeable_impl
    }
    .into()
}
