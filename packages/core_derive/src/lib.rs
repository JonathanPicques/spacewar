use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(RollbackEvent)]
pub fn derive_rollback_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let type_name = input.ident;

    TokenStream::from(quote! {
        impl core::event::events::RollbackEvent for #type_name {}
    })
}
