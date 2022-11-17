use proc_macro2::TokenStream as TokenStream2;

pub trait AnyField {
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
    fn get_fancy_display(&self, index: usize) -> TokenStream2;
    fn get_simple_display(&self, index: usize) -> TokenStream2;

    fn get_ident_string(&self) -> String {
        self.get_ident().to_string()
    }

    fn get_set_ident(&self) -> syn::Ident {
        quote::format_ident!("set_{}", self.get_ident())
    }
}
