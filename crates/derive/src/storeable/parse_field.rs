use crate::storeable::{
    field_list::FieldList,
    field_single::FieldSingle,
    parse_attr::{parse_attr, AttrType},
};

#[derive(Clone, Debug)]
pub enum FieldType {
    Single(FieldSingle, bool),
    List(FieldList),
    Content(syn::Ident),
    Other,
}

pub fn parse_field(field: &syn::Field) -> FieldType {
    let ident = field
        .ident
        .clone()
        .expect("Storeable should only be derived on structs with named fields");

    let mut attr_type = AttrType::Other;
    let mut key = None;
    let mut is_title = false;

    for attr in &field.attrs {
        match parse_attr(attr) {
            AttrType::Content => attr_type = AttrType::Content,
            AttrType::Key(k) => {
                if key.is_none() {
                    key = Some(k);
                } else {
                    panic!("a field may only have one token attribute")
                }
            }
            AttrType::Title => {
                is_title = true;
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

    assert!(
        matches!(attr_type, AttrType::Single) || !is_title,
        "title may only be specified on a field marked as string"
    );

    if matches!(attr_type, AttrType::Other) {
        return FieldType::Other;
    }

    if matches!(attr_type, AttrType::Content) {
        return FieldType::Content(ident);
    }

    let Some(key) = key else {
        panic!("no token for field {ident}");
    };

    match attr_type {
        AttrType::Single => FieldType::Single(FieldSingle { ident, key }, is_title),
        AttrType::List { singular } => FieldType::List(FieldList {
            ident,
            key,
            singular,
        }),
        _ => std::unreachable!(),
    }
}
