use {
    quote::quote,
    syn::{
        Expr, ExprLit, Field, Fields, FieldsNamed, Ident, ItemStruct, Lit, Path, PathSegment, Type,
        TypeArray, TypePath, Visibility, parse_macro_input,
    },
};

#[proc_macro_attribute]
pub fn field(
    _attributes: proc_macro::TokenStream,
    structure: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_struct: ItemStruct = parse_macro_input!(structure as ItemStruct);
    let structure: Structure = item_struct.into();
    let token_stream: proc_macro2::TokenStream = structure.into();
    token_stream.into()
}

struct Element {
    ident: Option<Ident>,
    bits: u8,
}

impl Element {
    fn length(&self) -> Option<proc_macro2::TokenStream> {
        let bits: u8 = self.bits;
        self.length_ident().map(|ident| {
            quote! {
                const #ident: u8 = #bits;
            }
        })
    }

    fn length_ident(&self) -> Option<Ident> {
        self.ident.as_ref().map(|ident| {
            Ident::new(
                &format!("{}_LENGTH", ident.to_string().to_uppercase()),
                ident.span(),
            )
        })
    }

    fn type2bits(ty: Type) -> u8 {
        match ty {
            Type::Array(TypeArray {
                attrs: _,
                bracket_token: _,
                elem,
                semi_token: _,
                len,
            }) => {
                let unit: u8 = Self::type2bits(*elem);
                let len: u8 = if let Expr::Lit(ExprLit {
                    attrs: _,
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
                attrs: _,
                qself: _,
                path:
                    Path {
                        leading_colon: _,
                        segments,
                    },
            }) => {
                let mut segments = segments.iter();
                let segment: &PathSegment = segments.next().unwrap();
                assert!(segments.next().is_none());
                let PathSegment {
                    ident,
                    arguments: _,
                } = segment;
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
            attrs: _,
            vis: _,
            modifiers: _,
            ident: Some(ident),
            colon_token: _,
            ty,
            default: _,
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

impl Structure {
    fn bits(&self) -> u8 {
        self.elements.iter().map(|element| element.bits).sum()
    }

    fn implement(&self) -> proc_macro2::TokenStream {
        let ident: &Ident = &self.ident;
        let length: Vec<proc_macro2::TokenStream> = self
            .elements
            .iter()
            .filter_map(|element| element.length())
            .collect();
        quote! {
            impl #ident {
                #(#length)*
            }
        }
    }

    fn inner_type(&self) -> Ident {
        Ident::new(&format!("u{}", self.bits()), self.ident.span())
    }

    fn true_type(&self) -> proc_macro2::TokenStream {
        let inner_type: Ident = self.inner_type();
        let Self {
            vis,
            ident,
            elements: _,
        } = self;
        quote! {#vis struct #ident(#inner_type);}
    }
}

impl From<ItemStruct> for Structure {
    fn from(item_struct: ItemStruct) -> Self {
        if let ItemStruct {
            attrs: _,
            vis,
            struct_token: _,
            ident,
            generics: _,
            fields:
                Fields::Named(FieldsNamed {
                    brace_token: _,
                    named,
                }),
            semi_token: _,
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

impl From<Structure> for proc_macro2::TokenStream {
    fn from(structure: Structure) -> Self {
        let true_type: proc_macro2::TokenStream = structure.true_type();
        let implement: proc_macro2::TokenStream = structure.implement();
        quote! {
            #true_type
            #implement
        }
    }
}
