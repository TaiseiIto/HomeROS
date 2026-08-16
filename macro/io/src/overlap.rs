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
        quote! {
            #true_type
        }
    }
}

struct Element {
    ident: Ident,
    ty: Type,
}

impl Element {
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
