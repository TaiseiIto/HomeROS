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
    fn debug(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let ident_string: String = ident.to_string();
        let elements: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| element.debug())
            .collect();
        quote! {
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    formatter
                        .debug_struct(#ident_string)
                        .#(#elements).*
                        .finish()
                }
            }
        }
    }

    fn implement(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let reads: Vec<TokenStream> = self
            .elements
            .iter()
            .filter_map(|element| element.read())
            .collect();
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
            let elements: Vec<Element> = named.into_iter().map(|field| field.into()).collect();
            Self { elements, ident }
        } else {
            panic!();
        }
    }
}

impl From<Structure> for TokenStream {
    fn from(structure: Structure) -> Self {
        let debug: TokenStream = structure.debug();
        let implement: TokenStream = structure.implement();
        quote! {
            #debug
            #implement
        }
    }
}

struct Element {
    ident: Ident,
    ty: Type,
}

impl Element {
    fn debug(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let ident_string: String = ident.to_string();
        let read: TokenStream = if let Some(read) = self.read_ident() {
            quote! {
                #read()
            }
        } else {
            quote! {
                #ident
            }
        };
        quote! {
            field(#ident_string, &self.#read)
        }
    }

    fn function_ident(&self, prefix: &str) -> Ident {
        Ident::new(&format!("{}_{}", prefix, self.ident), self.ident.span())
    }

    fn read(&self) -> Option<TokenStream> {
        self.read_ident().zip(self.type_ident()).map(|(read, ty)| {
            let ident: &Ident = &self.ident;
            let read: Ident = self.function_ident("read");
            quote! {
                pub fn #read(&self) -> #ty {
                    #ty::from_be(self.#ident)
                }
            }
        })
    }

    fn read_ident(&self) -> Option<Ident> {
        self.type_ident().map(|_| self.function_ident("read"))
    }

    fn type_ident(&self) -> Option<Ident> {
        if let Type::Path(TypePath {
            attrs: _,
            qself: _,
            path: Path {
                leading_colon: _,
                segments,
            },
        }) = &self.ty
        {
            let mut segments = segments.iter();
            let segment: &PathSegment = segments.next().unwrap();
            assert!(segments.next().is_none());
            let ident: Ident = segment.ident.clone();
            match ident.to_string().as_str() {
                "u8" | "u16" | "u32" | "u64" | "u128" => Some(ident),
                _ => None,
            }
        } else {
            None
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
