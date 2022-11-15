use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn;

pub fn impl_storeable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    #[derive(Clone, Copy, Debug)]
    enum TokenType {
        TString,
        TComp,
    }

    use TokenType::*;

    let syn::Data::Struct(ref data_struct) = ast.data else {
        panic!("Storeable can only be derived on structs");
    };

    let mut line = None;
    let mut strings = Vec::new();
    let mut composites = HashMap::new();

    let mut all = Vec::new();
    let mut tokens = HashMap::new();

    let mut field_order = Vec::new();

    for field in data_struct.fields.iter() {
        if field.ident.is_none() {
            panic!("macro should be used on structs with named fields");
        }

        field_order.push(field.ident.clone().unwrap());

        for attr in field.attrs.iter() {
            let Ok(meta) = attr.parse_meta() else {
                panic!("{:#?}", attr);
            };

            match meta {
                syn::Meta::Path(ref path) => {
                    let Some(ident) = path.get_ident() else {
                        panic!("attribute should be a single token\n{:#?}", path);
                    };

                    match ident.to_string().as_str() {
                        "line" => line = Some(field.clone()),
                        "string" => {
                            strings.push(field.ident.clone().unwrap());
                            all.push((field.ident.clone().unwrap(), TString));
                        }
                        _ => panic!(
                            "only string and list supported in this context\n{:#?}",
                            path
                        ),
                    }
                }
                syn::Meta::List(ref list) => {
                    let Some(ident) = list.path.get_ident() else {
                        panic!("attribute should be a single token\n{:#?}", list); 
                    };

                    match ident.to_string().as_str() {
                        "composite" => {
                            let items: Vec<_> = list.nested.iter().collect();
                            if items.len() != 1 {
                                panic!("composite should contain a single value");
                            }

                            let syn::NestedMeta::Meta(syn::Meta::Path(ref path)) = items[0] else {
                                panic!("contents of composite should be a single token");
                            };

                            let Some(of_ident) = path.get_ident() else {
                                panic!("contents of composite should be a single token");
                            };

                            composites.insert(field.ident.clone().unwrap(), of_ident.clone());
                            all.push((field.ident.clone().unwrap(), TComp));
                        }
                        "token" => {
                            let items: Vec<_> = list.nested.iter().collect();
                            if items.len() != 1 {
                                panic!("token should contain a single value");
                            }

                            let syn::NestedMeta::Meta(syn::Meta::Path(ref path)) = items[0] else {
                                panic!("contents of token should be a single token");
                            };

                            tokens.insert(field.ident.clone().unwrap(), path.clone());
                        }
                        _ => panic!(
                            "only composite and token supported in this context\n{:#?}",
                            list
                        ),
                    }
                }
                _ => panic!("string, list, and composite supported\n{:#?}", meta),
            }
        }

        if all.len() != tokens.len() {
            panic!("there should be a token on every field that is storeable")
        }
    }

    let Some(line) = line else {
        panic!("could not find member annotated with line");
    };

    let line_ident = line.ident.unwrap();
    let (comp, comp_of): (Vec<_>, Vec<_>) = composites.iter().unzip();

    let re_contents: Vec<_> = all
        .iter()
        .map(|(i, _)| {
            let tok = &tokens[i];
            quote! {
                #tok,
                bookmark_storage::pattern_match::WHITESPACE_PADDED_GROUP,
            }
        })
        .collect();

    let build_fields: Vec<_> = field_order
        .iter()
        .map(|i| {
            if i == &line_ident {
                quote! {line: Some(bookmark_storage::content_string::ContentString::UnappendedTo(#i))}
            } else {
                quote! {#i}
            }
        })
        .collect();

    let create_line_format = std::iter::repeat("{} {}")
        .take(all.len())
        .collect::<Vec<_>>()
        .join(" ");

    let create_line_iter: Vec<_> = all
        .iter()
        .map(|(i, t)| {
            [
                tokens[i].to_token_stream(),
                match t {
                    TString => i.to_token_stream(),
                    TComp => {
                        quote! {
                            #i.collect::<Vec<&str>>()
                                .join(&[" ", bookmark_storage::token::DELIM, " "].concat())
                        }
                    }
                },
            ]
            .into_iter()
        })
        .flatten()
        .collect();

    let parse_fields: Vec<_> = all
        .iter()
        .enumerate()
        .map(|(i, (id, ty))| {
            let c = i + 1;
            match ty {
                TComp => {
                    let of_id = &composites[id];
                    quote! {
                        let #of_id = captures.get(#c).ok_or_else(err)?.range();
                        let #id = bookmark_storage::pattern_match::split_by_delim_to_ranges(&line[#of_id.clone()]);
                    }
                }
                TString => {
                    quote! {let #id = captures.get(#c).ok_or_else(err)?.range();}
                }
            }
        })
        .collect();

    let all_simple: Vec<_> = all.iter().map(|(i, _)| i).collect();

    let new_args: Vec<_> = all
        .iter()
        .map(|(i, t)| match t {
            TString => quote! {
                #i: &str
            },
            TComp => quote! {
                #i: impl Iterator<Item = &'a str>
            },
        })
        .collect();

    let adders: Vec<_> = comp_of
        .iter()
        .map(|i| format_ident!("{}_push", i))
        .collect();

    let get_fields: Vec<_> = strings.iter().map(|s| s.to_string()).collect();
    let get_list_fields: Vec<_> = comp.iter().map(|s| s.to_string()).collect();

    let string_setters: Vec<_> = strings.iter().map(|i| format_ident!("set_{i}")).collect();
    let comp_setters: Vec<_> = comp.iter().map(|i| format_ident!("set_{i}")).collect();

    // --- //
    // gen //
    // --- //

    let gen = quote! {
        impl Clone for #name {
            fn clone(&self) -> Self {
                Self::with_string(self.to_line(), None).unwrap()
            }
        }
        impl From<#name> for String {
            fn from(c: #name) -> Self {
                c.to_line()
            }
        }
        impl bookmark_storage::Storeable for #name {
            fn is_edited(&self) -> bool {
                self.#line_ident.as_ref().unwrap().is_appended_to()
            }

            fn with_string(line: String, line_num: Option<usize>) -> Result<Self, bookmark_storage::ParseErr> {
                use lazy_static::lazy_static;
                lazy_static! {
                    static ref LINE_RE: regex::Regex = regex::Regex::new(
                        &[
                            r#"^"#,
                            #(
                            #re_contents
                            )*
                            r"$",
                        ]
                        .concat()
                    )
                    .unwrap();
                }

                let err = || bookmark_storage::ParseErr::Line(Some(line.clone()), line_num);

                let captures = LINE_RE.captures(&line).ok_or_else(err)?;

                #(
                #parse_fields
                )*

                Ok(Self{
                    #(
                    #build_fields
                    ),*
                })
            }
            fn with_str(
                line: &str,
                line_num: Option<usize>,
            ) -> Result<Self, bookmark_storage::ParseErr> {
                Self::with_string(line.into(), line_num)
            }
            fn to_line(&self) -> String {
                if let Some(bookmark_storage::content_string::ContentString::UnappendedTo(line)) = self.line.as_ref() {
                    line.clone()
                } else {
                    Self::create_line(#(self.#all_simple()),*)
                }
            }

            fn get(&self, property: &str) -> Result<bookmark_storage::Property, bookmark_storage::PropertyErr> {
                match property {
                #(
                    #get_fields => Ok(bookmark_storage::Property::Single(self.#strings().into())),
                )*
                #(
                    #get_list_fields => Ok(bookmark_storage::Property::List(self.#comp().map(String::from).collect())),
                )*
                    _ => Err(bookmark_storage::PropertyErr::DoesNotExist(property.into()))
                }
            }

        }
        impl #name {
            pub fn new<'a>(#(#new_args),*) -> Self {
                Self::with_string(Self::create_line(#(#all_simple),*), None).unwrap()
            }

            pub fn create_line<'a>(#(#new_args),*) -> String {
                format!(
                    #create_line_format,
                    #(
                    #create_line_iter
                    ),*
                )
            }

            fn raw_line(&self) -> &str {
                self.#line_ident.as_ref().unwrap().ref_any()
            }

            #(
                fn #comp_of(&self) -> &str {
                    &self.raw_line()[self.#comp_of.clone()]
                }
                pub fn #comp(&self) -> impl Iterator<Item = &str> {
                    self.#comp.iter().map(|r| &self.#comp_of()[r.clone()])
                }
                pub fn #adders(&mut self, #comp_of: &str) {
                    let (content_string, range) = self.#line_ident.take().unwrap().append(#comp_of);

                    self.#line_ident = Some(content_string);
                    self.#comp.push(range);
                }
            )*

            #(
                pub fn #strings(&self) -> &str {
                    &self.raw_line()[self.#strings.clone()]
                }
            )*

            #(
                pub fn #string_setters(&mut self, value: &str) {
                    let (content_string, range) = self.#line_ident.take().unwrap().append(value);

                    self.#line_ident = Some(content_string);
                    self.#strings = range;
                }
            )*

            #(
                pub fn #comp_setters<'a>(&mut self, value: impl Iterator<Item = &'a str>) {
                    self.#comp.clear();
                    for item in value {
                        self.#adders(item);
                    }
                }
            )*
        }
    };

    gen.into()
}
