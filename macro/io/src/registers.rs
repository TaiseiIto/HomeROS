use {
    proc_macro2::{Span, TokenStream},
    quote::quote,
    std::iter,
    syn::{Field, Fields, FieldsNamed, Ident, ItemStruct, Type, Visibility},
};

pub struct Structure {
    vis: Visibility,
    ident: Ident,
    elements: Vec<Element>,
}

impl Structure {
    fn implement(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let sizes: Vec<TokenStream> = self.sizes();
        let offsets: Vec<TokenStream> = self.offsets();
        quote! {
            impl #ident {
                #(#sizes)*
                #(#offsets)*
            }
        }
    }

    fn offsets(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .zip(
                self.elements
                    .iter()
                    .rev()
                    .skip(1)
                    .map(Some)
                    .chain(iter::once(None))
                    .rev(),
            )
            .map(|(element, previous_element)| element.offset(previous_element))
            .collect()
    }

    fn sizes(&self) -> Vec<TokenStream> {
        self.elements.iter().map(|element| element.size()).collect()
    }

    fn true_type(&self) -> TokenStream {
        let Self {
            vis,
            ident,
            elements,
        } = self;
        let elements: Vec<TokenStream> = elements
            .iter()
            .map(|element| element.true_declaration())
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
    fn const_ident(&self, suffix: &str) -> Ident {
        Ident::new(
            &format!("{}_{}", self.ident().to_string().to_uppercase(), suffix),
            self.span,
        )
    }

    fn ident(&self) -> Ident {
        let Self {
            ident,
            index,
            span,
            ty: _,
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

    fn offset(&self, previous: Option<&Self>) -> TokenStream {
        let offset: Ident = self.offset_ident();
        let value: TokenStream = if let Some(previous) = previous {
            let previous_offset: Ident = previous.offset_ident();
            let previous_size: Ident = previous.size_ident();
            quote! {
                Self::#previous_offset + Self::#previous_size
            }
        } else {
            quote! {
                0
            }
        };
        quote! {
            pub const #offset: usize = #value;
        }
    }

    fn offset_ident(&self) -> Ident {
        self.const_ident("OFFSET")
    }

    fn size(&self) -> TokenStream {
        let size: Ident = self.size_ident();
        let ty: &Type = &self.ty;
        quote! {
            pub const #size: usize = core::mem::size_of::<#ty>();
        }
    }

    fn size_ident(&self) -> Ident {
        self.const_ident("SIZE")
    }

    fn true_declaration(&self) -> TokenStream {
        let ident: Ident = self.ident();
        let ty: &Type = &self.ty;
        quote! {
            #ident: #ty
        }
    }
}
