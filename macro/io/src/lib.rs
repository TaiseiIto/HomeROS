mod overlap;
mod register;
mod registers;

use syn::{ItemStruct, ItemUnion, parse_macro_input};

#[proc_macro_attribute]
pub fn register(
    _attributes: proc_macro::TokenStream,
    structure: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_struct: ItemStruct = parse_macro_input!(structure as ItemStruct);
    let structure: register::Structure = item_struct.into();
    let token_stream: proc_macro2::TokenStream = structure.into();
    token_stream.into()
}

#[proc_macro_attribute]
pub fn registers(
    _attributes: proc_macro::TokenStream,
    structure: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_struct: ItemStruct = parse_macro_input!(structure as ItemStruct);
    let structure: registers::Structure = item_struct.into();
    let token_stream: proc_macro2::TokenStream = structure.into();
    token_stream.into()
}

#[proc_macro_attribute]
pub fn overlap(
    _attributes: proc_macro::TokenStream,
    overlap_registers: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_union: ItemUnion = parse_macro_input!(overlap_registers as ItemUnion);
    let overlap_registers: overlap::Registers = item_union.into();
    let token_stream: proc_macro2::TokenStream = overlap_registers.into();
    token_stream.into()
}
