use super::any_field::AnyField;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

fn gen_set(store_fields: &Vec<Box<dyn AnyField>>) -> TokenStream2 {
    let set_matches = store_fields.iter().map(|f| f.get_set_match());

    quote! {
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
    }
}

fn gen_get(store_fields: &Vec<Box<dyn AnyField>>) -> TokenStream2 {
    let get_matches = store_fields.iter().map(|f| f.get_get_match());

    quote! {
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
    }
}

fn gen_push(store_fields: &Vec<Box<dyn AnyField>>) -> TokenStream2 {
    let push_matches = store_fields.iter().map(|f| f.get_push_match());

    quote! {
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

fn gen_with_string(line: &syn::Ident, store_fields: &Vec<Box<dyn AnyField>>) -> TokenStream2 {
    let capture_extracts = store_fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.get_capture_extract(i + 1, &line));

    let field_names = store_fields.iter().map(|f| f.get_ident());

    let regex_parts = store_fields.iter().map(|f| {
        let key = f.get_key();
        quote! {
            #key,
            bookmark_storage::pattern_match::WHITESPACE_PADDED_GROUP,
        }
    });

    quote! {
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
    }
}

fn gen_to_line(store_fields: &Vec<Box<dyn AnyField>>) -> TokenStream2 {
    let to_line_calls = store_fields.iter().map(|f| f.get_to_line_call());

    quote! {
        fn to_line(&self) -> String {
            Self::create_line(#(#to_line_calls),*)
        }
    }
}

fn gen_is_edited(line: &syn::Ident) -> TokenStream2 {
    quote! {
        fn is_edited(&self) -> bool {
            self.#line.is_appended_to()
        }
    }
}

pub fn gen_storeable_impl(
    name: &syn::Ident,
    line: &syn::Ident,
    store_fields: &Vec<Box<dyn AnyField>>,
) -> TokenStream2 {
    let with_string_fn = gen_with_string(line, store_fields);
    let to_line_fn = gen_to_line(store_fields);
    let is_edited_fn = gen_is_edited(line);

    let set_fn = gen_set(store_fields);
    let get_fn = gen_get(store_fields);
    let push_fn = gen_push(store_fields);

    quote! {
        impl bookmark_storage::Storeable for #name {
            #is_edited_fn
            #to_line_fn
            #with_string_fn
            #set_fn
            #get_fn
            #push_fn
        }
    }
}
