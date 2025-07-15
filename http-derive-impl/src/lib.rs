use proc_macro::TokenStream;
use quote::quote;

#[proc_macro_derive(HttpStatus, attributes(http))]
pub fn http_status_impl(tokens: TokenStream) -> TokenStream {
    quote! {}.into()
}
