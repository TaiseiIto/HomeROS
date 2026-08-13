use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn field(attributes: TokenStream, item: TokenStream) -> TokenStream {
    quote! {}.try_into().unwrap()
}
