use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{Attribute, DeriveInput, Expr, ExprLit, Ident, Lit, Meta, MetaNameValue},
};

pub struct Symbol {
    name: Ident,
    terminal: Option<char>,
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
        Self {
            name: ident,
            terminal: attrs.into_iter().find_map(|attribute| {
                if let Attribute {
                    pound_token,
                    style,
                    bracket_token,
                    meta:
                        Meta::NameValue(MetaNameValue {
                            path,
                            eq_token,
                            value:
                                Expr::Lit(ExprLit {
                                    attrs,
                                    lit: Lit::Char(lit_char),
                                }),
                        }),
                } = attribute
                    && path.is_ident("terminal")
                {
                    Some(lit_char.value())
                } else {
                    None
                }
            }),
        }
    }
}

impl From<Symbol> for TokenStream {
    fn from(symbol: Symbol) -> Self {
        quote! {}
    }
}

pub enum Component {
    Array {
        unit: Box<Component>,
        size: usize,
    },
    Box(Box<Component>),
    Enum(Vec<Component>),
    Option(Box<Component>),
    Part(Ident),
    Tuple {
        name: Option<Ident>,
        elements: Vec<Component>,
    },
    Vec(Box<Component>),
}
