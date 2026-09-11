use {
    proc_macro2::{Span, TokenStream},
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

impl Symbol {
    fn implement(&self) -> TokenStream {
        let name: &Ident = &self.name;
        let parse: TokenStream = self.parse();
        quote! {
            impl #name {
                #parse
            }
        }
    }

    fn parse(&self) -> TokenStream {
        let value: TokenStream = self.value();
        quote! {
            fn parse(string: &str) -> Option<(Self, &str)> {
                #value
            }
        }
    }

    fn value(&self) -> TokenStream {
        let Self {
            name: _,
            terminal,
            definition,
        } = self;
        if let Some(terminal) = terminal
            && let Component::Part(ident) = definition
        {
            quote! {
                let mut string: ::core::str::Chars<'_> = string.chars();
                string
                    .next()
                    .and_then(|character| (character == #terminal)
                        .then_some((Self, string.as_str())))
            }
        } else {
            definition.value()
        }
    }
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
                        name: Some(ident.clone()),
                        elements: unnamed.into_iter().map(|field| field.ty.into()).collect(),
                    },
                    Fields::Unit => Component::Part(ident),
                    fields => panic!("Unknown fields={:#x?}", fields),
                },
                Data::Enum(DataEnum {
                    enum_token: _,
                    brace_token: _,
                    variants,
                }) => Component::Enum {
                    name: ident.clone(),
                    variants: variants.into_iter().map(|variant| variant.into()).collect(),
                },
                data => panic!("Unknown data={:#x?}", data),
            },
        }
    }
}

impl From<Symbol> for TokenStream {
    fn from(symbol: Symbol) -> Self {
        let implement: TokenStream = symbol.implement();
        quote! {
            #implement
        }
    }
}

pub enum Component {
    Array {
        unit: Box<Component>,
        size: Expr,
    },
    Box(Box<Component>),
    Enum {
        name: Ident,
        variants: Vec<Component>,
    },
    Option(Box<Component>),
    Part(Ident),
    Tuple {
        name: Option<Ident>,
        elements: Vec<Component>,
    },
    Vec(Box<Component>),
}

impl Component {
    fn ty(&self) -> TokenStream {
        match self {
            Self::Array { unit, size } => {
                let unit: TokenStream = unit.ty();
                quote! { [#unit; #size] }
            }
            Self::Box(component) => {
                let component: TokenStream = component.ty();
                quote! { Box<#component> }
            }
            Self::Enum { name, variants } => quote! { #name },
            Self::Option(component) => {
                let component: TokenStream = component.ty();
                quote! { Option<#component> }
            }
            Self::Part(ty) => quote! { #ty },
            Self::Tuple { name, elements } => {
                if let Some(name) = name {
                    quote! { #name }
                } else {
                    let elements: Vec<TokenStream> =
                        elements.iter().map(|element| element.ty()).collect();
                    quote! { (#(#elements),*) }
                }
            }
            Self::Vec(component) => {
                let component: TokenStream = component.ty();
                quote! { Vec<#component> }
            }
        }
    }

    fn value(&self) -> TokenStream {
        let ty: TokenStream = self.ty();
        match self {
            Self::Array { unit, size } => {
                let value: TokenStream = unit.value();
                let unit_type: TokenStream = unit.ty();
                quote! {
                    {
                        let mut symbols: Vec<#unit_type> = Vec::new();
                        let mut string: &str = string;
                        for _ in 0..#size {
                            if let Some((symbol, remaining_string)) = #value {
                                symbols.push(symbol);
                                string = remaining_string;
                            }
                        }
                        symbols
                            .try_into()
                            .ok()
                            .map(|symbols| (symbols, string))
                    }
                }
            }
            Self::Box(component) => {
                let value: TokenStream = component.value();
                quote! {
                    #value.map(|(symbol, string)| (Box::new(symbol), string))
                }
            }
            Self::Enum { name, variants } => {
                let variants: Vec<TokenStream> = variants
                    .iter()
                    .map(|variant| {
                        if let Self::Tuple { name, elements } = variant {
                            let (lets, symbols): (Vec<TokenStream>, Vec<Ident>) = elements
                                .iter()
                                .enumerate()
                                .map(|(index, element)| {
                                    let symbol: Ident =
                                        Ident::new(&format!("symbol{}", index), Span::call_site());
                                    let element_value: TokenStream = element.value();
                                    let let_statement: TokenStream = quote! {
                                        let Some((#symbol, string)) = #element_value
                                    };
                                    (let_statement, symbol)
                                })
                                .unzip();
                            quote! {
                                if #(#lets)&&* {
                                    Some((Self::#name(#(#symbols),*), string))
                                }
                            }
                        } else {
                            panic!();
                        }
                    })
                    .collect();
                quote! {
                    #(#variants)else* else { None }
                }
            }
            Self::Option(component) => {
                let value: TokenStream = component.value();
                quote! {
                    Some(if let Some((symbol, string)) = #value {
                        (Some(symbol), string)
                    } else {
                        (None, string)
                    })
                }
            }
            Self::Part(ident) => quote! { #ident::parse(string) },
            Self::Tuple { name, elements } => {
                let (lets, symbols): (Vec<TokenStream>, Vec<Ident>) = elements
                    .iter()
                    .enumerate()
                    .map(|(index, element)| {
                        let symbol: Ident =
                            Ident::new(&format!("symbol{}", index), Span::call_site());
                        let element_value: TokenStream = element.value();
                        let let_statement: TokenStream = quote! {
                            let Some((#symbol, string)) = #element_value
                        };
                        (let_statement, symbol)
                    })
                    .unzip();
                quote! {
                    if #(#lets)&&* {
                        Some((#name(#(#symbols),*), string))
                    } else {
                        None
                    }
                }
            }
            Self::Vec(component) => {
                let value: TokenStream = component.value();
                quote! {
                    {
                        let mut symbols: #ty = Vec::new();
                        let mut string = string;
                        while let Some((symbol, remaining_string)) = #value {
                            symbols.push(symbol);
                            string = remaining_string;
                        }
                        Some((symbols, string))
                    }
                }
            }
        }
    }
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
