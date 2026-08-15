use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{
        Expr, ExprLit, Field, Fields, FieldsNamed, Ident, ItemStruct, Lit, Path, PathSegment, Type,
        TypeArray, TypePath, Visibility,
    },
};

pub struct Structure {
    elements: Vec<Element>,
    ident: Ident,
    vis: Visibility,
}

impl Structure {
    fn bits(&self) -> u8 {
        self.elements.iter().map(|element| element.bits).sum()
    }

    fn bit_reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .map(|element| element.bit_read())
            .collect()
    }

    fn bit_updates(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .map(|element| element.bit_update())
            .collect()
    }

    fn implement(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let bit_reads: Vec<TokenStream> = self.bit_reads();
        let bit_updates: Vec<TokenStream> = self.bit_updates();
        let into_pretty: TokenStream = self.into_pretty();
        let lengths: Vec<TokenStream> = self.lengths();
        let mask_consts: Vec<TokenStream> = self.mask_consts();
        let offsets: Vec<TokenStream> = self.offsets();
        let read_volatile: TokenStream = self.read_volatile();
        let write_volatile: TokenStream = self.write_volatile();
        quote! {
            impl #ident {
                #(#bit_reads)*
                #(#bit_updates)*
                #into_pretty
                #(#lengths)*
                #(#mask_consts)*
                #(#offsets)*
                #read_volatile
                #write_volatile
            }
        }
    }

    fn inner_type(&self) -> Ident {
        Ident::new(&format!("u{}", self.bits()), self.ident.span())
    }

    fn into_pretty(&self) -> TokenStream {
        let pretty_structure: Ident = self.pretty_structure_ident();
        let into_pretty_elements: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| element.into_pretty())
            .collect();
        quote! {
            pub fn into_pretty(self) -> #pretty_structure {
                #pretty_structure {
                    #(#into_pretty_elements),*
                }
            }
        }
    }

    fn lengths(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .map(|element| element.length())
            .collect()
    }

    fn mask_consts(&self) -> Vec<TokenStream> {
        let element_offsets: Vec<ElementOffset> = self.into();
        element_offsets
            .into_iter()
            .map(|ElementOffset { element, offset }| element.mask_const(self, offset))
            .collect()
    }

    fn offsets(&self) -> Vec<TokenStream> {
        let element_offsets: Vec<ElementOffset> = self.into();
        element_offsets
            .into_iter()
            .map(|ElementOffset { element, offset }| element.offset(offset))
            .collect()
    }

    fn pretty_structure(&self) -> TokenStream {
        let vis: &Visibility = &self.vis;
        let pretty_structure: Ident = self.pretty_structure_ident();
        let elements: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| element.pretty())
            .collect();
        quote! {
            #vis struct #pretty_structure {
                #(#elements),*
            }
        }
    }

    fn pretty_structure_ident(&self) -> Ident {
        let ident: &Ident = &self.ident;
        Ident::new(&format!("{}Pretty", ident), ident.span())
    }

    fn true_type(&self) -> TokenStream {
        let inner_type: Ident = self.inner_type();
        let Self {
            vis,
            ident,
            elements: _,
        } = self;
        quote! {
            #[repr(transparent)]
            #vis struct #ident(#inner_type);
        }
    }

    fn read_volatile(&self) -> TokenStream {
        quote! {
            pub unsafe fn read_volatile(&self) -> Self {
                unsafe {
                    core::ptr::read_volatile(self as *const Self)
                }
            }
        }
    }

    fn write_volatile(&self) -> TokenStream {
        quote! {
            pub unsafe fn write_volatile(&mut self, argument: Self) {
                unsafe {
                    core::ptr::write_volatile(self as *mut Self, argument);
                }
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
                elements,
                ident,
                vis,
            }
        } else {
            panic!();
        }
    }
}

impl From<Structure> for TokenStream {
    fn from(structure: Structure) -> Self {
        let true_type: TokenStream = structure.true_type();
        let implement: TokenStream = structure.implement();
        let pretty_structure: TokenStream = structure.pretty_structure();
        quote! {
            #true_type
            #implement
            #pretty_structure
        }
    }
}

struct Element {
    bits: u8,
    ident: Ident,
    reserved: bool,
}

impl Element {
    fn bit_read(&self) -> TokenStream {
        let bits: u8 = self.bits;
        let bit_read: Ident = self.bit_read_ident();
        let offset: Ident = self.offset_ident();
        let bits_usize: usize = bits as usize;
        let bools: Vec<TokenStream> = (0..bits)
            .map(|bit| {
                quote! {
                    self.0 & (1 << (#bit + Self::#offset)) != 0
                }
            })
            .collect();
        quote! {
            fn #bit_read(&self) -> [bool; #bits_usize] {
                [#(#bools),*]
            }
        }
    }

    fn bit_read_ident(&self) -> Ident {
        self.function_ident("bit_read")
    }

    fn bit_update(&self) -> TokenStream {
        let bit_update: Ident = self.bit_update_ident();
        let mask_const: Ident = self.mask_const_ident();
        let offset: Ident = self.offset_ident();
        let bits: u8 = self.bits;
        let bits_usize: usize = bits as usize;
        let values: Vec<TokenStream> = (0..bits)
            .map(|bit| {
                quote! {
                    if argument[#bits_usize] { 1 << (#bit + Self::#offset) } else { 0 }
                }
            })
            .collect();
        quote! {
            fn #bit_update(self, argument: [bool; #bits_usize]) -> Self {
                Self((self.0 & !Self::#mask_const) | ((#(#values)|*) & Self::#mask_const))
            }
        }
    }

    fn bit_update_ident(&self) -> Ident {
        self.function_ident("bit_update")
    }

    fn const_ident(&self, suffix: &str) -> Ident {
        Ident::new(
            &format!("{}_{}", self.ident.to_string().to_uppercase(), suffix),
            self.ident.span(),
        )
    }

    fn function_ident(&self, suffix: &str) -> Ident {
        Ident::new(&format!("{}_{}", self.ident, suffix), self.ident.span())
    }

    fn into_pretty(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let bit_read: Ident = self.bit_read_ident();
        quote! {
            #ident: self.#bit_read()
        }
    }

    fn length(&self) -> TokenStream {
        let bits: u8 = self.bits;
        let length: Ident = self.length_ident();
        quote! {
            const #length: u8 = #bits;
        }
    }

    fn length_ident(&self) -> Ident {
        self.const_ident("LENGTH")
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
            let (ident, reserved): (Ident, bool) = if ident == "__" {
                (
                    Ident::new(&format!("reserved{}", index), ident.span()),
                    true,
                )
            } else {
                (ident, false)
            };
            let bits: u8 = Self::type2bits(ty);
            Self {
                bits,
                ident,
                reserved,
            }
        } else {
            panic!();
        }
    }

    fn mask_const(&self, structure: &Structure, offset: u8) -> TokenStream {
        let mask_const: Ident = self.mask_const_ident();
        match structure.bits() {
            8 => {
                let mask: u8 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                quote! {
                    const #mask_const: u8 = #mask;
                }
            }
            16 => {
                let mask: u16 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                quote! {
                    const #mask_const: u16 = #mask;
                }
            }
            32 => {
                let mask: u32 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                quote! {
                    const #mask_const: u32 = #mask;
                }
            }
            64 => {
                let mask: u64 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                quote! {
                    const #mask_const: u64 = #mask;
                }
            }
            128 => {
                let mask: u128 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                quote! {
                    const #mask_const: u128 = #mask;
                }
            }
            _ => panic!(),
        }
    }

    fn mask_const_ident(&self) -> Ident {
        self.const_ident("MASK")
    }

    fn offset(&self, offset: u8) -> TokenStream {
        let offset_const: Ident = self.offset_ident();
        quote! {
            const #offset_const: u8 = #offset;
        }
    }

    fn offset_ident(&self) -> Ident {
        self.const_ident("OFFSET")
    }

    fn pretty(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let bits: usize = self.bits as usize;
        quote! {
            #ident: [bool; #bits]
        }
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
                match ident.to_string().as_str() {
                    "bool" => 1,
                    "u8" => 8,
                    "u16" => 16,
                    "u32" => 32,
                    "u64" => 64,
                    "u128" => 128,
                    _ => panic!(),
                }
            }
            _ => panic!(),
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
