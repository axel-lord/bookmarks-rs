use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{self, Token};

pub fn implementation(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let syn::Data::Struct(data_struct) = &ast.data else {
        panic!("only works for stucts");
    };

    let (generic, generic_simple, where_clause) = 'parse_gen: {
        use syn::punctuated::Punctuated;

        let empty: (TokenStream2, TokenStream2, TokenStream2) = Default::default();
        let generics = &ast.generics;

        let (Some(ref lt), Some(ref gt)) = (generics.lt_token, generics.gt_token) else {
            break 'parse_gen empty;
        };

        let mut simple: Punctuated<_, Token![,]> = Punctuated::new();

        for g in generics.params.iter() {
            let g = match g {
                syn::GenericParam::Type(syn::TypeParam { ident, .. }) => {
                    syn::GenericParam::Type(syn::TypeParam {
                        ident: ident.clone(),
                        attrs: Vec::new(),
                        colon_token: None,
                        bounds: Punctuated::new(),
                        eq_token: None,
                        default: None,
                    })
                }
                syn::GenericParam::Lifetime(syn::LifetimeDef { lifetime, .. }) => {
                    syn::GenericParam::Lifetime(syn::LifetimeDef {
                        lifetime: lifetime.clone(),
                        attrs: Vec::new(),
                        colon_token: None,
                        bounds: Punctuated::new(),
                    })
                }
                syn::GenericParam::Const(_) => unimplemented!("not supported"),
            };
            simple.push(g);
        }

        let params = &generics.params;
        let w_clause = &generics.where_clause;

        (
            quote! {#lt #params #gt},
            quote! {#lt #simple #gt},
            w_clause.to_token_stream(),
        )
    };

    let fields: Vec<_> = data_struct.fields.iter().collect();
    let field_idents: Vec<_> = data_struct
        .fields
        .iter()
        .map(|f| {
            f.ident
                .clone()
                .expect("failed to get identifier for a field")
        })
        .collect();

    let gen = quote! {
        impl #generic #name #generic_simple #where_clause {
             pub fn build(#(#fields),*) -> Box<Self>{
                Box::new(Self { #(#field_idents),* })
             }
        }
    };

    gen.into()
}
