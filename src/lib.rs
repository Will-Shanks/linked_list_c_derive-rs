use proc_macro::TokenStream;
use quote::quote;
use syn;

#[allow(non_snake_case)]
#[proc_macro_derive(LlItem)]
pub fn LlItem_derive(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl linked_list_c::LlItem for #name {
            fn get_next(&self) -> *mut Self {
                self.next
            }
            fn set_next(&mut self, next: *mut Self) -> Option<*mut Self> {
                let old = self.next;
                self.next = next;
                if !old.is_null() {
                    Some(old)
                } else {
                    None
                }
            }
        }
    };
    TokenStream::from(expanded)
}
