use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{
        Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Path, PathSegment, Type,
        TypePath,
    },
};

pub struct Structure {
    elements: Vec<Element>,
    ident: Ident,
}

impl Structure {
    fn implement(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let reads: Vec<TokenStream> = self.elements.iter().map(Element::read).collect();
        quote! {
            impl #ident {
                #(#reads)*
            }
        }
    }
}

impl From<DeriveInput> for Structure {
    fn from(structure: DeriveInput) -> Self {
        if let DeriveInput {
            attrs: _,
            vis: _,
            ident,
            generics: _,
            data:
                Data::Struct(DataStruct {
                    struct_token: _,
                    fields:
                        Fields::Named(FieldsNamed {
                            brace_token: _,
                            named,
                        }),
                    semi_token: _,
                }),
        } = structure
        {
            let elements: Vec<Element> = named.into_iter().map(Into::into).collect();
            Self { elements, ident }
        } else {
            panic!();
        }
    }
}

impl From<Structure> for TokenStream {
    fn from(structure: Structure) -> Self {
        let implement: TokenStream = structure.implement();
        quote! {
            #implement
        }
    }
}

struct Element {
    ident: Ident,
    ty: Type,
}

impl Element {
    fn read(&self) -> TokenStream {
        let Self { ident, ty } = self;
        quote! {
            fn #ident(&self) -> #ty {
                unsafe {
                    core::ptr::addr_of!(self.#ident).read_unaligned()
                }
            }
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
            Self { ident, ty }
        } else {
            panic!();
        }
    }
}
