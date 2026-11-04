use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{
        Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Meta, Path,
        Type, Visibility,
    },
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
        quote! {
            impl #ident {
            }
        }
    }
}

impl From<DeriveInput> for Structure {
    fn from(structure: DeriveInput) -> Self {
        if let DeriveInput {
            attrs: _,
            vis,
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
    is_header: bool,
}

impl From<Field> for Element {
    fn from(field: Field) -> Self {
        if let Field {
            attrs,
            vis: _,
            modifiers: _,
            ident: Some(ident),
            colon_token: _,
            ty,
            default: _,
        } = field
        {
            let is_header: bool = attrs.iter().fold(
                false,
                |is_header,
                 Attribute {
                     pound_token: _,
                     style: _,
                     bracket_token: _,
                     meta,
                 }| {
                    is_header
                        || match meta {
                            Meta::Path(path) => path.is_ident("header"),
                            _ => false,
                        }
                },
            );
            Self {
                ident,
                ty,
                is_header,
            }
        } else {
            panic!();
        }
    }
}
