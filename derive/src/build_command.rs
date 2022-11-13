use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::{self, Token};

pub fn impl_build_command(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    // if name.to_string().as_str() == "Load" {
    //     println!("Load ast:\n{:#?}", ast);
    // }

    let syn::Data::Struct(data_struct) = &ast.data else {
        panic!("only works for stucts");
    };

    let (generic, generic_simple, where_clause) = 'parse_gen: {
        let empty: (TokenStream2, TokenStream2, TokenStream2) = Default::default();
        let generics = &ast.generics;

        let (Some(ref lt), Some(ref gt)) = (generics.lt_token, generics.gt_token) else {
            break 'parse_gen empty;
        };

        use syn::punctuated::Punctuated;

        let mut simple: Punctuated<_, Token![,]> = Punctuated::new();

        for g in generics.params.iter() {
            let g = match g {
                syn::GenericParam::Type(syn::TypeParam { ident, .. }) => syn::TypeParam {
                    ident: ident.clone(),
                    attrs: Vec::new(),
                    colon_token: None,
                    bounds: Punctuated::new(),
                    eq_token: None,
                    default: None,
                },
                _ => panic!("todo: implement"),
            };
            simple.push(g);
        }

        let params = &generics.params;
        let w_clause = &generics.where_clause;

        (
            quote! {#lt #params #gt},
            quote! {#lt #simple #gt},
            w_clause.to_token_stream().into(),
        )
    };

    // println!(
    //     "{} {} {}",
    //     generic.to_string(),
    //     generic_simple.to_string(),
    //     where_clause.to_string()
    // );

    let fields: Vec<_> = data_struct.fields.iter().collect();
    let field_idents: Vec<_> = data_struct
        .fields
        .iter()
        .map(|f| f.ident.clone().unwrap())
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
