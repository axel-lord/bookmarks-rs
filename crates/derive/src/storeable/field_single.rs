use crate::storeable::any_field::AnyField;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;

#[derive(Debug, Clone)]
pub struct FieldSingle {
    pub ident: syn::Ident,
    pub key: TokenStream2,
}

impl FieldSingle {
    pub fn get_title_display(&self) -> TokenStream2 {
        let ident = &self.ident;
        quote! {
            write!(f, "{}: ", self.#ident())?;
        }
    }

    fn get_get_method(&self, line: &syn::Ident) -> TokenStream2 {
        let ident = &self.ident;
        let doc_str = format!(
            "Get the contents of the {ident} field, roughly equivalent to using get(\"{ident}\")."
        );

        quote! {
            #[doc = #doc_str]
            pub fn #ident(&self) -> &str {
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
            pub fn #set_ident(&mut self, #ident: &str) -> &mut Self {
                self.#ident = self.#line.push(#ident).into();

                self
            }
        }
    }
}

impl AnyField for FieldSingle {
    fn get_key(&self) -> TokenStream2 {
        self.key.clone()
    }

    fn get_ident(&self) -> &syn::Ident {
        &self.ident
    }

    fn get_push_match(&self) -> TokenStream2 {
        Default::default()
    }

    fn get_create_line_param(&self) -> TokenStream2 {
        let ident = &self.ident;
        quote! {#ident: &str,}
    }

    fn get_create_line_format_param(&self) -> TokenStream2 {
        let token = self.key.clone();
        let ident = &self.ident;
        quote! {
            #token,
            #ident,
        }
    }

    fn get_new_init(&self, line: &syn::Ident) -> TokenStream2 {
        let ident = &self.ident;
        quote! {#ident: #line.push(#ident).into(),}
    }

    fn get_set_match(&self) -> TokenStream2 {
        let set_ident = self.get_set_ident();
        let ident_string = self.get_ident_string();

        quote! {(#ident_string, bookmark_storage::Property::Single(value)) => {
            self.#set_ident(&value);
        }}
    }

    fn get_get_match(&self) -> TokenStream2 {
        let ident = &self.ident;
        let ident_string = self.get_ident_string();
        quote! {
            #ident_string => {
                bookmark_storage::Property::Single(self.#ident().into())
            }
        }
    }

    fn get_to_line_call(&self) -> TokenStream2 {
        let ident = &self.ident;
        quote! {&self.#ident()}
    }

    fn get_capture_extract(&self, line: &syn::Ident) -> TokenStream2 {
        let ident = &self.ident;
        quote! {
            let #ident = bookmark_storage::pattern_match::range_trim(&#line, start..end).into();
        }
    }

    fn get_fancy_display(&self, _: usize) -> TokenStream2 {
        let ident = &self.ident;
        let format_string = format!("\n\t{}: {{}}", ident);
        quote! {
            write!(f, #format_string, self.#ident())?;
        }
    }

    fn get_simple_display(&self, index: usize) -> TokenStream2 {
        let ident = &self.ident;
        let key = self.key.clone();

        if index == 0 {
            quote! {
                write!(f, "{} {}", #key, self.#ident())?;
            }
        } else {
            quote! {
                write!(f, " {} {}", #key, self.#ident())?;
            }
        }
    }

    fn get_field_methods(&self, line: &syn::Ident) -> TokenStream2 {
        let set_fn = self.get_set_method(line);
        let get_fn = self.get_get_method(line);

        quote! {
            #set_fn
            #get_fn
        }
    }
}
