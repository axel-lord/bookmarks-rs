use crate::storeable::any_field::AnyField;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[derive(Debug, Clone)]
pub struct FieldList {
    pub ident: syn::Ident,
    pub key: TokenStream2,
    pub singular: syn::Ident,
}

impl FieldList {
    fn get_push_ident(&self) -> syn::Ident {
        quote::format_ident!("push_{}", self.singular)
    }
}

impl AnyField for FieldList {
    fn get_key(&self) -> TokenStream2 {
        self.key.clone()
    }

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn get_push_match(&self) -> TokenStream2 {
        let match_str = self.get_ident_string();
        let push_ident = self.get_push_ident();
        quote! {#match_str => self.#push_ident(value),}
    }

    fn get_create_line_param(&self) -> TokenStream2 {
        let ident = self.get_ident();
        quote! {#ident: impl Iterator<Item = impl AsRef<str>>,}
    }

    fn get_create_line_format_param(&self) -> TokenStream2 {
        let token = self.get_key();
        let ident = self.get_ident();
        quote! {
            #token,
            bookmark_storage::join_with_delim(#ident),
        }
    }

    fn get_new_init(&self, line: &syn::Ident) -> TokenStream2 {
        let ident = self.get_ident();
        quote! {#ident: #line.extend(#ident).into(),}
    }

    fn get_set_match(&self) -> TokenStream2 {
        let set_ident = self.get_set_ident();
        let ident_string = self.get_ident_string();

        quote! {(#ident_string, bookmark_storage::Property::List(values)) => {
            self.#set_ident(values.iter());
        }}
    }

    fn get_get_match(&self) -> TokenStream2 {
        let ident = self.get_ident();
        let ident_string = self.get_ident_string();
        quote! {
            #ident_string => {
                bookmark_storage::Property::List(self.#ident().map(String::from).collect())
            }
        }
    }

    fn get_to_line_call(&self) -> TokenStream2 {
        let ident = self.get_ident();
        quote! {self.#ident()}
    }

    fn get_capture_extract(&self, number: usize, line: &syn::Ident) -> TokenStream2 {
        let ident = self.get_ident();
        quote! {
            let group = captures.get(#number).ok_or_else(err)?.range();
            let #ident =
                bookmark_storage::pattern_match::split_list_field(#line.get(group.clone()).unwrap())
                    .map(|f| f + group.start)
                    .collect();
        }
    }

    fn get_fancy_display(&self, _: usize) -> TokenStream2 {
        let ident = self.get_ident();
        let format_string = format!("\n\t{}: ", ident);
        quote! {
            if !self.#ident.is_empty() {
                write!(f, #format_string)?;
                bookmark_storage::write_list_field(f, self.#ident())?;
            }
        }
    }

    fn get_simple_display(&self, index: usize) -> TokenStream2 {
        let ident = self.get_ident();
        let key = self.get_key();

        if index == 0 {
            quote! {
                write!(f, "{} ", #key)?;
                bookmark_storage::write_delim_list(f, self.#ident())?;
            }
        } else {
            quote! {
                write!(f, " {} ", #key)?;
                bookmark_storage::write_delim_list(f, self.#ident())?;
            }
        }
    }

    fn get_field_methods(&self, line: &syn::Ident) -> TokenStream2 {
        let ident = self.get_ident();
        let push_ident = self.get_push_ident();
        let set_ident = self.get_set_ident();
        let single_ident = &self.singular;

        quote! {
            pub fn #ident(&self) -> impl Iterator<Item = &str> {
                self.#ident.get(&self.#line)
            }

            pub fn #set_ident<'a>(
                &mut self,
                #ident: impl Iterator<Item = impl AsRef<str>>,
            ) -> &mut Self {
                self.#ident.clear();

                for item in #ident {
                    self.#ident.push(self.#line.push(item.as_ref()).into());
                }

                self
            }

            pub fn #push_ident(&mut self, #single_ident: &str) -> &mut Self {
                self.#ident.push(self.#line.push(#single_ident).into());

                self
            }
        }
    }
}
