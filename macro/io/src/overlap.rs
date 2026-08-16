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
    fn reader_type(&self) -> TokenStream {
        let Self {
            elements,
            ident: _,
            vis,
        } = self;
        let reader_type: Ident = self.reader_type_ident();
        let reader_elements: Vec<TokenStream> = elements
            .iter()
            .map(|element| element.reader_declaration())
            .collect();
        quote! {
            #vis struct #reader_type {
                #(#reader_elements),*
            }
        }
    }

    fn reader_type_ident(&self) -> Ident {
        let ident: &Ident = &self.ident;
        Ident::new(&format!("{}Reader", ident), ident.span())
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
        let true_type: TokenStream = registers.true_type();
        let reader_type: TokenStream = registers.reader_type();
        quote! {
            #true_type
            #reader_type
        }
    }
}

struct Element {
    ident: Ident,
    type_path: Path,
}

impl Element {
    fn reader_declaration(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let reader_type_path: Path = self.reader_type_path();
        quote! {
            #ident: #reader_type_path
        }
    }

    fn reader_type_path(&self) -> Path {
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
