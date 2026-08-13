use {proc_macro::TokenStream, quote::quote};

#[proc_macro_attribute]
pub fn field(attributes: TokenStream, item: TokenStream) -> TokenStream {
    quote! {}.into()
}
