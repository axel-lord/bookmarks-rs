use crate::{bookmark::Bookmark, category::Category, info::Info, shared};

macro_rules! reset_values_create {
    ($name:ident, $($field_ident:ident: $field_ty:ty),*) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $($field_ident: shared::BufferStorage<$field_ty>,)*
        }

        impl $name {
            pub fn new($($field_ident: shared::BufferStorage<$field_ty>),*) -> Self {
                Self {$($field_ident,)*}
            }
            pub fn reset(&self) {
                $(self.$field_ident.reset();)*
            }
        }
    };
}

reset_values_create!(
    ResetValues,
    infos: Info,
    categories: Category,
    bookmarks: Bookmark
);
