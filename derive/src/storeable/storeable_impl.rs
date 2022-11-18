use super::any_field::AnyField;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

pub fn gen_storeable_impl(
    name: &syn::Ident,
    line: &syn::Ident,
    store_fields: &Vec<Box<dyn AnyField>>,
) -> TokenStream2 {
    let regex_parts = store_fields
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

    let field_names = store_fields
        .iter()
        .map(|f| f.get_ident())
        .collect::<Vec<_>>();

    let to_line_calls = store_fields
        .iter()
        .map(|f| f.get_to_line_call())
        .collect::<Vec<_>>();

    let push_matches = store_fields
        .iter()
        .map(|f| f.get_push_match())
        .collect::<Vec<_>>();

    let set_matches = store_fields
        .iter()
        .map(|f| f.get_set_match())
        .collect::<Vec<_>>();

    let get_matches = store_fields
        .iter()
        .map(|f| f.get_get_match())
        .collect::<Vec<_>>();

    quote! {
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
                                #regex_parts
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
    }
}
