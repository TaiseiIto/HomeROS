use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{Field, FieldsNamed, Ident, ItemUnion, Type, Visibility},
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
    ty: Type,
}

impl Element {
    fn pretty_declaration(&self) -> TokenStream {
        let Self { ident, ty } = self;
        quote! {
            #ident: #ty
        }
    }

    fn true_declaration(&self) -> TokenStream {
        let Self { ident, ty } = self;
        quote! {
            #ident: core::mem::ManuallyDrop<#ty>
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
