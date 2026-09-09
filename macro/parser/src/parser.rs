use {proc_macro2::TokenStream, quote::quote, syn::DeriveInput};

pub struct Symbol;

impl From<DeriveInput> for Symbol {
    fn from(symbol: DeriveInput) -> Self {
        Self
    }
}

impl From<Symbol> for TokenStream {
    fn from(symbol: Symbol) -> Self {
        quote! {}
    }
}
