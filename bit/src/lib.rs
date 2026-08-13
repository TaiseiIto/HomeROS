use {
    quote::quote,
    syn::{
        Expr, ExprLit, Field, Fields, FieldsNamed, Ident, ItemStruct, Lit, Path, PathSegment, Type,
        TypeArray, TypePath, Visibility, parse_macro_input,
    },
};

#[proc_macro_attribute]
pub fn field(
    _attributes: proc_macro::TokenStream,
    structure: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let item_struct: ItemStruct = parse_macro_input!(structure as ItemStruct);
    let structure: Structure = item_struct.into();
    let token_stream: proc_macro2::TokenStream = structure.into();
    token_stream.into()
}

struct Element {
    ident: Option<Ident>,
    bits: u8,
}

impl Element {
    fn bits_read(&self, offset: u8) -> Option<proc_macro2::TokenStream> {
        self.function_ident("bits_read").map(|bits_read| {
            let bits: u8 = self.bits;
            let (return_type, return_value): (proc_macro2::TokenStream, proc_macro2::TokenStream) =
                if bits == 1 {
                    (quote! { bool }, quote! { self.0 & (1 << #offset) != 0 })
                } else {
                    let bools: Vec<proc_macro2::TokenStream> = (0..bits)
                        .map(|bit| {
                            let shift: u8 = offset + bit;
                            quote! {
                                self.0 & (1 << #shift) != 0
                            }
                        })
                        .collect();
                    let bits: usize = bits as usize;
                    (quote! { [bool; #bits] }, quote! { [#(#bools),*] })
                };
            quote! {
                pub fn #bits_read(&self) -> #return_type {
                    #return_value
                }
            }
        })
    }

    fn bits_remake(&self, structure: &Structure, offset: u8) -> Option<proc_macro2::TokenStream> {
        self.function_ident("bits_remake")
            .zip(self.function_ident("mask_remake"))
            .map(|(bits_remake, mask_remake)| {
                let inner_type: Ident = structure.inner_type();
                let bits: u8 = self.bits;
                let (argument_type, argument_value): (
                    proc_macro2::TokenStream,
                    proc_macro2::TokenStream,
                ) = if bits == 1 {
                    (
                        quote! { bool },
                        quote! { if argument { 1 << #offset } else { 0 } },
                    )
                } else {
                    let values: Vec<proc_macro2::TokenStream> = (0..bits)
                        .map(|bit| {
                            let shift: u8 = offset + bit;
                            quote! {
                                if argument[#bit] { 1 << #shift } else { 0 }
                            }
                        })
                        .collect();
                    let bits: usize = bits as usize;
                    (quote! { [bool; #bits] }, quote! { #(#values)|* })
                };
                quote! {
                    pub fn #bits_remake(self, argument: #argument_type) -> Self {
                        let value: #inner_type = #argument_value;
                        self.#mask_remake(value)
                    }
                }
            })
    }

    fn const_ident(&self, suffix: &str) -> Option<Ident> {
        self.ident.as_ref().map(|ident| {
            Ident::new(
                &format!("{}_{}", ident.to_string().to_uppercase(), suffix),
                ident.span(),
            )
        })
    }

    fn function_ident(&self, suffix: &str) -> Option<Ident> {
        self.ident
            .as_ref()
            .map(|ident| Ident::new(&format!("{}_{}", ident, suffix), ident.span()))
    }

    fn length(&self) -> Option<proc_macro2::TokenStream> {
        let bits: u8 = self.bits;
        self.const_ident("LENGTH").map(|ident| {
            quote! {
                const #ident: u8 = #bits;
            }
        })
    }

    fn mask_const(&self, structure: &Structure, offset: u8) -> Option<proc_macro2::TokenStream> {
        self.const_ident("MASK")
            .map(|ident| match structure.bits() {
                8 => {
                    let mask: u8 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                    quote! {
                        const #ident: u8 = #mask;
                    }
                }
                16 => {
                    let mask: u16 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                    quote! {
                        const #ident: u16 = #mask;
                    }
                }
                32 => {
                    let mask: u32 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                    quote! {
                        const #ident: u32 = #mask;
                    }
                }
                64 => {
                    let mask: u64 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                    quote! {
                        const #ident: u64 = #mask;
                    }
                }
                128 => {
                    let mask: u128 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                    quote! {
                        const #ident: u128 = #mask;
                    }
                }
                _ => panic!(),
            })
    }

    fn mask_read(&self, structure: &Structure) -> Option<proc_macro2::TokenStream> {
        self.function_ident("mask_read")
            .zip(self.const_ident("MASK"))
            .map(|(mask_read, mask_const)| {
                let inner_type: Ident = structure.inner_type();
                quote! {
                    pub fn #mask_read(&self) -> #inner_type {
                        self.0 & Self::#mask_const
                    }
                }
            })
    }

    fn mask_remake(&self, structure: &Structure) -> Option<proc_macro2::TokenStream> {
        self.function_ident("mask_remake")
            .zip(self.const_ident("MASK"))
            .map(|(mask_remake, mask_const)| {
                let inner_type: Ident = structure.inner_type();
                quote! {
                    pub fn #mask_remake(self, value: #inner_type) -> Self {
                        Self((self.0 & !Self::#mask_const) | (value & Self::#mask_const))
                    }
                }
            })
    }

    fn offset(&self, offset: u8) -> Option<proc_macro2::TokenStream> {
        self.const_ident("OFFSET").map(|ident| {
            quote! {
                const #ident: u8 = #offset;
            }
        })
    }

    fn type2bits(ty: Type) -> u8 {
        match ty {
            Type::Array(TypeArray {
                attrs: _,
                bracket_token: _,
                elem,
                semi_token: _,
                len,
            }) => {
                let unit: u8 = Self::type2bits(*elem);
                let len: u8 = if let Expr::Lit(ExprLit {
                    attrs: _,
                    lit: Lit::Int(lit_int),
                }) = len
                {
                    lit_int.base10_parse().unwrap()
                } else {
                    panic!();
                };
                len * unit
            }
            Type::Path(TypePath {
                attrs: _,
                qself: _,
                path:
                    Path {
                        leading_colon: _,
                        segments,
                    },
            }) => {
                let mut segments = segments.iter();
                let segment: &PathSegment = segments.next().unwrap();
                assert!(segments.next().is_none());
                let PathSegment {
                    ident,
                    arguments: _,
                } = segment;
                assert!(ident == "bool");
                1
            }
            _ => panic!(),
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
            let ident: Option<Ident> = (ident != "reserved").then_some(ident);
            let bits: u8 = Self::type2bits(ty);
            Self { ident, bits }
        } else {
            panic!();
        }
    }
}

struct ElementOffset<'a> {
    element: &'a Element,
    offset: u8,
}

impl<'a> From<&'a Structure> for Vec<ElementOffset<'a>> {
    fn from(structure: &'a Structure) -> Self {
        structure
            .elements
            .iter()
            .fold((Vec::new(), 0), |(mut element_offsets, offset), element| {
                element_offsets.push(ElementOffset { element, offset });
                (element_offsets, offset + element.bits)
            })
            .0
    }
}

struct Structure {
    vis: Visibility,
    ident: Ident,
    elements: Vec<Element>,
}

impl Structure {
    fn bits(&self) -> u8 {
        self.elements.iter().map(|element| element.bits).sum()
    }

    fn bits_reads(&self) -> Vec<proc_macro2::TokenStream> {
        let element_offsets: Vec<ElementOffset> = self.into();
        element_offsets
            .into_iter()
            .filter_map(|ElementOffset { element, offset }| element.bits_read(offset))
            .collect()
    }

    fn bits_remakes(&self) -> Vec<proc_macro2::TokenStream> {
        let element_offsets: Vec<ElementOffset> = self.into();
        element_offsets
            .into_iter()
            .filter_map(|ElementOffset { element, offset }| element.bits_remake(self, offset))
            .collect()
    }

    fn implement(&self) -> proc_macro2::TokenStream {
        let ident: &Ident = &self.ident;
        let bits_reads: Vec<proc_macro2::TokenStream> = self.bits_reads();
        let bits_remakes: Vec<proc_macro2::TokenStream> = self.bits_remakes();
        let lengths: Vec<proc_macro2::TokenStream> = self.lengths();
        let mask_consts: Vec<proc_macro2::TokenStream> = self.mask_consts();
        let mask_reads: Vec<proc_macro2::TokenStream> = self.mask_reads();
        let mask_remake: Vec<proc_macro2::TokenStream> = self.mask_remakes();
        let offsets: Vec<proc_macro2::TokenStream> = self.offsets();
        quote! {
            impl #ident {
                #(#bits_reads)*
                #(#bits_remakes)*
                #(#lengths)*
                #(#mask_consts)*
                #(#mask_reads)*
                #(#mask_remake)*
                #(#offsets)*
            }
        }
    }

    fn inner_type(&self) -> Ident {
        Ident::new(&format!("u{}", self.bits()), self.ident.span())
    }

    fn lengths(&self) -> Vec<proc_macro2::TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.length())
            .collect()
    }

    fn mask_consts(&self) -> Vec<proc_macro2::TokenStream> {
        let element_offsets: Vec<ElementOffset> = self.into();
        element_offsets
            .into_iter()
            .filter_map(|ElementOffset { element, offset }| element.mask_const(self, offset))
            .collect()
    }

    fn mask_reads(&self) -> Vec<proc_macro2::TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.mask_read(self))
            .collect()
    }

    fn mask_remakes(&self) -> Vec<proc_macro2::TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.mask_remake(self))
            .collect()
    }

    fn offsets(&self) -> Vec<proc_macro2::TokenStream> {
        let element_offsets: Vec<ElementOffset> = self.into();
        element_offsets
            .into_iter()
            .filter_map(|ElementOffset { element, offset }| element.offset(offset))
            .collect()
    }

    fn true_type(&self) -> proc_macro2::TokenStream {
        let inner_type: Ident = self.inner_type();
        let Self {
            vis,
            ident,
            elements: _,
        } = self;
        quote! {#vis struct #ident(#inner_type);}
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

impl From<Structure> for proc_macro2::TokenStream {
    fn from(structure: Structure) -> Self {
        let true_type: proc_macro2::TokenStream = structure.true_type();
        let implement: proc_macro2::TokenStream = structure.implement();
        quote! {
            #true_type
            #implement
        }
    }
}
