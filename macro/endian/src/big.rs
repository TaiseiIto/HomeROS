use {proc_macro2::TokenStream, quote::quote, syn::DeriveInput};

pub struct Structure {}

impl From<DeriveInput> for Structure {
    fn from(structure: DeriveInput) -> Self {
        Self {}
    }
}

impl From<Structure> for TokenStream {
    fn from(structure: Structure) -> Self {
        quote! {}
    }
}
