use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{
        Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Meta,
        MetaList, Path, PathSegment, Type, TypePath, punctuated::Punctuated, token::Comma,
    },
};

pub struct Structure {
    ident: Ident,
    elements: Vec<Element>,
    prints: Vec<Ident>,
}

impl Structure {
    fn debug(&self) -> TokenStream {
        let Self {
            ident,
            elements,
            prints,
        } = self;
        let elements: Vec<TokenStream> = elements.iter().skip(1).map(Element::debug).collect();
        let prints: Vec<TokenStream> = prints
            .iter()
            .map(|print| {
                quote! {
                    if let Some(#print) = self.#print() {
                        debug_struct.field(stringify!(#print), &#print);
                    }
                }
            })
            .collect();
        quote! {
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut debug_struct: ::core::fmt::DebugStruct = formatter.debug_struct(stringify!(#ident));
                    debug_struct.field("typ", &self.get_header().get_type());
                    debug_struct.field("length", &self.get_header().size());
                    debug_struct.field("handle", &self.get_header().handle());
                    #(#elements)*
                    #(#prints)*
                    debug_struct.finish()
                }
            }
        }
    }

    fn get_header(&self) -> TokenStream {
        let header: &Ident = &self.elements.first().unwrap().ident;
        quote! {
            fn get_header(&self) -> &Header {
                &self.#header
            }
        }
    }

    fn implement(&self) -> TokenStream {
        let Self {
            ident,
            elements,
            prints: _,
        } = self;
        let get_header: TokenStream = self.get_header();
        let strings: TokenStream = self.strings();
        let gets: Vec<TokenStream> = elements.iter().skip(1).map(Element::get).collect();
        quote! {
            impl #ident {
                #get_header
                #strings
                #(#gets)*
            }
        }
    }

    fn strings(&self) -> TokenStream {
        quote! {
            fn strings(&self) -> ::alloc::vec::Vec<::alloc::string::String> {
                self.get_header().strings().collect()
            }
        }
    }

    fn table(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        quote! {
            impl crate::Table for #ident {}
        }
    }
}

impl From<DeriveInput> for Structure {
    fn from(structure: DeriveInput) -> Self {
        if let DeriveInput {
            attrs,
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
            let elements: Vec<Element> = named
                .into_iter()
                .filter_map(|element| element.try_into().ok())
                .collect();
            let prints: Vec<Ident> = attrs
                .iter()
                .fold(None, |prints, attribute| {
                    if let Attribute {
                        pound_token: _,
                        style: _,
                        bracket_token: _,
                        meta:
                            Meta::List(
                                meta_list @ MetaList {
                                    path,
                                    delimiter: _,
                                    tokens: _,
                                },
                            ),
                    } = attribute
                    {
                        match path.get_ident().unwrap().to_string().as_str() {
                            "print" => {
                                let prints: Punctuated<Ident, Comma> = meta_list
                                    .parse_args_with(Punctuated::parse_terminated)
                                    .unwrap();
                                Some(prints.iter().cloned().collect())
                            }
                            _ => prints,
                        }
                    } else {
                        prints
                    }
                })
                .unwrap_or_default();
            Self {
                ident,
                elements,
                prints,
            }
        } else {
            panic!();
        }
    }
}

impl From<Structure> for TokenStream {
    fn from(structure: Structure) -> Self {
        let debug: TokenStream = structure.debug();
        let implement: TokenStream = structure.implement();
        let table: TokenStream = structure.table();
        quote! {
            #debug
            #implement
            #table
        }
    }
}

struct Element {
    ident: Ident,
    ty: Type,
    is_string: bool,
    debug_type: Option<Type>,
}

impl Element {
    fn debug(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let get_ident: Ident = self.get_ident();
        quote! {
            if let Some(#ident) = self.#get_ident() {
                debug_struct.field(stringify!(#ident), &#ident);
            }
        }
    }

    fn get(&self) -> TokenStream {
        let Self {
            ident,
            ty,
            is_string,
            debug_type,
        } = self;
        let get_ident: Ident = self.get_ident();
        let (return_type, process): (TokenStream, TokenStream) = if *is_string {
            (
                quote! {
                    ::alloc::string::String
                },
                quote! {
                    .then(|| self.#ident() as usize)
                    .and_then(|#ident| #ident.checked_sub(1))
                    .and_then(|#ident| self.strings().get(#ident).cloned())
                },
            )
        } else if let Some(debug_type) = debug_type {
            (
                quote! {
                    #debug_type
                },
                quote! {
                    .then(|| self.#ident().into())
                },
            )
        } else {
            (
                quote! {
                    #ty
                },
                quote! {
                    .then(|| self.#ident())
                },
            )
        };
        quote! {
            fn #get_ident(&self) -> Option<#return_type> {
                (::core::mem::offset_of!(Self, #ident) + ::core::mem::size_of_val(&self.#ident()) <= self.get_header().size())#process
            }
        }
    }

    fn get_ident(&self) -> Ident {
        Ident::new(&format!("get_{}", self.ident), self.ident.span())
    }
}

impl TryFrom<Field> for Element {
    type Error = ();

    fn try_from(field: Field) -> Result<Self, Self::Error> {
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
            let is_phantom_data: bool = if let Type::Path(TypePath {
                attrs: _,
                qself: _,
                path:
                    Path {
                        leading_colon: _,
                        ref segments,
                    },
            }) = ty
            {
                segments.first().is_some_and(
                    |PathSegment {
                         ident,
                         arguments: _,
                     }| ident.to_string().as_str() == "PhantomData",
                )
            } else {
                false
            };
            if is_phantom_data {
                Err(())
            } else {
                let (is_string, debug_type): (bool, Option<Type>) = attrs.iter().fold(
                    (false, None),
                    |(is_string, debug_type),
                     Attribute {
                         pound_token: _,
                         style: _,
                         bracket_token: _,
                         meta,
                     }| match meta {
                        Meta::List(
                            meta_list @ MetaList {
                                path,
                                delimiter: _,
                                tokens: _,
                            },
                        ) => match path.get_ident().unwrap().to_string().as_str() {
                            "debug_type" => {
                                (is_string, Some(meta_list.parse_args::<Type>().unwrap()))
                            }
                            _ => (is_string, debug_type),
                        },
                        Meta::Path(path) => match path.get_ident() {
                            Some(path) => match path.to_string().as_str() {
                                "string" => (true, debug_type),
                                _ => (is_string, debug_type),
                            },
                            None => (is_string, debug_type),
                        },
                        _ => (is_string, debug_type),
                    },
                );
                Ok(Self {
                    ident,
                    ty,
                    is_string,
                    debug_type,
                })
            }
        } else {
            panic!();
        }
    }
}
