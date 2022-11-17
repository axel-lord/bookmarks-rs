use proc_macro::TokenStream;
use quote::quote;

pub mod any_field;
pub mod conversion_boilerplate;
pub mod display_implementation;
pub mod field_list;
pub mod field_single;
pub mod parse_attr;
pub mod parse_field;
pub mod parse_struct;

use conversion_boilerplate::conversion_boilerplate;
use display_implementation::display_implementation;
use parse_struct::{parse_struct, StructInfo};

pub fn impl_storeable(ast: &syn::DeriveInput) -> TokenStream {
    let StructInfo {
        name,
        line,
        store_fields,
        display_fields,
        title_field,
    } = parse_struct(ast);

    let conversions = conversion_boilerplate(&name);

    let push_matches = store_fields
        .iter()
        .map(|f| f.get_push_match())
        .collect::<Vec<_>>();

    let field_access = store_fields
        .iter()
        .map(|f| f.get_field_methods(&line))
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

    let new_fields = store_fields
        .iter()
        .map(|f| f.get_new_init(&line))
        .collect::<Vec<_>>();

    let set_matches = store_fields
        .iter()
        .map(|f| f.get_set_match())
        .collect::<Vec<_>>();

    let get_matches = store_fields
        .iter()
        .map(|f| f.get_get_match())
        .collect::<Vec<_>>();

    let to_line_calls = store_fields
        .iter()
        .map(|f| f.get_to_line_call())
        .collect::<Vec<_>>();

    let field_names = store_fields
        .iter()
        .map(|f| f.get_ident())
        .collect::<Vec<_>>();

    let keys = store_fields
        .iter()
        .map(|f| {
            let key = f.get_key();
            quote! {
                #key,
                bookmark_storage::pattern_match::WHITESPACE_PADDED_GROUP,
            }
        })
        .collect::<Vec<_>>();

    let capture_extracts = store_fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.get_capture_extract(i + 1, &line))
        .collect::<Vec<_>>();

    let display_implementation =
        display_implementation(&name, &store_fields, &display_fields, &title_field);

    quote! {
        #conversions
        #display_implementation

        impl bookmark_storage::Storeable for #name {
            fn with_string(
                #line: String,
                line_num: Option<usize>,
            ) -> Result<Self, bookmark_storage::ParseErr> {
                use lazy_static::lazy_static;
                lazy_static! {
                    static ref LINE_RE: regex::Regex = regex::Regex::new(
                        &[
                            "^",
                            #(
                                #keys
                            )*
                            "$",
                        ]
                        .concat()
                    )
                    .unwrap();
                }

                let err = || bookmark_storage::ParseErr::Line(Some(#line.clone()), line_num);
                let captures = LINE_RE.captures(&#line).ok_or_else(err)?;

                #(
                    #capture_extracts
                )*

                Ok(Self {
                    #line: #line.into(),
                    #(#field_names,)*
                })
            }
            fn with_str(line: &str, line_num: Option<usize>) -> Result<Self, bookmark_storage::ParseErr> {
                Self::with_string(line.into(), line_num)
            }

            fn to_line(&self) -> String {
                Self::create_line(#(#to_line_calls),*)
            }

            fn is_edited(&self) -> bool {
                self.#line.is_appended_to()
            }

            fn get(
                &self,
                property: &str,
            ) -> Result<bookmark_storage::Property, bookmark_storage::PropertyErr> {
                Ok(match property {
                    #(
                        #get_matches
                    )*
                    _ => return Err(bookmark_storage::PropertyErr::DoesNotExist(property.into())),
                })
            }
            fn set(
                &mut self,
                property: &str,
                value: bookmark_storage::Property,
            ) -> Result<&mut Self, bookmark_storage::PropertyErr> {
                match (property, value) {
                    #(
                        #set_matches
                    )*
                    _ => return Err(bookmark_storage::PropertyErr::DoesNotExist(property.into())),
                };
                Ok(self)
            }

            fn push(
                &mut self,
                property: &str,
                value: &str,
            ) -> Result<&mut Self, bookmark_storage::PropertyErr> {
                match property {
                    #(
                        #push_matches
                    )*
                    _ => return Err(bookmark_storage::PropertyErr::DoesNotExist(property.into())),
                };
                Ok(self)
            }
        }

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
    .into()
}
