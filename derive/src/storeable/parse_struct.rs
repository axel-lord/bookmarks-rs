use super::{
    any_field::AnyField,
    field_single::FieldSingle,
    parse_field::{parse_field, FieldType},
};

pub struct StructInfo {
    pub name: syn::Ident,
    pub line: syn::Ident,
    pub store_fields: Vec<Box<dyn AnyField>>,
    pub display_fields: Vec<Box<dyn AnyField>>,
    pub title_field: Option<FieldSingle>,
}

pub fn parse_struct(ast: &syn::DeriveInput) -> StructInfo {
    let name = ast.ident.clone();

    let mut line = None;
    let mut store_fields: Vec<Box<dyn AnyField>> = Vec::new();

    let mut display_fields: Vec<Box<dyn AnyField>> = Vec::new();
    let mut title_field = None;

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
            FieldType::Single(field, is_title) => {
                if is_title {
                    if title_field.is_some() {
                        panic!("only one field may be marked as title");
                    }
                    title_field = Some(field.clone());
                } else {
                    display_fields.push(Box::new(field.clone()));
                }
                store_fields.push(Box::new(field));
            }
            FieldType::List(field) => {
                store_fields.push(Box::new(field.clone()));
                display_fields.push(Box::new(field.clone()));
            }
            FieldType::Other => continue,
        }
    }

    if line.is_none() {
        panic!("no field tagged line present");
    }

    let line = line.unwrap();

    StructInfo {
        name,
        line,
        store_fields,
        display_fields,
        title_field,
    }
}
