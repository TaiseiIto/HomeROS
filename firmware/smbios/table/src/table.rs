use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{
        Attribute, Data, DataStruct, DeriveInput, Field, Fields, FieldsNamed, Ident, Meta, Type,
    },
};

pub struct Structure {
    ident: Ident,
    elements: Vec<Element>,
}

impl Structure {
    fn debug(&self) -> TokenStream {
        let Self { ident, elements } = self;
        let debugs: Vec<TokenStream> = elements.iter().skip(1).map(Element::debug).collect();
        quote! {
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut debug_struct: ::core::fmt::DebugStruct = formatter.debug_struct(stringify!(#ident));
                    debug_struct.field("typ", &self.get_header().get_type());
                    debug_struct.field("length", &self.get_header().size());
                    debug_struct.field("handle", &self.get_header().handle());
                    #(#debugs)*
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
        let Self { ident, elements } = self;
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
            Self { ident, elements }
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
        } = self;
        let get_ident: Ident = self.get_ident();
        if *is_string {
            quote! {
                fn #get_ident(&self) -> Option<::alloc::string::String> {
                    (::core::mem::offset_of!(Self, #ident) + ::core::mem::size_of_val(&self.#ident()) <= self.get_header().size())
                        .then(|| self.#ident() as usize)
                        .and_then(|#ident| #ident.checked_sub(1))
                        .and_then(|#ident| self.strings().get(#ident).cloned())
                }
            }
        } else {
            quote! {
                fn #get_ident(&self) -> Option<#ty> {
                    (::core::mem::offset_of!(Self, #ident) + ::core::mem::size_of_val(&self.#ident()) <= self.get_header().size()).then(|| self.#ident())
                }
            }
        }
    }

    fn get_ident(&self) -> Ident {
        Ident::new(&format!("get_{}", self.ident), self.ident.span())
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
            let is_string: bool = attrs.iter().fold(
                false,
                |is_string,
                 Attribute {
                     pound_token: _,
                     style: _,
                     bracket_token: _,
                     meta,
                 }| match meta {
                    Meta::Path(path) => match path.get_ident() {
                        Some(path) => match path.to_string().as_str() {
                            "string" => true,
                            _ => is_string,
                        },
                        None => is_string,
                    },
                    _ => is_string,
                },
            );
            Self {
                ident,
                ty,
                is_string,
            }
        } else {
            panic!();
        }
    }
}
