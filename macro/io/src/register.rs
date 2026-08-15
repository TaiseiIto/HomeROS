use {
    proc_macro2::TokenStream,
    quote::quote,
    syn::{
        Expr, ExprLit, Field, Fields, FieldsNamed, Ident, ItemStruct, Lit, Path, PathSegment, Type,
        TypeArray, TypePath, Visibility,
    },
};

pub struct Structure {
    vis: Visibility,
    ident: Ident,
    elements: Vec<Element>,
}

impl Structure {
    fn bits(&self) -> u8 {
        self.elements.iter().map(|element| element.bits).sum()
    }

    fn bit_reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.bit_read())
            .collect()
    }

    fn bit_updates(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.bit_update())
            .collect()
    }

    fn debug(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let ident_string: String = ident.to_string();
        let fields: Vec<TokenStream> =
            self.elements
                .iter()
                .filter_map(|element| {
                    element.ident.clone().zip(element.shift_read_ident()).map(
                        |(ident, shift_read)| {
                            let ident_string: String = ident.to_string();
                            quote! {
                                field(#ident_string, &self.#shift_read())
                            }
                        },
                    )
                })
                .collect();
        quote! {
            impl core::fmt::Debug for #ident {
                fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    formatter
                        .debug_struct(#ident_string)
                        .#(#fields).*
                        .finish()
                }
            }
        }
    }

    fn implement(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let bit_reads: Vec<TokenStream> = self.bit_reads();
        let bit_updates: Vec<TokenStream> = self.bit_updates();
        let lengths: Vec<TokenStream> = self.lengths();
        let mask_consts: Vec<TokenStream> = self.mask_consts();
        let mask_reads: Vec<TokenStream> = self.mask_reads();
        let mask_update: Vec<TokenStream> = self.mask_updates();
        let offsets: Vec<TokenStream> = self.offsets();
        let shift_reads: Vec<TokenStream> = self.shift_reads();
        let shift_updates: Vec<TokenStream> = self.shift_updates();
        let uint_reads: Vec<TokenStream> = self.uint_reads();
        let uint_updates: Vec<TokenStream> = self.uint_updates();
        let read_volatile: TokenStream = self.read_volatile();
        let write_volatile: TokenStream = self.write_volatile();
        quote! {
            impl #ident {
                #(#bit_reads)*
                #(#bit_updates)*
                #(#lengths)*
                #(#mask_consts)*
                #(#mask_reads)*
                #(#mask_update)*
                #(#offsets)*
                #(#shift_reads)*
                #(#shift_updates)*
                #(#uint_reads)*
                #(#uint_updates)*
                #read_volatile
                #write_volatile
            }
        }
    }

    fn inner_type(&self) -> Ident {
        Ident::new(&format!("u{}", self.bits()), self.ident.span())
    }

    fn lengths(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.length())
            .collect()
    }

    fn mask_consts(&self) -> Vec<TokenStream> {
        let element_offsets: Vec<ElementOffset> = self.into();
        element_offsets
            .into_iter()
            .filter_map(|ElementOffset { element, offset }| element.mask_const(self, offset))
            .collect()
    }

    fn mask_reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.mask_read(self))
            .collect()
    }

    fn mask_updates(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.mask_update(self))
            .collect()
    }

    fn offsets(&self) -> Vec<TokenStream> {
        let element_offsets: Vec<ElementOffset> = self.into();
        element_offsets
            .into_iter()
            .filter_map(|ElementOffset { element, offset }| element.offset(offset))
            .collect()
    }

    fn shift_reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.shift_read(self))
            .collect()
    }

    fn shift_updates(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.shift_update(self))
            .collect()
    }

    fn uint_reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.uint_read())
            .collect()
    }

    fn uint_updates(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.uint_update(self))
            .collect()
    }

    fn true_type(&self) -> TokenStream {
        let inner_type: Ident = self.inner_type();
        let Self {
            vis,
            ident,
            elements: _,
        } = self;
        quote! {
            #[derive(Clone, Copy, Eq, PartialEq)]
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
        let implement: TokenStream = structure.implement();
        let debug: TokenStream = structure.debug();
        quote! {
            #true_type
            #implement
            #debug
        }
    }
}

struct Element {
    ident: Option<Ident>,
    bits: u8,
}

impl Element {
    fn bit_read(&self) -> Option<TokenStream> {
        let bits: u8 = self.bits;
        self.bit_read_ident()
            .zip(self.offset_ident())
            .map(|(bit_read, offset)| {
                let (return_type, return_value): (TokenStream, TokenStream) = if bits == 1 {
                    (
                        quote! { bool },
                        quote! { self.0 & (1 << Self::#offset) != 0 },
                    )
                } else {
                    let bits_usize: usize = bits as usize;
                    let bools: Vec<TokenStream> = (0..bits)
                        .map(|bit| {
                            quote! {
                                self.0 & (1 << (#bit + Self::#offset)) != 0
                            }
                        })
                        .collect();
                    (quote! { [bool; #bits_usize] }, quote! { [#(#bools),*] })
                };
                quote! {
                    pub fn #bit_read(self) -> #return_type {
                        #return_value
                    }
                }
            })
    }

    fn bit_read_ident(&self) -> Option<Ident> {
        self.function_ident(if self.bits == 1 {
            "bit_read"
        } else {
            "bits_read"
        })
    }

    fn bit_update(&self) -> Option<TokenStream> {
        if let (Some(bit_update), Some(mask_update), Some(offset)) = (
            self.bit_update_ident(),
            self.mask_update_ident(),
            self.offset_ident(),
        ) {
            Some((bit_update, mask_update, offset))
        } else {
            None
        }
        .map(|(bit_update, mask_update, offset)| {
            let bits: u8 = self.bits;
            let (argument_type, argument_value): (TokenStream, TokenStream) = if bits == 1 {
                (
                    quote! { bool },
                    quote! { if argument { 1 << Self::#offset } else { 0 } },
                )
            } else {
                let bits_usize: usize = bits as usize;
                let values: Vec<TokenStream> = (0..bits)
                    .map(|bit| {
                        quote! {
                            if argument[#bits_usize] { 1 << (#bit + Self::#offset) } else { 0 }
                        }
                    })
                    .collect();
                (quote! { [bool; #bits_usize] }, quote! { #(#values)|* })
            };
            quote! {
                pub fn #bit_update(self, argument: #argument_type) -> Self {
                    self.#mask_update(#argument_value)
                }
            }
        })
    }

    fn bit_update_ident(&self) -> Option<Ident> {
        self.function_ident(if self.bits == 1 {
            "bit_update"
        } else {
            "bits_update"
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

    fn length(&self) -> Option<TokenStream> {
        let bits: u8 = self.bits;
        self.length_ident().map(|length| {
            quote! {
                pub const #length: u8 = #bits;
            }
        })
    }

    fn length_ident(&self) -> Option<Ident> {
        self.const_ident("LENGTH")
    }

    fn mask_const(&self, structure: &Structure, offset: u8) -> Option<TokenStream> {
        self.mask_const_ident()
            .map(|mask_const| match structure.bits() {
                8 => {
                    let mask: u8 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                    quote! {
                        pub const #mask_const: u8 = #mask;
                    }
                }
                16 => {
                    let mask: u16 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                    quote! {
                        pub const #mask_const: u16 = #mask;
                    }
                }
                32 => {
                    let mask: u32 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                    quote! {
                        pub const #mask_const: u32 = #mask;
                    }
                }
                64 => {
                    let mask: u64 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                    quote! {
                        pub const #mask_const: u64 = #mask;
                    }
                }
                128 => {
                    let mask: u128 = (offset..offset + self.bits).map(|offset| 1 << offset).sum();
                    quote! {
                        pub const #mask_const: u128 = #mask;
                    }
                }
                _ => panic!(),
            })
    }

    fn mask_const_ident(&self) -> Option<Ident> {
        self.const_ident("MASK")
    }

    fn mask_read(&self, structure: &Structure) -> Option<TokenStream> {
        self.mask_read_ident()
            .zip(self.mask_const_ident())
            .map(|(mask_read, mask_const)| {
                let inner_type: Ident = structure.inner_type();
                quote! {
                    pub fn #mask_read(self) -> #inner_type {
                        self.0 & Self::#mask_const
                    }
                }
            })
    }

    fn mask_read_ident(&self) -> Option<Ident> {
        self.function_ident("mask_read")
    }

    fn mask_update(&self, structure: &Structure) -> Option<TokenStream> {
        self.mask_update_ident()
            .zip(self.mask_const_ident())
            .map(|(mask_update, mask_const)| {
                let inner_type: Ident = structure.inner_type();
                quote! {
                    pub fn #mask_update(self, argument: #inner_type) -> Self {
                        Self((self.0 & !Self::#mask_const) | (argument & Self::#mask_const))
                    }
                }
            })
    }

    fn mask_update_ident(&self) -> Option<Ident> {
        self.function_ident("mask_update")
    }

    fn offset(&self, offset: u8) -> Option<TokenStream> {
        self.offset_ident().map(|offset_const| {
            quote! {
                pub const #offset_const: u8 = #offset;
            }
        })
    }

    fn offset_ident(&self) -> Option<Ident> {
        self.const_ident("OFFSET")
    }

    fn shift_read(&self, structure: &Structure) -> Option<TokenStream> {
        if let (Some(shift_read), Some(mask_read), Some(offset)) = (
            self.shift_read_ident(),
            self.mask_read_ident(),
            self.offset_ident(),
        ) {
            Some((shift_read, mask_read, offset))
        } else {
            None
        }
        .map(|(shift_read, mask_read, offset)| {
            let inner_type: Ident = structure.inner_type();
            quote! {
                pub fn #shift_read(self) -> #inner_type {
                    self.#mask_read() >> Self::#offset
                }
            }
        })
    }

    fn shift_read_ident(&self) -> Option<Ident> {
        self.function_ident("shift_read")
    }

    fn shift_update(&self, structure: &Structure) -> Option<TokenStream> {
        if let (Some(shift_update), Some(mask_update), Some(offset)) = (
            self.shift_update_ident(),
            self.mask_update_ident(),
            self.offset_ident(),
        ) {
            Some((shift_update, mask_update, offset))
        } else {
            None
        }
        .map(|(shift_update, mask_update, offset)| {
            let inner_type: Ident = structure.inner_type();
            quote! {
                pub fn #shift_update(self, argument: #inner_type) -> Self {
                    self.#mask_update(argument << Self::#offset)
                }
            }
        })
    }

    fn shift_update_ident(&self) -> Option<Ident> {
        self.function_ident("shift_update")
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

    fn uint_read(&self) -> Option<TokenStream> {
        let bits: u8 = self.bits;
        self.uint_read_ident()
            .zip(self.shift_read_ident())
            .map(|(uint_read, shift_read)| {
                let return_type: Ident = Ident::new(
                    match bits {
                        8 => "u8",
                        16 => "u16",
                        32 => "u32",
                        64 => "u64",
                        128 => "u128",
                        _ => panic!(),
                    },
                    uint_read.span(),
                );
                quote! {
                    pub fn #uint_read(self) -> #return_type {
                        self.#shift_read() as #return_type
                    }
                }
            })
    }

    fn uint_read_ident(&self) -> Option<Ident> {
        let bits: u8 = self.bits;
        (bits.is_power_of_two() && (8..=128).contains(&bits))
            .then(|| self.function_ident("uint_read"))
            .flatten()
    }

    fn uint_update(&self, structure: &Structure) -> Option<TokenStream> {
        let bits: u8 = self.bits;
        self.uint_update_ident().zip(self.shift_update_ident()).map(
            |(uint_update, shift_update)| {
                let argument_type: Ident = Ident::new(
                    match bits {
                        8 => "u8",
                        16 => "u16",
                        32 => "u32",
                        64 => "u64",
                        128 => "u128",
                        _ => panic!(),
                    },
                    uint_update.span(),
                );
                let inner_type: Ident = structure.inner_type();
                quote! {
                    pub fn #uint_update(self, argument: #argument_type) -> Self {
                        self.#shift_update(argument as #inner_type)
                    }
                }
            },
        )
    }

    fn uint_update_ident(&self) -> Option<Ident> {
        let bits: u8 = self.bits;
        (bits.is_power_of_two() && (8..=128).contains(&bits))
            .then(|| self.function_ident("uint_update"))
            .flatten()
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
            let ident: Option<Ident> = (ident != "__").then_some(ident);
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
