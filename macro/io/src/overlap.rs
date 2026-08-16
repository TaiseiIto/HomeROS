use {proc_macro2::TokenStream, quote::quote, syn::ItemUnion};

pub struct Registers {}

impl From<ItemUnion> for Registers {
    fn from(item_union: ItemUnion) -> Self {
        Self {}
    }
}

impl From<Registers> for TokenStream {
    fn from(registers: Registers) -> Self {
        quote! {}
    }
}
