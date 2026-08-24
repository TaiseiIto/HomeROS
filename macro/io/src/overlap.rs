use {
    proc_macro2::TokenStream,
    quote::quote,
    std::iter,
    syn::{
        Field, FieldsNamed, Ident, ItemUnion, Path, PathSegment, Type, TypePath, Visibility,
        punctuated::Punctuated, token::PathSep,
    },
};

pub struct Registers {
    elements: Vec<Element>,
    ident: Ident,
    vis: Visibility,
}

impl Registers {
    fn prettify(&self) -> TokenStream {
        let pretty_type: Ident = self.pretty_ident();
        let reader_type: Ident = self.reader_ident();
        let elements: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| element.prettify())
            .collect();
        quote! {
            pub fn prettify(self) -> #pretty_type {
                #pretty_type::Reader(#reader_type {
                    #(#elements),*
                })
            }
        }
    }

    fn pretty_declaration(&self) -> TokenStream {
        let vis: &Visibility = &self.vis;
        let pretty_type: Ident = self.pretty_ident();
        let reader_type: Ident = self.reader_ident();
        let writer_type: Ident = self.writer_ident();
        quote! {
            #[derive(Clone, Debug)]
            #vis enum #pretty_type {
                Reader(#reader_type),
                Writer(#writer_type),
            }
        }
    }

    fn pretty_ident(&self) -> Ident {
        self.ident.clone()
    }

    fn pretty_implement(&self) -> TokenStream {
        let pretty_type: Ident = self.pretty_ident();
        let pretty_reads: Vec<TokenStream> = self.pretty_reads();
        let pretty_writes: Vec<TokenStream> = self.pretty_writes();
        let unprettify: TokenStream = self.unprettify();
        quote! {
            impl #pretty_type {
                #(#pretty_reads)*
                #(#pretty_writes)*
                #unprettify
            }
        }
    }

    fn pretty_reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .map(|element| element.pretty_read())
            .collect()
    }

    fn pretty_writes(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .map(|element| element.pretty_write(self))
            .collect()
    }

    fn reader_declaration(&self) -> TokenStream {
        let Self {
            elements,
            ident: _,
            vis,
        } = self;
        let reader_type: Ident = self.reader_ident();
        let reader_elements: Vec<TokenStream> = elements
            .iter()
            .map(|element| element.reader_declaration())
            .collect();
        quote! {
            #[derive(Clone, Debug)]
            #vis struct #reader_type {
                #(#reader_elements),*
            }
        }
    }

    fn reader_ident(&self) -> Ident {
        self.type_ident("Reader")
    }

    /// # TODO
    /// Add memory barier.
    fn read_memory(&self) -> TokenStream {
        let pretty_type: Ident = self.pretty_ident();
        quote! {
            pub unsafe fn read_memory(&self) -> #pretty_type {
                unsafe {
                    ::core::ptr::read_volatile(self as *const Self)
                }.prettify()
            }
        }
    }

    fn read_port(&self) -> TokenStream {
        let pretty_type: Ident = self.pretty_ident();
        quote! {
            #[cfg(target_arch = "x86_64")]
            pub unsafe fn read_port(port: u16) -> #pretty_type {
                match ::core::mem::size_of::<Self>() {
                    1 => {
                        let mut buffer: u8;
                        unsafe {
                            ::core::arch::asm!("in al, dx", out("al") buffer, in("dx") port);
                            ::core::ptr::read_volatile((&buffer as *const u8) as *const Self)
                        }.prettify()
                    },
                    2 => {
                        let mut buffer: u16;
                        unsafe {
                            ::core::arch::asm!("in ax, dx", out("ax") buffer, in("dx") port);
                            ::core::ptr::read_volatile((&buffer as *const u16) as *const Self)
                        }.prettify()
                    },
                    4 => {
                        let mut buffer: u32;
                        unsafe {
                            ::core::arch::asm!("in eax, dx", out("eax") buffer, in("dx") port);
                            ::core::ptr::read_volatile((&buffer as *const u32) as *const Self)
                        }.prettify()
                    },
                    8 => {
                        let mut buffer: u64 = 0;
                        for current_port in (port..)
                            .step_by(::core::mem::size_of::<u32>())
                            .take_while(|current_port| (*current_port as usize) < (port as usize) + ::core::mem::size_of::<u64>()) {
                            let mut current_buffer: u32;
                            unsafe {
                                ::core::arch::asm!("in eax, dx", out("eax") current_buffer, in("dx") current_port);
                            }
                            buffer += (current_buffer as u64) << (((current_port - port) as u32) * u8::BITS);
                        }
                        unsafe {
                            ::core::ptr::read_volatile((&buffer as *const u64) as *const Self)
                        }.prettify()
                    },
                    16 => {
                        let mut buffer: u128 = 0;
                        for current_port in (port..)
                            .step_by(::core::mem::size_of::<u32>())
                            .take_while(|current_port| (*current_port as usize) < (port as usize) + ::core::mem::size_of::<u128>()) {
                            let mut current_buffer: u32;
                            unsafe {
                                ::core::arch::asm!("in eax, dx", out("eax") current_buffer, in("dx") current_port);
                            }
                            buffer += (current_buffer as u128) << (((current_port - port) as u32) * u8::BITS);
                        }
                        unsafe {
                            ::core::ptr::read_volatile((&buffer as *const u128) as *const Self)
                        }.prettify()
                    },
                    _ => panic!(),
                }
            }
        }
    }

    fn true_declaration(&self) -> TokenStream {
        let vis: &Visibility = &self.vis;
        let true_type: Ident = self.true_type();
        let elements: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| element.true_declaration())
            .collect();
        quote! {
            #[repr(C)]
            #vis union #true_type {
                #(#elements),*
            }
        }
    }

    fn true_implement(&self) -> TokenStream {
        let true_type: Ident = self.true_type();
        let prettify: TokenStream = self.prettify();
        let read_memory: TokenStream = self.read_memory();
        let read_port: TokenStream = self.read_port();
        let write_memory: TokenStream = self.write_memory();
        let write_port: TokenStream = self.write_port();
        quote! {
            impl #true_type {
                #prettify
                #read_memory
                #read_port
                #write_memory
                #write_port
            }
        }
    }

    fn true_type(&self) -> Ident {
        self.type_ident("Raw")
    }

    fn type_ident(&self, suffix: &str) -> Ident {
        let ident: &Ident = &self.ident;
        Ident::new(&format!("{}{}", ident, suffix), ident.span())
    }

    fn unprettify(&self) -> TokenStream {
        let true_type: Ident = self.true_type();
        let pretty_type: Ident = self.pretty_ident();
        let elements: Vec<TokenStream> = self
            .elements
            .iter()
            .map(|element| element.unprettify(self))
            .collect();
        quote! {
            pub fn unprettify(self) -> #true_type {
                if let #pretty_type::Writer(writer) = self {
                    match writer {
                        #(#elements),*
                    }
                } else {
                    panic!();
                }
            }
        }
    }

    /// # TODO
    /// Add memory barier.
    fn write_memory(&self) -> TokenStream {
        let pretty_type: Ident = self.pretty_ident();
        quote! {
            pub unsafe fn write_memory(&mut self, value: #pretty_type) {
                unsafe {
                    ::core::ptr::write_volatile(self as *mut Self, value.unprettify());
                }
            }
        }
    }

    fn write_port(&self) -> TokenStream {
        let pretty_type: Ident = self.pretty_ident();
        quote! {
            #[cfg(target_arch = "x86_64")]
            pub unsafe fn write_port(port: u16, value: #pretty_type) {
                let value: *const Self = (&value.unprettify()) as *const Self;
                match ::core::mem::size_of::<Self>() {
                    1 => {
                        let value: *const u8 = value as *const u8;
                        let value: u8 = unsafe { *value };
                        unsafe {
                            ::core::arch::asm!("out dx, al", in("dx") port, in("al") value);
                        }
                    },
                    2 => {
                        let value: *const u16 = value as *const u16;
                        let value: u16 = unsafe { *value };
                        unsafe {
                            ::core::arch::asm!("out dx, ax", in("dx") port, in("ax") value);
                        }
                    },
                    4 => {
                        let value: *const u32 = value as *const u32;
                        let value: u32 = unsafe { *value };
                        unsafe {
                            ::core::arch::asm!("out dx, eax", in("dx") port, in("eax") value);
                        }
                    },
                    8 => {
                        let value: *const u64 = value as *const u64;
                        let value: u64 = unsafe { *value };
                        for current_port in (port..)
                            .step_by(::core::mem::size_of::<u32>())
                            .take_while(|current_port| (*current_port as usize) < (port as usize) + ::core::mem::size_of::<u64>()) {
                            let buffer: u32 = ((value >> (((current_port - port) as u32) * u8::BITS)) & 0xffffffff) as u32;
                            unsafe {
                                ::core::arch::asm!("out dx, eax", in("dx") current_port, in("eax") buffer);
                            }
                        }
                    },
                    16 => {
                        let value: *const u128 = value as *const u128;
                        let value: u128 = unsafe { *value };
                        for current_port in (port..)
                            .step_by(::core::mem::size_of::<u32>())
                            .take_while(|current_port| (*current_port as usize) < (port as usize) + ::core::mem::size_of::<u128>()) {
                            let buffer: u32 = ((value >> (((current_port - port) as u32) * u8::BITS)) & 0xffffffff) as u32;
                            unsafe {
                                ::core::arch::asm!("out dx, eax", in("dx") current_port, in("eax") buffer);
                            }
                        }
                    },
                    _ => panic!(),
                }
            }
        }
    }

    fn writer_declaration(&self) -> TokenStream {
        let Self {
            elements,
            ident: _,
            vis,
        } = self;
        let writer_type: Ident = self.writer_ident();
        let elements: Vec<TokenStream> = elements
            .iter()
            .map(|element| element.writer_declaration())
            .collect();
        quote! {
            #[derive(Clone, Debug)]
            #vis enum #writer_type {
                #(#elements),*
            }
        }
    }

    fn writer_ident(&self) -> Ident {
        self.type_ident("Writer")
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
        let pretty_declaration: TokenStream = registers.pretty_declaration();
        let pretty_implement: TokenStream = registers.pretty_implement();
        let reader_declaration: TokenStream = registers.reader_declaration();
        let true_declaration: TokenStream = registers.true_declaration();
        let true_implement: TokenStream = registers.true_implement();
        let writer_declaration: TokenStream = registers.writer_declaration();
        quote! {
            #pretty_declaration
            #pretty_implement
            #reader_declaration
            #true_declaration
            #true_implement
            #writer_declaration
        }
    }
}

struct Element {
    ident: Ident,
    ty: Type,
}

impl Element {
    fn function_ident(&self, prefix: &str) -> Ident {
        let ident: &Ident = &self.ident;
        Ident::new(&format!("{}_{}", prefix, ident), ident.span())
    }

    fn prettify(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        quote! {
            #ident: unsafe {self.#ident.read_memory()}
        }
    }

    fn pretty_read(&self) -> TokenStream {
        let pretty_read: Ident = self.pretty_read_ident();
        let pretty_type: Type = self.pretty_type();
        let ident: &Ident = &self.ident;
        quote! {
            pub fn #pretty_read(&self) -> #pretty_type {
                if let Self::Reader(reader) = self {
                    reader.#ident.clone()
                } else {
                    panic!();
                }
            }
        }
    }

    fn pretty_read_ident(&self) -> Ident {
        self.function_ident("read")
    }

    fn pretty_type(&self) -> Type {
        self.ty.clone()
    }

    fn pretty_write(&self, registers: &Registers) -> TokenStream {
        let pretty_write: Ident = self.pretty_write_ident();
        let pretty_type: Type = self.pretty_type();
        let writer: Ident = self.writer_ident();
        let writer_type: Ident = registers.writer_ident();
        quote! {
            pub fn #pretty_write(value: #pretty_type) -> Self {
                Self::Writer(#writer_type::#writer(value))
            }
        }
    }

    fn pretty_write_ident(&self) -> Ident {
        self.function_ident("write")
    }

    fn reader_declaration(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let pretty_type: Type = self.pretty_type();
        quote! {
            #ident: #pretty_type
        }
    }

    fn true_declaration(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let true_type: Type = self.true_type();
        quote! {
            #ident: ::core::mem::ManuallyDrop<#true_type>
        }
    }

    fn true_type(&self) -> Type {
        if let Type::Path(TypePath {
            attrs,
            qself,
            path: Path {
                leading_colon,
                segments,
            },
        }) = &self.ty
        {
            let mut segments: Punctuated<PathSegment, PathSep> = segments.clone();
            if let Some(last_segment) = segments.last_mut() {
                let ident: &mut Ident = &mut last_segment.ident;
                *ident = Ident::new(&format!("{}Raw", ident), ident.span());
            } else {
                panic!();
            }
            Type::Path(TypePath {
                attrs: attrs.clone(),
                qself: qself.clone(),
                path: Path {
                    leading_colon: *leading_colon,
                    segments,
                },
            })
        } else {
            panic!();
        }
    }

    fn unprettify(&self, registers: &Registers) -> TokenStream {
        let element_ident: &Ident = &self.ident;
        let true_type: Ident = registers.true_type();
        let writer: Ident = self.writer_ident();
        let writer_type: Ident = registers.writer_ident();
        quote! {
            #writer_type::#writer(writer) => #true_type {
                #element_ident : ::core::mem::ManuallyDrop::new(writer.unprettify())
            }
        }
    }

    fn writer_declaration(&self) -> TokenStream {
        let writer: Ident = self.writer_ident();
        let pretty_type: Type = self.pretty_type();
        quote! {
            #writer(#pretty_type)
        }
    }

    fn writer_ident(&self) -> Ident {
        let ident: &Ident = &self.ident;
        let writer: String = ident
            .to_string()
            .split('_')
            .map(|word| {
                word.chars()
                    .enumerate()
                    .map(|(index, character)| match index {
                        0 => character.to_uppercase().collect::<String>(),
                        _ => iter::once(character).collect::<String>(),
                    })
                    .collect::<String>()
            })
            .collect();
        Ident::new(writer.as_str(), ident.span())
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
