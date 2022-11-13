use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(BuildCommand)]
pub fn build_command_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_build_command(&ast)
}

#[proc_macro_derive(Storeable, attributes(line, string, composite))]
pub fn storeable_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_storeable(&ast)
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

fn impl_storeable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    //panic!("{:#?}", ast);

    let syn::Data::Struct(ref data_struct) = ast.data else {
        panic!("Storeable can only be derived on structs");
    };

    let mut line = None;
    let mut strings = Vec::new();
    let mut composites = Vec::new();

    for field in data_struct.fields.iter() {
        if field.ident.is_none() {
            panic!("macro should be used on structs with bamed fields");
        }

        for attr in field.attrs.iter() {
            let Ok(meta) = attr.parse_meta() else {
                panic!("{:#?}", attr);
            };

            match meta {
                syn::Meta::Path(ref path) => {
                    let Some(ident) = path.get_ident() else {
                        panic!("attribute should be a single token");
                    };

                    match ident.to_string().as_str() {
                        "line" => line = Some(field.clone()),
                        "string" => strings.push(field.ident.clone().unwrap()),
                        _ => panic!("only string and list supported in this context"),
                    }
                }
                syn::Meta::List(ref list) => {
                    let Some(ident) = list.path.get_ident() else {
                        panic!("attribute should be a single token"); 
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

                            composites.push((of_ident.clone(), field.ident.clone().unwrap()));
                        }
                        _ => panic!("only composite supported in this context"),
                    }
                }
                _ => panic!("string, list, and composite supported"),
            }
        }
    }

    let Some(line) = line else {
        panic!("could not find member annotated with line");
    };

    let line_ident = line.ident.unwrap();
    let (comp_of, comp): (Vec<_>, Vec<_>) = composites.into_iter().unzip();

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

        }
        impl #name {
            fn raw_line(&self) -> &str {
                self.#line_ident.as_ref().unwrap().ref_any()
            }

            #(
                fn #comp_of(&self) -> &str {
                    &self.raw_line()[self.#comp_of.clone()]
                }
            )*

            #(
            pub fn #strings(&self) -> &str {
                &self.raw_line()[self.#strings.clone()]
            }
            )*

            #(
                pub fn #comp(&self) -> impl Iterator<Item = &str> {
                    self.tags.iter().map(|r| &self.#comp_of()[r.clone()])
                }
            )*
        }
    };

    gen.into()
}
