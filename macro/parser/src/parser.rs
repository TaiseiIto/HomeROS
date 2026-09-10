use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{DeriveInput, Ident},
};

pub struct Symbol {
    name: Ident,
}

impl From<DeriveInput> for Symbol {
    fn from(symbol: DeriveInput) -> Self {
        let DeriveInput {
            attrs,
            vis,
            ident,
            generics,
            data,
        } = symbol;
        Self { name: ident }
    }
}

impl From<Symbol> for TokenStream {
    fn from(symbol: Symbol) -> Self {
        quote! {}
    }
}
