#![feature(smart_pointer_try_map)]

mod parser;

use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Parser, attributes(terminal))]
pub fn parser(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(input as DeriveInput);
    let input: parser::Symbol = input.into();
    let token_stream: proc_macro2::TokenStream = input.into();
    token_stream.into()
}
