use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{
        Field, FieldsNamed, Ident, ItemUnion, Path, PathSegment, Type, TypePath, Visibility,
        punctuated::Punctuated, token::PathSep,
    },
};

pub struct Registers {
    elements: Vec<Element>,
    ident: Ident,
    vis: Visibility,
}

impl Registers {
    fn pretty_type(&self) -> TokenStream {
        let Self {
            elements,
            ident: _,
            vis,
        } = self;
        let pretty_type: Ident = self.pretty_type_ident();
        let pretty_elements: Vec<TokenStream> = elements
            .iter()
            .map(|element| element.pretty_declaration())
            .collect();
        quote! {
            #vis struct #pretty_type {
                #(#pretty_elements),*
            }
        }
    }

    fn pretty_type_ident(&self) -> Ident {
        let ident: &Ident = &self.ident;
        Ident::new(&format!("{}Pretty", ident), ident.span())
    }

    fn true_type(&self) -> TokenStream {
        let Self {
            elements,
            ident,
            vis,
        } = self;
        let elements: Vec<TokenStream> = elements
            .iter()
            .map(|element| element.true_declaration())
            .collect();
        quote! {
            #[repr(C)]
            #vis union #ident {
                #(#elements),*
            }
        }
    }
}

impl From<ItemUnion> for Registers {
    fn from(item_union: ItemUnion) -> Self {
        let ItemUnion {
            attrs: _,
            vis,
            union_token: _,
            ident,
            generics: _,
            fields:
                FieldsNamed {
                    brace_token: _,
                    named,
                },
        } = item_union;
        let elements: Vec<Element> = named.into_iter().map(|field| field.into()).collect();
        Self {
            elements,
            ident,
            vis,
        }
    }
}

impl From<Registers> for TokenStream {
    fn from(registers: Registers) -> Self {
        let pretty_type: TokenStream = registers.pretty_type();
        let true_type: TokenStream = registers.true_type();
        quote! {
            #pretty_type
            #true_type
        }
    }
}

struct Element {
    ident: Ident,
    type_path: Path,
}

impl Element {
    fn pretty_declaration(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let pretty_type_path: Path = self.pretty_type_path();
        quote! {
            #ident: #pretty_type_path
        }
    }

    fn pretty_type_path(&self) -> Path {
        let Path {
            leading_colon,
            segments,
        } = &self.type_path;
        let leading_colon: Option<PathSep> = leading_colon.clone();
        let mut segments: Punctuated<PathSegment, PathSep> = segments.clone();
        segments.last_mut().map(|last_segment| {
            let ident: &mut Ident = &mut last_segment.ident;
            *ident = Ident::new(&format!("{}Pretty", ident), ident.span());
        });
        Path {
            leading_colon,
            segments,
        }
    }

    fn true_declaration(&self) -> TokenStream {
        let Self { ident, type_path } = self;
        quote! {
            #ident: core::mem::ManuallyDrop<#type_path>
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
            ty:
                Type::Path(TypePath {
                    attrs: _,
                    qself: _,
                    path: type_path,
                }),
            default: _,
        } = field
        {
            Self { ident, type_path }
        } else {
            panic!();
        }
    }
}
