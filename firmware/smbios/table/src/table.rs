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
    fn header_ident(&self) -> Ident {
        self.elements
            .iter()
            .find_map(|element| element.is_header.then_some(element.ident.clone()))
            .unwrap()
    }

    fn implement(&self) -> TokenStream {
        let Self {
            vis,
            ident,
            elements,
        } = self;
        let gets: Vec<TokenStream> = elements
            .iter()
            .filter_map(|element| element.get(self.header_ident()))
            .collect();
        quote! {
            impl #ident {
                #(#gets)*
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

impl Element {
    fn get(&self, header_ident: Ident) -> Option<TokenStream> {
        let Self {
            ident,
            ty,
            is_header,
        } = self;
        (!is_header).then(|| {
            let get_ident: Ident = Ident::new(&format!("get_{}", ident), ident.span());
            quote! {
                fn #get_ident(&self) -> Option<#ty> {
                    (::core::mem::offset_of!(Self, #ident) + ::core::mem::size_of_val(&self.#ident()) <= self.#header_ident().size()).then(|| self.#ident())
                }
            }
        })
    }
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
