/// Trait used to mark a type as serializable in sections.
pub trait Section {
    /// Name of the type.
    const ITEM_NAME: &'static str;
    /// Content of the line signaling the beginning of the section.
    const TOKEN_BEGIN: &'static str;
    /// Content of the line signaling the end of the section.
    const TOKEN_END: &'static str;
}
