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
                    },
                    2 => {
                        let mut buffer: u16;
                        unsafe {
                            core::arch::asm!("in dx, ax", in("dx") port, out("ax") buffer);
                            core::ptr::read_volatile((&buffer as *const u16) as *const Self)
                        }.prettify()
                    },
                    4 => {
                        let mut buffer: u32;
                        unsafe {
                            core::arch::asm!("in dx, eax", in("dx") port, out("eax") buffer);
                            core::ptr::read_volatile((&buffer as *const u32) as *const Self)
                        }.prettify()
                    },
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
                    },
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
                    },
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
        quote! {
            impl #ident {
                #prettify
                #read_memory
                #read_port
            }
        }
    }

    fn type_ident(&self, suffix: &str) -> Ident {
        let ident: &Ident = &self.ident;
        Ident::new(&format!("{}{}", ident, suffix), ident.span())
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
        let reader_declaration: TokenStream = registers.reader_declaration();
        let true_declaration: TokenStream = registers.true_declaration();
        let true_implement: TokenStream = registers.true_implement();
        let writer_declaration: TokenStream = registers.writer_declaration();
        quote! {
            #pretty_declaration
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
    fn prettify(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        quote! {
            #ident: unsafe {self.#ident.read_memory()}
        }
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
