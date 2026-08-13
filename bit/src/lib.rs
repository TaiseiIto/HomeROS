use {
    quote::quote,
    syn::{
        ItemStruct,
        parse::{Parse, ParseStream, Result},
        parse_macro_input,
    },
};

#[proc_macro_attribute]
pub fn field(
    _attributes: proc_macro::TokenStream,
    structure: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_struct: ItemStruct = parse_macro_input!(structure as ItemStruct);
    quote! {}.into()
}

struct Element {
    name: Option<String>,
    bits: u8,
}

struct Structure {
    name: String,
    elements: Vec<Element>,
}
