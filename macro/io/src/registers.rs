use {proc_macro2::TokenStream, quote::quote, syn::ItemStruct};

pub struct Structure {}

impl From<ItemStruct> for Structure {
    fn from(item_struct: ItemStruct) -> Self {
        Self {}
    }
}

impl From<Structure> for TokenStream {
    fn from(structure: Structure) -> Self {
        quote! {}
    }
}
