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
            fn prettify(self) -> #pretty_type {
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
            #vis enum #pretty_type {
                Reader(#reader_type),
                Writer(#writer_type),
            }
        }
    }

    fn pretty_ident(&self) -> Ident {
        self.type_ident("Pretty")
    }

    fn pretty_implement(&self) -> TokenStream {
        let pretty_type: Ident = self.pretty_ident();
        let pretty_reads: Vec<TokenStream> = self.pretty_reads();
        let unprettify: TokenStream = self.unprettify();
        quote! {
            impl #pretty_type {
                #(#pretty_reads)*
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
            #vis struct #reader_type {
                #(#reader_elements),*
            }
        }
    }

    fn reader_ident(&self) -> Ident {
        self.type_ident("Reader")
    }

    fn read_memory(&self) -> TokenStream {
        let pretty_type: Ident = self.pretty_ident();
        quote! {
            pub unsafe fn read_memory(&self) -> #pretty_type {
                unsafe {
                    core::ptr::read_volatile(self as *const Self)
                }.prettify()
            }
        }
    }

    fn read_port(&self) -> TokenStream {
        let pretty_type: Ident = self.pretty_ident();
        quote! {
            #[cfg(target_arch = "x86_64")]
            pub unsafe fn read_port(port: u16) -> #pretty_type {
                match core::mem::size_of::<Self>() {
                    1 => {
                        let mut buffer: u8;
                        unsafe {
                            core::arch::asm!("in dx, al", in("dx") port, out("al") buffer);
                            core::ptr::read_volatile((&buffer as *const u8) as *const Self)
                        }.prettify()
                    }
                    2 => {
                        let mut buffer: u16;
                        unsafe {
                            core::arch::asm!("in dx, ax", in("dx") port, out("ax") buffer);
                            core::ptr::read_volatile((&buffer as *const u16) as *const Self)
                        }.prettify()
                    }
                    4 => {
                        let mut buffer: u32;
                        unsafe {
                            core::arch::asm!("in dx, eax", in("dx") port, out("eax") buffer);
                            core::ptr::read_volatile((&buffer as *const u32) as *const Self)
                        }.prettify()
                    }
                    8 => {
                        let port0: u16 = port;
                        let port1: u16 = port0 + 4;
                        let mut buffer0: u32;
                        let mut buffer1: u32;
                        unsafe {
                            core::arch::asm!("in dx, eax", in("dx") port0, out("eax") buffer0);
                            core::arch::asm!("in dx, eax", in("dx") port1, out("eax") buffer1);
                        }
                        let buffer: [u32; 2] = [buffer0, buffer1];
                        unsafe {
                            core::ptr::read_volatile((&buffer as *const [u32; 2]) as *const Self)
                        }.prettify()
                    }
                    16 => {
                        let port0: u16 = port;
                        let port1: u16 = port0 + 4;
                        let port2: u16 = port1 + 4;
                        let port3: u16 = port2 + 4;
                        let mut buffer0: u32;
                        let mut buffer1: u32;
                        let mut buffer2: u32;
                        let mut buffer3: u32;
                        unsafe {
                            core::arch::asm!("in dx, eax", in("dx") port0, out("eax") buffer0);
                            core::arch::asm!("in dx, eax", in("dx") port1, out("eax") buffer1);
                            core::arch::asm!("in dx, eax", in("dx") port2, out("eax") buffer2);
                            core::arch::asm!("in dx, eax", in("dx") port3, out("eax") buffer3);
                        }
                        let buffer: [u32; 4] = [buffer0, buffer1, buffer2, buffer3];
                        unsafe {
                            core::ptr::read_volatile((&buffer as *const [u32; 4]) as *const Self)
                        }.prettify()
                    }
                    _ => panic!(),
                }
            }
        }
    }

    fn true_declaration(&self) -> TokenStream {
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

    fn true_implement(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let prettify: TokenStream = self.prettify();
        let read_memory: TokenStream = self.read_memory();
        let read_port: TokenStream = self.read_port();
        let write_memory: TokenStream = self.write_memory();
        let write_port: TokenStream = self.write_port();
        quote! {
            impl #ident {
                #prettify
                #read_memory
                #read_port
                #write_memory
                #write_port
            }
        }
    }

    fn type_ident(&self, suffix: &str) -> Ident {
        let ident: &Ident = &self.ident;
        Ident::new(&format!("{}{}", ident, suffix), ident.span())
    }

    fn unprettify(&self) -> TokenStream {
        let Self {
            elements,
            ident,
            vis: _,
        } = self;
        let pretty_type: Ident = self.pretty_ident();
        let elements: Vec<TokenStream> = elements
            .iter()
            .map(|element| element.unprettify(self))
            .collect();
        quote! {
            fn unprettify(self) -> #ident {
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

    fn write_memory(&self) -> TokenStream {
        let pretty_type: Ident = self.pretty_ident();
        quote! {
            pub unsafe fn write_memory(&mut self, argument: #pretty_type) {
                unsafe {
                    core::ptr::write_volatile(self as *mut Self, argument.unprettify());
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
                match core::mem::size_of::<Self>() {
                    1 => {
                        let value: *const u8 = value as *const u8;
                        let value: u8 = unsafe { *value };
                        unsafe {
                            core::arch::asm!("out dx, al", in("dx") port, in("al") value);
                        }
                    }
                    2 => {
                        let value: *const u16 = value as *const u16;
                        let value: u16 = unsafe { *value };
                        unsafe {
                            core::arch::asm!("out dx, ax", in("dx") port, in("ax") value);
                        }
                    }
                    4 => {
                        let value: *const u32 = value as *const u32;
                        let value: u32 = unsafe { *value };
                        unsafe {
                            core::arch::asm!("out dx, eax", in("dx") port, in("eax") value);
                        }
                    }
                    8 => {
                        let value: *const [u32; 2] = value as *const [u32; 2];
                        let value: [u32; 2] = unsafe { *value };
                        let value0: u32 = value[0];
                        let value1: u32 = value[1];
                        let port0: u16 = port;
                        let port1: u16 = port0 + 4;
                        unsafe {
                            core::arch::asm!("out dx, eax", in("dx") port0, in("eax") value0);
                            core::arch::asm!("out dx, eax", in("dx") port1, in("eax") value1);
                        }
                    }
                    16 => {
                        let value: *const [u32; 4] = value as *const [u32; 4];
                        let value: [u32; 4] = unsafe { *value };
                        let value0: u32 = value[0];
                        let value1: u32 = value[1];
                        let value2: u32 = value[2];
                        let value3: u32 = value[3];
                        let port0: u16 = port;
                        let port1: u16 = port0 + 4;
                        let port2: u16 = port1 + 4;
                        let port3: u16 = port2 + 4;
                        unsafe {
                            core::arch::asm!("out dx, eax", in("dx") port0, in("eax") value0);
                            core::arch::asm!("out dx, eax", in("dx") port1, in("eax") value1);
                            core::arch::asm!("out dx, eax", in("dx") port2, in("eax") value2);
                            core::arch::asm!("out dx, eax", in("dx") port3, in("eax") value2);
                        }
                    }
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
    type_path: Path,
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
        let pretty_type_path: Path = self.pretty_type_path();
        let ident: &Ident = &self.ident;
        quote! {
            pub fn #pretty_read(&self) -> &#pretty_type_path {
                if let Self::Reader(reader) = self {
                    &reader.#ident
                } else {
                    panic!();
                }
            }
        }
    }

    fn pretty_read_ident(&self) -> Ident {
        self.function_ident("read")
    }

    fn pretty_type_path(&self) -> Path {
        let Path {
            leading_colon,
            segments,
        } = &self.type_path;
        let leading_colon: Option<PathSep> = *leading_colon;
        let mut segments: Punctuated<PathSegment, PathSep> = segments.clone();
        if let Some(last_segment) = segments.last_mut() {
            let ident: &mut Ident = &mut last_segment.ident;
            *ident = Ident::new(&format!("{}Pretty", ident), ident.span());
        } else {
            panic!();
        }
        Path {
            leading_colon,
            segments,
        }
    }

    fn reader_declaration(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let pretty_type_path: Path = self.pretty_type_path();
        quote! {
            #ident: #pretty_type_path
        }
    }

    fn true_declaration(&self) -> TokenStream {
        let Self { ident, type_path } = self;
        quote! {
            #ident: core::mem::ManuallyDrop<#type_path>
        }
    }

    fn unprettify(&self, registers: &Registers) -> TokenStream {
        let element_ident: &Ident = &self.ident;
        let registers_ident: &Ident = &registers.ident;
        let writer: Ident = self.writer_ident();
        let writer_type: Ident = registers.writer_ident();
        quote! {
            #writer_type::#writer(writer) => #registers_ident { #element_ident : core::mem::ManuallyDrop::new(writer.unprettify()) }
        }
    }

    fn writer_declaration(&self) -> TokenStream {
        let writer: Ident = self.writer_ident();
        let pretty_type_path: Path = self.pretty_type_path();
        quote! {
            #writer(#pretty_type_path)
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
            ty:
                Type::Path(TypePath {
                    attrs: _,
                    qself: _,
                    path: type_path,
                }),
            default: _,
        } = field
        {
            Self { ident, type_path }
        } else {
            panic!();
        }
    }
}
