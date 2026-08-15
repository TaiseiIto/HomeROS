use {
    proc_macro2::{Span, TokenStream},
    quote::quote,
    syn::{Field, Fields, FieldsNamed, Ident, ItemStruct, Type, Visibility},
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
                let ident: Ident = element.ident();
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
            let elements: Vec<Element> = named
                .into_iter()
                .enumerate()
                .map(|(index, field)| Element::new(index, field))
                .collect();
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
        let true_type: TokenStream = structure.true_type();
        quote! {
            #true_type
            #implement
        }
    }
}

struct Element {
    ident: Option<Ident>,
    index: usize,
    span: Span,
    ty: Type,
}

impl Element {
    fn ident(&self) -> Ident {
        let Self {
            ident,
            index,
            span,
            ty,
        } = self;
        ident
            .clone()
            .unwrap_or(Ident::new(&format!("reserved{}", index), *span))
    }

    fn new(index: usize, field: Field) -> Self {
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
            let span: Span = ident.span();
            let ident: Option<Ident> = (ident != "__").then_some(ident);
            Self {
                ident,
                index,
                span,
                ty,
            }
        } else {
            panic!();
        }
    }
}
