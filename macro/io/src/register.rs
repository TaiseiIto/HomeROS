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

    fn bits_reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .map(|element| element.bits_read())
            .collect()
    }

    fn bits_updates(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .map(|element| element.bits_update())
            .collect()
    }

    fn debug(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        quote! {
            impl core::fmt::Debug for #ident {
                fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    unsafe {
                        self.read_memory()
                    }.fmt(formatter)
                }
            }
        }
    }

    fn implement(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let bits_reads: Vec<TokenStream> = self.bits_reads();
        let bits_updates: Vec<TokenStream> = self.bits_updates();
        let lengths: Vec<TokenStream> = self.lengths();
        let mask_consts: Vec<TokenStream> = self.mask_consts();
        let offsets: Vec<TokenStream> = self.offsets();
        let prettify: TokenStream = self.prettify();
        let read_memory: TokenStream = self.read_memory();
        let write_memory: TokenStream = self.write_memory();
        quote! {
            impl #ident {
                #(#bits_reads)*
                #(#bits_updates)*
                #(#lengths)*
                #(#mask_consts)*
                #(#offsets)*
                #prettify
                #read_memory
                #write_memory
            }
        }
    }

    fn inner_type(&self) -> Ident {
        Ident::new(&format!("u{}", self.bits()), self.ident.span())
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

    fn prettify(&self) -> TokenStream {
        let pretty_structure: Ident = self.pretty_structure_ident();
        let prettify_elements: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| element.prettify())
            .collect();
        quote! {
            fn prettify(self) -> #pretty_structure {
                #pretty_structure {
                    #(#prettify_elements),*
                }
            }
        }
    }

    fn pretty_bit_reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.pretty_bit_read())
            .collect()
    }

    fn pretty_bit_updates(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.pretty_bit_update(self))
            .collect()
    }

    fn pretty_bits_reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.pretty_bits_read())
            .collect()
    }

    fn pretty_bits_updates(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.pretty_bits_update(self))
            .collect()
    }

    fn pretty_debug(&self) -> TokenStream {
        let pretty_structure: Ident = self.pretty_structure_ident();
        let ident_string: String = self.ident.to_string();
        let elements: Vec<TokenStream> = self
            .elements
            .iter()
            .filter_map(|element| {
                element
                    .pretty_bit_read_ident()
                    .or(element
                        .pretty_uint_read_ident_and_type()
                        .map(|(ident, _)| ident))
                    .or(element.pretty_shift_read_ident())
                    .map(|read| {
                        let ident_string: String = element.ident.to_string();
                        quote! {
                            field(#ident_string, &self.#read())
                        }
                    })
            })
            .collect();
        quote! {
            impl core::fmt::Debug for #pretty_structure {
                fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    formatter
                        .debug_struct(#ident_string)
                        .#(#elements).*
                        .finish()
                }
            }
        }
    }

    fn pretty_implement(&self) -> TokenStream {
        let pretty_structure: Ident = self.pretty_structure_ident();
        let pretty_bit_reads: Vec<TokenStream> = self.pretty_bit_reads();
        let pretty_bit_updates: Vec<TokenStream> = self.pretty_bit_updates();
        let pretty_bits_reads: Vec<TokenStream> = self.pretty_bits_reads();
        let pretty_bits_updates: Vec<TokenStream> = self.pretty_bits_updates();
        let pretty_mask_reads: Vec<TokenStream> = self.pretty_mask_reads();
        let pretty_mask_updates: Vec<TokenStream> = self.pretty_mask_updates();
        let pretty_shift_reads: Vec<TokenStream> = self.pretty_shift_reads();
        let pretty_shift_updates: Vec<TokenStream> = self.pretty_shift_updates();
        let pretty_uint_reads: Vec<TokenStream> = self.pretty_uint_reads();
        let pretty_uint_updates: Vec<TokenStream> = self.pretty_uint_updates();
        let read_port: Option<TokenStream> = self.read_port();
        let unprettify: TokenStream = self.unprettify();
        let write_port: Option<TokenStream> = self.write_port();
        quote! {
            impl #pretty_structure {
                #(#pretty_bit_reads)*
                #(#pretty_bit_updates)*
                #(#pretty_bits_reads)*
                #(#pretty_bits_updates)*
                #(#pretty_mask_reads)*
                #(#pretty_mask_updates)*
                #(#pretty_shift_reads)*
                #(#pretty_shift_updates)*
                #(#pretty_uint_reads)*
                #(#pretty_uint_updates)*
                #read_port
                #unprettify
                #write_port
            }
        }
    }

    fn pretty_mask_reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.pretty_mask_read(self))
            .collect()
    }

    fn pretty_mask_updates(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.pretty_mask_update(self))
            .collect()
    }

    fn pretty_shift_reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.pretty_shift_read(self))
            .collect()
    }

    fn pretty_shift_updates(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.pretty_shift_update(self))
            .collect()
    }

    fn pretty_uint_reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.pretty_uint_read())
            .collect()
    }

    fn pretty_uint_updates(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.pretty_uint_update(self))
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
            #[derive(Default)]
            #[repr(transparent)]
            #vis struct #ident(#inner_type);
        }
    }

    fn unprettify(&self) -> TokenStream {
        let Self {
            vis: _,
            ident,
            elements,
        } = self;
        let unprettifies: Vec<TokenStream> = elements
            .iter()
            .filter_map(|element| element.unprettify())
            .collect();
        quote! {
            fn unprettify(self) -> #ident {
                #ident::default().#(#unprettifies).*
            }
        }
    }

    fn read_port(&self) -> Option<TokenStream> {
        let structure: &Ident = &self.ident;
        let inner_type: Ident = self.inner_type();
        match inner_type.to_string().as_str() {
            "u8" => Some(quote! {
                "in dx, al", in("dx") port, out("al") value
            }),
            "u16" => Some(quote! {
                "in dx, ax", in("dx") port, out("ax") value
            }),
            "u32" => Some(quote! {
                "in dx, eax", in("dx") port, out("eax") value
            }),
            _ => None,
        }
        .map(|asm| {
            quote! {
                #[cfg(target_arch = "x86_64")]
                pub unsafe fn read_port(port: u16) -> Self {
                    let mut value: #inner_type;
                    unsafe {
                        core::arch::asm!(#asm);
                    }
                    #structure(value).prettify()
                }
            }
        })
    }

    fn read_memory(&self) -> TokenStream {
        let pretty_structure: Ident = self.pretty_structure_ident();
        quote! {
            pub unsafe fn read_memory(&self) -> #pretty_structure {
                unsafe {
                    core::ptr::read_volatile(self as *const Self)
                }.prettify()
            }
        }
    }

    fn write_port(&self) -> Option<TokenStream> {
        let structure: &Ident = &self.ident;
        let inner_type: Ident = self.inner_type();
        match inner_type.to_string().as_str() {
            "u8" => Some(quote! {
                "out dx, al", in("dx") port, in("al") value
            }),
            "u16" => Some(quote! {
                "out dx, ax", in("dx") port, in("ax") value
            }),
            "u32" => Some(quote! {
                "out dx, eax", in("dx") port, in("eax") value
            }),
            _ => None,
        }
        .map(|asm| {
            quote! {
                #[cfg(target_arch = "x86_64")]
                pub unsafe fn write_port(self, port: u16) {
                    let value: #inner_type = self.unprettify().0;
                    unsafe {
                        core::arch::asm!(#asm);
                    }
                }
            }
        })
    }

    fn write_memory(&self) -> TokenStream {
        let pretty_structure: Ident = self.pretty_structure_ident();
        quote! {
            pub unsafe fn write_memory(&mut self, argument: #pretty_structure) {
                let argument: Self = argument.unprettify();
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
        let debug: TokenStream = structure.debug();
        let pretty_implement: TokenStream = structure.pretty_implement();
        let pretty_structure: TokenStream = structure.pretty_structure();
        let pretty_debug: TokenStream = structure.pretty_debug();
        quote! {
            #true_type
            #implement
            #debug
            #pretty_structure
            #pretty_implement
            #pretty_debug
        }
    }
}

struct Element {
    bits: u8,
    ident: Ident,
    reserved: bool,
}

impl Element {
    fn bits_read(&self) -> TokenStream {
        let bits: u8 = self.bits;
        let bits_read: Ident = self.bits_read_ident();
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
            fn #bits_read(&self) -> [bool; #bits_usize] {
                [#(#bools),*]
            }
        }
    }

    fn bits_read_ident(&self) -> Ident {
        self.function_ident("bits_read")
    }

    fn bits_update(&self) -> TokenStream {
        let bits_update: Ident = self.bits_update_ident();
        let mask_const: Ident = self.mask_const_ident();
        let offset: Ident = self.offset_ident();
        let bits: u8 = self.bits;
        let bits_usize: usize = bits as usize;
        let values: Vec<TokenStream> = (0..bits)
            .map(|bit| {
                let bit_usize: usize = bit as usize;
                quote! {
                    if argument[#bit_usize] { 1 << (#bit + Self::#offset) } else { 0 }
                }
            })
            .collect();
        quote! {
            fn #bits_update(self, argument: [bool; #bits_usize]) -> Self {
                Self((self.0 & !Self::#mask_const) | ((#(#values)|*) & Self::#mask_const))
            }
        }
    }

    fn bits_update_ident(&self) -> Ident {
        self.function_ident("bits_update")
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

    fn prettify(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let bits_read: Ident = self.bits_read_ident();
        quote! {
            #ident: self.#bits_read()
        }
    }

    fn pretty(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let bits: usize = self.bits as usize;
        quote! {
            #ident: [bool; #bits]
        }
    }

    fn pretty_bit_read(&self) -> Option<TokenStream> {
        self.pretty_bit_read_ident().map(|pretty_bit_read| {
            let ident: &Ident = &self.ident;
            quote! {
                pub fn #pretty_bit_read(&self) -> bool {
                    self.#ident[0]
                }
            }
        })
    }

    fn pretty_bit_read_ident(&self) -> Option<Ident> {
        (!self.reserved && self.bits == 1).then_some(self.function_ident("bit_read"))
    }

    fn pretty_bit_update(&self, structure: &Structure) -> Option<TokenStream> {
        self.pretty_bit_update_ident().map(|pretty_bits_update| {
            let unpack: Vec<TokenStream> = structure
                .elements
                .iter()
                .map(|element| {
                    let ident: &Ident = &element.ident;
                    if *ident == self.ident {
                        quote! {
                            #ident: _
                        }
                    } else {
                        quote! {
                            #ident
                        }
                    }
                })
                .collect();
            let pack: Vec<Ident> = structure
                .elements
                .iter()
                .map(|element| element.ident.clone())
                .collect();
            let ident: &Ident = &self.ident;
            quote! {
                pub fn #pretty_bits_update(self, #ident: bool) -> Self {
                    let #ident: [bool; 1] = [#ident];
                    let Self {#(#unpack),*} = self;
                    Self {#(#pack),*}
                }
            }
        })
    }

    fn pretty_bit_update_ident(&self) -> Option<Ident> {
        (!self.reserved && self.bits == 1).then_some(self.function_ident("bit_update"))
    }

    fn pretty_bits_read(&self) -> Option<TokenStream> {
        self.pretty_bits_read_ident().map(|pretty_bits_read| {
            let ident: &Ident = &self.ident;
            let bits: usize = self.bits as usize;
            quote! {
                pub fn #pretty_bits_read(&self) -> [bool; #bits] {
                    self.#ident
                }
            }
        })
    }

    fn pretty_bits_read_ident(&self) -> Option<Ident> {
        (!self.reserved).then_some(self.function_ident("bits_read"))
    }

    fn pretty_bits_update(&self, structure: &Structure) -> Option<TokenStream> {
        self.pretty_bits_update_ident().map(|pretty_bits_update| {
            let bits: usize = self.bits as usize;
            let unpack: Vec<TokenStream> = structure
                .elements
                .iter()
                .map(|element| {
                    let ident: &Ident = &element.ident;
                    if *ident == self.ident {
                        quote! {
                            #ident: _
                        }
                    } else {
                        quote! {
                            #ident
                        }
                    }
                })
                .collect();
            let pack: Vec<Ident> = structure
                .elements
                .iter()
                .map(|element| element.ident.clone())
                .collect();
            let ident: &Ident = &self.ident;
            quote! {
                pub fn #pretty_bits_update(self, #ident: [bool; #bits]) -> Self {
                    let Self {#(#unpack),*} = self;
                    Self {#(#pack),*}
                }
            }
        })
    }

    fn pretty_bits_update_ident(&self) -> Option<Ident> {
        (!self.reserved).then_some(self.function_ident("bits_update"))
    }

    fn pretty_mask_read(&self, structure: &Structure) -> Option<TokenStream> {
        self.pretty_mask_read_ident()
            .zip(self.pretty_shift_read_ident())
            .map(|(pretty_mask_read, pretty_shift_read)| {
                let return_type: Ident = structure.inner_type();
                let structure: &Ident = &structure.ident;
                let offset: Ident = self.offset_ident();
                quote! {
                    pub fn #pretty_mask_read(&self) -> #return_type {
                        self.#pretty_shift_read() << #structure::#offset
                    }
                }
            })
    }

    fn pretty_mask_read_ident(&self) -> Option<Ident> {
        (!self.reserved).then_some(self.function_ident("mask_read"))
    }

    fn pretty_mask_update(&self, structure: &Structure) -> Option<TokenStream> {
        self.pretty_mask_update_ident()
            .zip(self.pretty_shift_update_ident())
            .map(|(pretty_mask_update, pretty_shift_update)| {
                let ident: &Ident = &self.ident;
                let argument_type: Ident = structure.inner_type();
                let structure: &Ident = &structure.ident;
                let offset: Ident = self.offset_ident();
                quote! {
                    pub fn #pretty_mask_update(self, #ident: #argument_type) -> Self {
                        self.#pretty_shift_update(#ident << #structure::#offset)
                    }
                }
            })
    }

    fn pretty_mask_update_ident(&self) -> Option<Ident> {
        (!self.reserved).then_some(self.function_ident("mask_update"))
    }

    fn pretty_shift_read(&self, structure: &Structure) -> Option<TokenStream> {
        self.pretty_shift_read_ident().map(|pretty_shift_read| {
            let Self {
                bits,
                ident,
                reserved: _,
            } = self;
            let return_type: Ident = structure.inner_type();
            let bits: Vec<TokenStream> = (0..*bits)
                .map(|shift| {
                    let shift_usize: usize = shift as usize;
                    quote! {
                        (if self.#ident[#shift_usize] {
                            1 << #shift
                        } else {
                            0
                        })
                    }
                })
                .collect();
            quote! {
                pub fn #pretty_shift_read(&self) -> #return_type {
                    #(#bits)|*
                }
            }
        })
    }

    fn pretty_shift_read_ident(&self) -> Option<Ident> {
        (!self.reserved).then_some(self.function_ident("shift_read"))
    }

    fn pretty_shift_update(&self, structure: &Structure) -> Option<TokenStream> {
        self.pretty_shift_update_ident()
            .zip(self.pretty_bits_update_ident())
            .map(|(pretty_shift_update, pretty_bits_update)| {
                let Self {
                    bits,
                    ident,
                    reserved: _,
                } = self;
                let argument_type: Ident = structure.inner_type();
                let bits_usize: usize = *bits as usize;
                let bools: Vec<TokenStream> = (0..bits_usize)
                    .map(|shift| {
                        quote! {
                            #ident & (1 << #shift) != 0
                        }
                    })
                    .collect();
                quote! {
                    pub fn #pretty_shift_update(self, #ident: #argument_type) -> Self {
                        let argument: [bool; #bits_usize] = [#(#bools),*];
                        self.#pretty_bits_update(argument)
                    }
                }
            })
    }

    fn pretty_shift_update_ident(&self) -> Option<Ident> {
        (!self.reserved).then_some(self.function_ident("shift_update"))
    }

    fn pretty_uint_read(&self) -> Option<TokenStream> {
        self.pretty_uint_read_ident_and_type()
            .zip(self.pretty_shift_read_ident())
            .map(|((pretty_uint_read, return_type), pretty_shift_read)| {
                quote! {
                    pub fn #pretty_uint_read(&self) -> #return_type {
                        self.#pretty_shift_read() as #return_type
                    }
                }
            })
    }

    fn pretty_uint_read_ident_and_type(&self) -> Option<(Ident, Ident)> {
        let Self {
            bits,
            ident,
            reserved,
        } = self;
        (!reserved && (8..=128).contains(bits) && bits.is_power_of_two()).then_some({
            let return_type: String = format!("u{}", bits);
            (
                self.function_ident(&format!("{}_read", return_type)),
                Ident::new(&return_type, ident.span()),
            )
        })
    }

    fn pretty_uint_update(&self, structure: &Structure) -> Option<TokenStream> {
        self.pretty_uint_update_ident_and_type()
            .zip(self.pretty_shift_update_ident())
            .map(
                |((pretty_uint_update, argument_type), pretty_shift_update)| {
                    let inner_type: Ident = structure.inner_type();
                    quote! {
                        pub fn #pretty_uint_update(self, argument: #argument_type) -> Self {
                            self.#pretty_shift_update(argument as #inner_type)
                        }
                    }
                },
            )
    }

    fn pretty_uint_update_ident_and_type(&self) -> Option<(Ident, Ident)> {
        let Self {
            bits,
            ident,
            reserved,
        } = self;
        (!reserved && (8..=128).contains(bits) && bits.is_power_of_two()).then_some({
            let argument_type: String = format!("u{}", bits);
            (
                self.function_ident(&format!("{}_update", argument_type)),
                Ident::new(&argument_type, ident.span()),
            )
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

    fn unprettify(&self) -> Option<TokenStream> {
        let bits_update: Ident = self.bits_update_ident();
        let Self {
            bits: _,
            ident,
            reserved,
        } = self;
        (!reserved).then_some(quote! {
            #bits_update(self.#ident)
        })
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
