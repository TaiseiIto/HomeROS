use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{Field, Fields, FieldsNamed, Ident, ItemStruct, Type, Visibility},
};

pub struct Structure {
    vis: Visibility,
    ident: Ident,
    elements: Vec<Element>,
}

impl Structure {
    fn true_type(&self) -> TokenStream {
        let Self {
            vis,
            ident,
            elements,
        } = self;
        let elements: Vec<TokenStream> = elements
            .iter()
            .enumerate()
            .map(|(index, element)| {
                let ident: Ident = element
                    .ident
                    .clone()
                    .unwrap_or(Ident::new(&format!("reserved{}", index), ident.span()));
                let ty: &Type = &element.ty;
                quote! {
                    #ident: #ty
                }
            })
            .collect();
        quote! {
            #[repr(packed)]
            #vis struct #ident {
                #(#elements),*
            }
        }
    }
}

impl From<ItemStruct> for Structure {
    fn from(item_struct: ItemStruct) -> Self {
        if let ItemStruct {
            attrs: _,
            vis,
            struct_token: _,
            ident,
            generics: _,
            fields:
                Fields::Named(FieldsNamed {
                    brace_token: _,
                    named,
                }),
            semi_token: _,
        } = item_struct
        {
            let elements: Vec<Element> = named.into_iter().map(|field| field.into()).collect();
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
        let true_type: TokenStream = structure.true_type();
        quote! {
            #true_type
        }
    }
}

struct Element {
    ident: Option<Ident>,
    ty: Type,
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
            let ident: Option<Ident> = (ident != "__").then_some(ident);
            Self { ident, ty }
        } else {
            panic!();
        }
    }
}
