use quote::quote;

pub fn conversion_boilerplate(name: &syn::Ident) -> proc_macro2::TokenStream {
    quote! {
        impl From<#name> for String {
            fn from(value: #name) -> Self {
                use bookmark_storage::Storeable;
                value.to_line()
            }
        }
        impl std::convert::TryFrom<String> for #name {
            type Error = bookmark_storage::ParseErr;
            fn try_from(value: String) -> Result<Self, Self::Error> {
                use bookmark_storage::Storeable;
                Self::with_string(value, None)
            }
        }
        impl std::convert::TryFrom<&str> for #name {
            type Error = bookmark_storage::ParseErr;
            fn try_from(value: &str) -> Result<Self, Self::Error> {
                use bookmark_storage::Storeable;
                Self::with_string(value, None)
            }
        }

    }
}
