#![feature(string_into_chars)]

use {
    quote::quote,
    regex::{Automaton, Capture, Match},
    std::collections::BTreeMap,
    syn::{LitStr, parse_macro_input},
};

#[proc_macro]
pub fn guid(guid: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let guid: String = parse_macro_input!(guid as LitStr).value();
    let automaton: Automaton = r"^(?<data1>[\da-f]{8})-(?<data2>[\da-f]{4})-(?<data3>[\da-f]{4})-(?<data4prefix>[\da-f]{4})-(?<data4suffix>[\da-f]{12})$".parse().unwrap();
    let matches: Vec<Match> = automaton.input(&guid);
    let mat: Match = matches.into_iter().next().unwrap();
    let captures: BTreeMap<String, Vec<Capture>> = mat.captures();
    let data1: u32 =
        u32::from_str_radix(&captures["data1"].first().unwrap().to_string(), 16).unwrap();
    let data2: u16 =
        u16::from_str_radix(&captures["data2"].first().unwrap().to_string(), 16).unwrap();
    let data3: u16 =
        u16::from_str_radix(&captures["data3"].first().unwrap().to_string(), 16).unwrap();
    let data4prefix: Vec<char> = captures["data4prefix"]
        .first()
        .unwrap()
        .to_string()
        .into_chars()
        .collect();
    let data4prefix: Vec<u8> = data4prefix
        .chunks(2)
        .map(|byte| u8::from_str_radix(&format!("{}{}", byte[0], byte[1]), 16).unwrap())
        .collect();
    let data4prefix: [u8; 2] = data4prefix.try_into().unwrap();
    let data4suffix: Vec<char> = captures["data4suffix"]
        .first()
        .unwrap()
        .to_string()
        .into_chars()
        .collect();
    let data4suffix: Vec<u8> = data4suffix
        .chunks(2)
        .map(|byte| u8::from_str_radix(&format!("{}{}", byte[0], byte[1]), 16).unwrap())
        .collect();
    let data4suffix: [u8; 6] = data4suffix.try_into().unwrap();
    let data4: Vec<u8> = data4prefix.into_iter().chain(data4suffix).collect();
    quote! {
        crate::Guid {
            data1: #data1,
            data2: #data2,
            data3: #data3,
            data4: [ #(#data4),* ],
        }
    }
    .into()
}
