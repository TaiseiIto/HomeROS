use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{
        AngleBracketedGenericArguments, Attribute, Data, DataEnum, DataStruct, DeriveInput, Expr,
        ExprLit, Fields, FieldsUnnamed, GenericArgument, Ident, Lit, Meta, MetaNameValue, Path,
        PathArguments, PathSegment, Type, TypeArray, TypePath, TypeTuple, Variant,
    },
};

pub struct Symbol {
    name: Ident,
    terminal: Option<char>,
    definition: Component,
}

impl From<DeriveInput> for Symbol {
    fn from(symbol: DeriveInput) -> Self {
        let DeriveInput {
            attrs,
            vis: _,
            ident,
            generics: _,
            data,
        } = symbol;
        Self {
            name: ident.clone(),
            terminal: attrs.into_iter().find_map(|attribute| {
                if let Attribute {
                    pound_token: _,
                    style: _,
                    bracket_token: _,
                    meta:
                        Meta::NameValue(MetaNameValue {
                            path,
                            eq_token: _,
                            value:
                                Expr::Lit(ExprLit {
                                    attrs: _,
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
            definition: match data {
                Data::Struct(DataStruct {
                    struct_token: _,
                    fields,
                    semi_token: _,
                }) => match fields {
                    Fields::Unnamed(FieldsUnnamed {
                        paren_token: _,
                        unnamed,
                    }) => Component::Tuple {
                        name: None,
                        elements: unnamed.into_iter().map(|field| field.ty.into()).collect(),
                    },
                    Fields::Unit => Component::Part(ident),
                    fields => panic!("Unknown fields={:#x?}", fields),
                },
                Data::Enum(DataEnum {
                    enum_token: _,
                    brace_token: _,
                    variants,
                }) => Component::Enum(variants.into_iter().map(|variant| variant.into()).collect()),
                data => panic!("Unknown data={:#x?}", data),
            },
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
        size: Expr,
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

impl From<Type> for Component {
    fn from(ty: Type) -> Self {
        match ty {
            Type::Array(TypeArray {
                attrs: _,
                bracket_token: _,
                elem,
                semi_token: _,
                len,
            }) => Self::Array {
                unit: Box::map(elem, |ty| ty.into()),
                size: len,
            },
            Type::Path(TypePath {
                attrs: _,
                qself: _,
                path:
                    Path {
                        leading_colon: _,
                        mut segments,
                    },
            }) => {
                let PathSegment { ident, arguments } = segments.pop().unwrap();
                assert!(segments.is_empty());
                match ident.to_string().as_str() {
                    ident @ ("Box" | "Option" | "Vec") => match arguments {
                        PathArguments::AngleBracketed(AngleBracketedGenericArguments {
                            colon2_token: _,
                            lt_token: _,
                            mut args,
                            gt_token: _,
                        }) => {
                            if let GenericArgument::Type(ty) = args.pop().unwrap() {
                                assert!(args.is_empty());
                                let component: Box<Self> = Box::new(ty.into());
                                match ident {
                                    "Box" => Self::Box(component),
                                    "Option" => Self::Option(component),
                                    "Vec" => Self::Vec(component),
                                    _ => panic!("Unknown wrapper"),
                                }
                            } else {
                                panic!("Unknown generic argument");
                            }
                        }
                        _ => panic!("Unknown arguments"),
                    },
                    _ => Self::Part(ident),
                }
            }
            Type::Tuple(TypeTuple {
                attrs: _,
                paren_token: _,
                elems,
            }) => Self::Tuple {
                name: None,
                elements: elems.into_iter().map(|ty| ty.into()).collect(),
            },
            _ => panic!("Unknown type"),
        }
    }
}

impl From<Variant> for Component {
    fn from(variant: Variant) -> Self {
        if let Variant {
            attrs: _,
            ident,
            fields:
                Fields::Unnamed(FieldsUnnamed {
                    paren_token: _,
                    unnamed,
                }),
            discriminant: _,
        } = variant
        {
            Self::Tuple {
                name: Some(ident),
                elements: unnamed.into_iter().map(|field| field.ty.into()).collect(),
            }
        } else {
            panic!("Unknwon variant");
        }
    }
}
