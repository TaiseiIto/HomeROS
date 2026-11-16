use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Type, Visibility},
};

pub struct Structure {
    vis: Visibility,
    ident: Ident,
    elements: Vec<Element>,
}

impl Structure {
    fn implement(&self) -> TokenStream {
        let Self {
            vis,
            ident,
            elements,
        } = self;
        let reads: Vec<TokenStream> = elements.iter().map(|element| element.read(vis)).collect();
        quote! {
            impl #ident {
                #(#reads)*
            }
        }
    }
}

impl From<DeriveInput> for Structure {
    fn from(
        DeriveInput {
            attrs: _,
            vis,
            ident,
            generics: _,
            data,
        }: DeriveInput,
    ) -> Self {
        if let Data::Struct(DataStruct {
            struct_token: _,
            fields:
                Fields::Named(FieldsNamed {
                    brace_token: _,
                    named,
                }),
            semi_token: _,
        }) = data
        {
            let elements: Vec<Element> = named.into_iter().map(Into::into).collect();
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
    fn read(&self, vis: &Visibility) -> TokenStream {
        let Self { ident, ty } = self;
        quote! {
            #vis fn #ident(&self) -> #ty {
                unsafe {
                    core::ptr::addr_of!(self.#ident).read_unaligned()
                }
            }
        }
    }
}

impl From<Field> for Element {
    fn from(
        Field {
            attrs: _,
            vis: _,
            modifiers: _,
            ident,
            colon_token: _,
            ty,
            default: _,
        }: Field,
    ) -> Self {
        if let Some(ident) = ident {
            Self { ident, ty }
        } else {
            panic!();
        }
    }
}
