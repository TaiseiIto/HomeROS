mod table;

use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Table)]
pub fn table(structure: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let structure: DeriveInput = parse_macro_input!(structure as DeriveInput);
    let structure: table::Structure = structure.into();
    let token_stream: proc_macro2::TokenStream = structure.into();
    token_stream.into()
}
