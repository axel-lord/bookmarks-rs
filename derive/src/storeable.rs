use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn;

trait AnyField {
    fn get_ident(&self) -> &syn::Ident;
    fn get_key(&self) -> TokenStream2;
    fn get_push_match(&self) -> TokenStream2;
    fn get_field_methods(&self, line: &syn::Ident) -> TokenStream2;
    fn get_create_line_param(&self) -> TokenStream2;
    fn get_create_line_format_param(&self) -> TokenStream2;
    fn get_new_init(&self, line: &syn::Ident) -> TokenStream2;
    fn get_set_match(&self) -> TokenStream2;
    fn get_get_match(&self) -> TokenStream2;
    fn get_to_line_call(&self) -> TokenStream2;
    fn get_capture_extract(&self, number: usize, line: &syn::Ident) -> TokenStream2;

    fn get_ident_string(&self) -> String {
        self.get_ident().to_string()
    }

    fn get_set_ident(&self) -> syn::Ident {
        quote::format_ident!("set_{}", self.get_ident())
    }
}

#[derive(Debug, Clone)]
struct FieldSingle {
    ident: syn::Ident,
    key: TokenStream2,
}

#[derive(Debug, Clone)]
struct FieldList {
    ident: syn::Ident,
    key: TokenStream2,
    singular: syn::Ident,
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
        quote! {#ident: impl Iterator<Item = &'a str>,}
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
        let ident = self.get_ident();
        quote! {#ident: &str,}
    }

    fn get_create_line_format_param(&self) -> TokenStream2 {
        let token = self.get_key();
        let ident = self.get_ident();
        quote! {
            #token,
            #ident,
        }
    }

    fn get_new_init(&self, line: &syn::Ident) -> TokenStream2 {
        let ident = self.get_ident();
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
        let ident = self.get_ident();
        let ident_string = self.get_ident_string();
        quote! {
            #ident_string => {
                bookmark_storage::Property::Single(self.#ident().into())
            }
        }
    }

    fn get_to_line_call(&self) -> TokenStream2 {
        let ident = self.get_ident();
        quote! {&self.#ident()}
    }

    fn get_capture_extract(&self, number: usize, _line: &syn::Ident) -> TokenStream2 {
        let ident = self.get_ident();
        quote! {
            let #ident = captures.get(#number).ok_or_else(err)?.range().into();
        }
    }

    fn get_field_methods(&self, line: &syn::Ident) -> TokenStream2 {
        let ident = self.get_ident();
        let set_ident = self.get_set_ident();

        quote! {
            pub fn #ident(&self) -> &str {
                self.#ident.get(&self.#line)
            }

            pub fn #set_ident(&mut self, #ident: &str) -> &mut Self {
                self.#ident = self.#line.push(#ident).into();

                self
            }
        }
    }
}

#[derive(Clone, Debug)]
enum FieldType {
    Single(FieldSingle),
    List(FieldList),
    Content(syn::Ident),
    Other,
}

enum AttrType {
    Single,
    List { singular: syn::Ident },
    Content,
    Key(TokenStream2),
    Other,
}

fn parse_attr(attr: &syn::Attribute) -> AttrType {
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
                if items.len() != 1 {
                    panic!("composite field should only contain one item")
                }
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
                if items.len() != 1 {
                    panic!("token should contain a single value");
                }

                let syn::NestedMeta::Meta(syn::Meta::Path(ref path)) = items[0] else {
                     panic!("contents of token should be a single token\n{:#?}", items[0]);
                 };

                // let Some(key) = path.get_ident() else {
                //      panic!("contents of token should be a single token\n{:#?}", path);
                // };

                return AttrType::Key(path.clone().to_token_stream());
            }
            _ => return AttrType::Other,
        }
    }

    AttrType::Other
}

fn parse_field(field: &syn::Field) -> FieldType {
    let ident = field
        .ident
        .clone()
        .expect("Storeable should only be derived on structs with named fields");

    let mut attr_type = AttrType::Other;
    let mut key = None;

    for attr in field.attrs.iter() {
        match parse_attr(&attr) {
            AttrType::Content => attr_type = AttrType::Content,
            AttrType::Key(k) => {
                if key.is_none() {
                    key = Some(k);
                } else {
                    panic!("a field may only have one token attribute")
                }
            }
            AttrType::Single => {
                if matches!(attr_type, AttrType::Other) {
                    attr_type = AttrType::Single;
                } else {
                    panic!("field type may be either string or composite")
                }
            }
            AttrType::List { singular } => {
                if matches!(attr_type, AttrType::Other) {
                    attr_type = AttrType::List { singular }
                } else {
                    panic!("field type may be either string or composite")
                }
            }
            AttrType::Other => (),
        }
    }

    if matches!(attr_type, AttrType::Other) {
        return FieldType::Other;
    }

    if matches!(attr_type, AttrType::Content) {
        return FieldType::Content(ident);
    }

    let Some(key) = key else {
        panic!("no token for field {}", ident);
    };

    match attr_type {
        AttrType::Single => FieldType::Single(FieldSingle { ident, key }),
        AttrType::List { singular } => FieldType::List(FieldList {
            ident,
            key,
            singular,
        }),
        _ => std::unreachable!(),
    }
}

pub fn impl_storeable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let mut line = None;
    let mut single_fields = Vec::new();
    let mut list_fields = Vec::new();
    let mut store_fields: Vec<Box<dyn AnyField>> = Vec::new();

    let syn::Data::Struct(ref data) = ast.data else {
        panic!("Storeable should only be derived on structs");
    };

    for field in data.fields.iter() {
        match parse_field(field) {
            FieldType::Content(ident) => {
                if line.is_none() {
                    line = Some(ident);
                    continue;
                } else {
                    panic!("only one line can be marked as line")
                }
            }
            FieldType::Single(field) => {
                single_fields.push(field.clone());
                store_fields.push(Box::new(field));
            }
            FieldType::List(field) => {
                list_fields.push(field.clone());
                store_fields.push(Box::new(field));
            }
            FieldType::Other => continue,
        }
    }

    if line.is_none() {
        panic!("no field tagged line present");
    }

    let line = line.unwrap();

    let conversions = conversion_boilerplate(name);

    let push_matches = store_fields
        .iter()
        .map(|f| f.get_push_match())
        .collect::<Vec<_>>();

    let field_access = store_fields
        .iter()
        .map(|f| f.get_field_methods(&line))
        .collect::<Vec<_>>();

    let create_line_params = store_fields
        .iter()
        .map(|f| f.get_create_line_param())
        .collect::<Vec<_>>();

    let create_line_format_params = store_fields
        .iter()
        .map(|f| f.get_create_line_format_param())
        .collect::<Vec<_>>();

    let create_line_format_string = std::iter::repeat("{}")
        .take(store_fields.len() * 2)
        .collect::<Vec<_>>()
        .join(" ");

    let new_fields = store_fields
        .iter()
        .map(|f| f.get_new_init(&line))
        .collect::<Vec<_>>();

    let set_matches = store_fields
        .iter()
        .map(|f| f.get_set_match())
        .collect::<Vec<_>>();

    let get_matches = store_fields
        .iter()
        .map(|f| f.get_get_match())
        .collect::<Vec<_>>();

    let to_line_calls = store_fields
        .iter()
        .map(|f| f.get_to_line_call())
        .collect::<Vec<_>>();

    let field_names = store_fields
        .iter()
        .map(|f| f.get_ident())
        .collect::<Vec<_>>();

    let keys = store_fields
        .iter()
        .map(|f| {
            let key = f.get_key();
            quote! {
                #key,
                bookmark_storage::pattern_match::WHITESPACE_PADDED_GROUP,
            }
        })
        .collect::<Vec<_>>();

    let capture_extracts = store_fields
        .iter()
        .enumerate()
        .map(|(i, f)| f.get_capture_extract(i + 1, &line))
        .collect::<Vec<_>>();

    quote! {
        #conversions

        impl bookmark_storage::Storeable for #name {
            fn with_string(
                #line: String,
                line_num: Option<usize>,
            ) -> Result<Self, bookmark_storage::ParseErr> {
                use lazy_static::lazy_static;
                lazy_static! {
                    static ref LINE_RE: regex::Regex = regex::Regex::new(
                        &[
                            "^",
                            #(
                                #keys
                            )*
                            "$",
                        ]
                        .concat()
                    )
                    .unwrap();
                }

                let err = || bookmark_storage::ParseErr::Line(Some(#line.clone()), line_num);
                let captures = LINE_RE.captures(&#line).ok_or_else(err)?;

                #(
                    #capture_extracts
                )*

                Ok(Self {
                    #line: #line.into(),
                    #(#field_names,)*
                })
            }
            fn with_str(line: &str, line_num: Option<usize>) -> Result<Self, bookmark_storage::ParseErr> {
                Self::with_string(line.into(), line_num)
            }

            fn to_line(&self) -> String {
                Self::create_line(#(#to_line_calls),*)
            }

            fn is_edited(&self) -> bool {
                self.#line.is_appended_to()
            }

            fn get(
                &self,
                property: &str,
            ) -> Result<bookmark_storage::Property, bookmark_storage::PropertyErr> {
                Ok(match property {
                    #(
                        #get_matches
                    )*
                    _ => return Err(bookmark_storage::PropertyErr::DoesNotExist(property.into())),
                })
            }
            fn set(
                &mut self,
                property: &str,
                value: bookmark_storage::Property,
            ) -> Result<&mut Self, bookmark_storage::PropertyErr> {
                match (property, value) {
                    #(
                        #set_matches
                    )*
                    _ => return Err(bookmark_storage::PropertyErr::DoesNotExist(property.into())),
                };
                Ok(self)
            }

            fn push(
                &mut self,
                property: &str,
                value: &str,
            ) -> Result<&mut Self, bookmark_storage::PropertyErr> {
                match property {
                    #(
                        #push_matches
                    )*
                    _ => return Err(bookmark_storage::PropertyErr::DoesNotExist(property.into())),
                };
                Ok(self)
            }
        }

        impl #name {

            pub fn create_line<'a>(#(#create_line_params)*) -> String {
                format!(#create_line_format_string, #(#create_line_format_params)*)
            }

            pub fn new<'a>(#(#create_line_params)*) -> Self {
                let mut #line = bookmark_storage::content_string::ContentString::new();
                Self {
                    #(#new_fields)*
                    #line,
                }
            }

            #(
                #field_access
            )*
        }

    }
    .into()
}

fn conversion_boilerplate(name: &syn::Ident) -> proc_macro2::TokenStream {
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
                Self::with_str(value, None)
            }
        }

    }
}
