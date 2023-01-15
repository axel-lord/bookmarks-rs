use proc_macro2::TokenStream as TokenStream2;
use quote::ToTokens;

#[derive(Clone, Debug)]
pub enum AttrType {
    Single,
    List { singular: syn::Ident },
    Content,
    Key(TokenStream2),
    Title,
    Other,
}

pub fn parse_attr(attr: &syn::Attribute) -> AttrType {
    let Ok(meta) = attr.parse_meta() else {
        return AttrType::Other;
    };

    if let syn::Meta::Path(ref path) = meta {
        let Some(ident) = path.get_ident() else {
            return AttrType::Other;
        };

        match ident.to_string().as_str() {
            "line" => return AttrType::Content,
            "string" => return AttrType::Single,
            "title" => return AttrType::Title,
            _ => return AttrType::Other,
        }
    }

    if let syn::Meta::List(ref list) = meta {
        let Some(ident) = list.path.get_ident() else {
            return AttrType::Other;
        };

        match ident.to_string().as_str() {
            "composite" => {
                let items = list.nested.iter().collect::<Vec<_>>();
                assert!(
                    items.len() == 1,
                    "composite field should only contain one item"
                );
                let syn::NestedMeta::Meta(syn::Meta::Path(ref path)) = items[0] else {
                     panic!("contents of composite should be a single token");
                 };

                let Some(singular) = path.get_ident() else {
                     panic!("contents of composite should be a single token");
                 };
                let singular = singular.clone();

                return AttrType::List { singular };
            }
            "token" => {
                let items = list.nested.iter().collect::<Vec<_>>();
                assert!(items.len() == 1, "token should contain a single value");

                let syn::NestedMeta::Meta(syn::Meta::Path(ref path)) = items[0] else {
                     panic!("contents of token should be a single token\n{:#?}", items[0]);
                 };

                return AttrType::Key(path.clone().to_token_stream());
            }
            _ => return AttrType::Other,
        }
    }

    AttrType::Other
}
