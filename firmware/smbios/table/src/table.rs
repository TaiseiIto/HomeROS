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
    fn debug(&self) -> TokenStream {
        let Self {
            vis,
            ident,
            elements,
        } = self;
        quote! {
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    let mut debug_struct: ::core::fmt::DebugStruct = formatter.debug_struct(::core::stringify!(#ident));
                    debug_struct.field("typ", &self.get_header().get_type());
                    debug_struct.field("length", &self.get_header().size());
                    debug_struct.field("handle", &self.get_header().handle());
                    debug_struct.finish()
                }
            }
        }
    }

    fn implement(&self) -> TokenStream {
        let Self {
            vis,
            ident,
            elements,
        } = self;
        let gets: Vec<TokenStream> = elements.iter().skip(1).map(Element::get).collect();
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
    is_string: bool,
}

impl Element {
    fn get(&self) -> TokenStream {
        let Self {
            ident,
            ty,
            is_string,
        } = self;
        let get_ident: Ident = Ident::new(&format!("get_{}", ident), ident.span());
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
