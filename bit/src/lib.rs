use {
    quote::quote,
    syn::{
        Expr, ExprLit, Field, Fields, FieldsNamed, Ident, ItemStruct, Lit, LitInt, Path,
        PathSegment, Type, TypeArray, TypePath, Visibility,
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
    ident: Option<Ident>,
    bits: u8,
}

impl Element {
    fn type2bits(ty: Type) -> u8 {
        match ty {
            Type::Array(TypeArray {
                attrs,
                bracket_token,
                elem,
                semi_token,
                len,
            }) => {
                let unit: u8 = Self::type2bits(*elem);
                let len: u8 = if let Expr::Lit(ExprLit {
                    attrs,
                    lit: Lit::Int(lit_int),
                }) = len
                {
                    lit_int.base10_parse::<u8>().unwrap()
                } else {
                    panic!();
                };
                len * unit
            }
            Type::Path(TypePath {
                attrs,
                qself,
                path:
                    Path {
                        leading_colon,
                        segments,
                    },
            }) => {
                let mut segments = segments.iter();
                let segment: &PathSegment = segments.next().unwrap();
                assert!(segments.next().is_none());
                let PathSegment { ident, arguments } = segment;
                assert!(ident == "bool");
                1
            }
            _ => panic!(),
        }
    }
}

impl From<Field> for Element {
    fn from(field: Field) -> Self {
        if let Field {
            attrs,
            vis,
            modifiers,
            ident: Some(ident),
            colon_token,
            ty,
            default,
        } = field
        {
            let ident: Option<Ident> = (ident != "reserved").then_some(ident);
            let bits: u8 = Self::type2bits(ty);
            Self { ident, bits }
        } else {
            panic!();
        }
    }
}

struct Structure {
    vis: Visibility,
    ident: Ident,
    elements: Vec<Element>,
}

impl From<ItemStruct> for Structure {
    fn from(item_struct: ItemStruct) -> Self {
        if let ItemStruct {
            attrs,
            vis,
            struct_token,
            ident,
            generics,
            fields: Fields::Named(FieldsNamed { brace_token, named }),
            semi_token,
        } = item_struct
        {
            let elements: Vec<Element> = named.into_iter().map(|field| field.into()).collect();
            Self {
                vis,
                ident,
                elements,
            }
        } else {
            panic!();
        }
    }
}
