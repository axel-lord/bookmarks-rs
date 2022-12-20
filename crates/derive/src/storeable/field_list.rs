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

    fn get_push_method(&self, line: &syn::Ident) -> TokenStream2 {
        let ident = &self.ident;
        let single_ident = &self.singular;
        let push_ident = self.get_push_ident();
        let doc_str = format!("Push a value to the contents of the {ident} field, roughly equivalent to using push(\"{single_ident}\", {single_ident}).");

        quote! {
            #[doc = #doc_str]
            pub fn #push_ident(&mut self, #single_ident: &str) -> &mut Self {
                self.#ident.push(self.#line.push(#single_ident).into());

                self
            }
        }
    }

    fn get_get_method(&self, line: &syn::Ident) -> TokenStream2 {
        let ident = &self.ident;
        let doc_str = format!(
            "Get the contents of the {ident} field, roughly equivalent to using get(\"{ident}\")."
        );

        quote! {
            #[doc = #doc_str]
            pub fn #ident(&self) -> impl DoubleEndedIterator<Item = &str> {
                self.#ident.get(&self.#line)
            }
        }
    }

    fn get_set_method(&self, line: &syn::Ident) -> TokenStream2 {
        let ident = &self.ident;
        let set_ident = self.get_set_ident();
        let doc_str = format!("Set the contents of the {ident} field, roughly equivalent to using set(\"{ident}\", {ident}).");

        quote! {
            #[doc = #doc_str]
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
        }
    }
}

impl AnyField for FieldList {
    fn get_capture_extract(&self, line: &syn::Ident) -> TokenStream2 {
        let ident = &self.ident;
        quote! {
            let #ident = bookmark_storage::pattern_match::split_list_field(&#line[start..end])
                .map(|f| f + start)
                .collect();
        }
    }

    fn get_create_line_format_param(&self) -> TokenStream2 {
        let token = self.key.clone();
        let ident = &self.ident;
        quote! {
            #token,
            bookmark_storage::pattern_match::join_with_delim(#ident),
        }
    }

    fn get_create_line_param(&self) -> TokenStream2 {
        let ident = &self.ident;
        quote! {#ident: impl Iterator<Item = impl AsRef<str>>,}
    }

    fn get_fancy_display(&self, _: usize) -> TokenStream2 {
        let ident = &self.ident;
        let format_string = format!("\n\t{}: ", ident);
        quote! {
            if !self.#ident.is_empty() {
                write!(f, #format_string)?;
                bookmark_storage::pattern_match::write_list_field(f, self.#ident())?;
            }
        }
    }

    fn get_field_methods(&self, line: &syn::Ident) -> TokenStream2 {
        let get_fn = self.get_get_method(line);
        let set_fn = self.get_set_method(line);
        let push_fn = self.get_push_method(line);

        quote! {
            #get_fn
            #set_fn
            #push_fn
        }
    }

    fn get_get_match(&self) -> TokenStream2 {
        let ident = &self.ident;
        let ident_string = self.get_ident_string();
        quote! {
            #ident_string => {
                bookmark_storage::Property::List(self.#ident().map(String::from).collect())
            }
        }
    }

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn get_key(&self) -> TokenStream2 {
        self.key.clone()
    }

    fn get_new_init(&self, line: &syn::Ident) -> TokenStream2 {
        let ident = &self.ident;
        quote! {#ident: #line.extend(#ident).into(),}
    }

    fn get_push_match(&self) -> TokenStream2 {
        let match_str = self.get_ident_string();
        let push_ident = self.get_push_ident();
        quote! {#match_str => self.#push_ident(value),}
    }

    fn get_set_match(&self) -> TokenStream2 {
        let set_ident = self.get_set_ident();
        let ident_string = self.get_ident_string();

        quote! {(#ident_string, bookmark_storage::Property::List(values)) => {
            self.#set_ident(values.iter());
        }}
    }

    fn get_simple_display(&self, index: usize) -> TokenStream2 {
        let ident = &self.ident;
        let key = self.key.clone();

        if index == 0 {
            quote! {
                write!(f, "{} ", #key)?;
                bookmark_storage::pattern_match::write_delim_list(f, self.#ident())?;
            }
        } else {
            quote! {
                write!(f, " {} ", #key)?;
                bookmark_storage::pattern_match::write_delim_list(f, self.#ident())?;
            }
        }
    }

    fn get_to_line_call(&self) -> TokenStream2 {
        let ident = &self.ident;
        quote! {self.#ident()}
    }
}
