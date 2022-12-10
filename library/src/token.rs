//! Tokens used to parse bookmarks, info and categories.

use paste::paste;
macro_rules! field_idents {
    ($($mod_name:ident {$($field_name:ident),* $(,)?}),* $(,)?) => {
        paste! {
        $(
        #[doc = "Constants for the \"" $mod_name "\" section of an input file."]
        pub mod $mod_name {
            $(
            #[doc = "Prefixed to the\"" $field_name "\" section of a line."]
            pub const [<$field_name:upper>]: &str = concat!("<", stringify!($field_name), ">");
            )*

            #[doc = "Section begins after a line matching this."]
            pub const BEGIN: &str = concat!("#", stringify!([<$mod_name:upper _BEGIN>]));
            #[doc = "Section ends on a line matching this."]
            pub const END: &str = concat!("#", stringify!([<$mod_name:upper _END>]));
        }
        )*
        }
    };
}

field_idents!(
    info { category, tag },
    unsorted { url, info, tag },
    category {
        id,
        desc,
        name,
        identifier,
        sub,
    },
);
