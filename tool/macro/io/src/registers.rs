use {
    proc_macro2::{Span, TokenStream},
    quote::quote,
    std::iter,
    syn::{
        Attribute, Field, Fields, FieldsNamed, Ident, ItemStruct, Meta, MetaList, Path,
        PathSegment, Type, TypePath, Visibility,
        punctuated::Punctuated,
        token::{Comma, PathSep},
    },
};

pub struct Structure {
    vis: Visibility,
    ident: Ident,
    elements: Vec<Element>,
    debug_functions: Vec<Ident>,
}

impl Structure {
    fn accessor_debug(&self) -> TokenStream {
        let accessor: Ident = self.accessor_ident();
        let ident: &Ident = &self.ident;
        let ident_string: String = ident.to_string();
        let elements: Vec<TokenStream> = self
            .elements
            .iter()
            .filter_map(Element::accessor_debug)
            .collect();
        quote! {
            impl ::core::fmt::Debug for #accessor {
                fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    formatter
                        .debug_struct(#ident_string)
                        .#(#elements).*
                        .finish()
                }
            }
        }
    }

    fn accessor_declaration(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let vis: &Visibility = &self.vis;
        let accessor: Ident = self.accessor_ident();
        quote! {
            #vis enum #accessor {
                Memory(&'static mut #ident),
                #[cfg(target_arch = "x86_64")]
                Port(u16),
            }
        }
    }

    fn accessor_ident(&self) -> Ident {
        let ident: &Ident = &self.ident;
        Ident::new(&format!("{}Accessor", ident), ident.span())
    }

    fn accessor_implement(&self) -> TokenStream {
        let accessor: Ident = self.accessor_ident();
        let new_address: TokenStream = self.new_address();
        let new_port: TokenStream = self.new_port();
        let reads: Vec<TokenStream> = self.reads();
        let writes: Vec<TokenStream> = self.writes();
        quote! {
            impl #accessor {
                #new_address
                #new_port
                #(#reads)*
                #(#writes)*
            }
        }
    }

    fn debug(&self) -> TokenStream {
        let Self {
            vis: _,
            ident,
            elements,
            debug_functions,
        } = self;
        let ident_string: String = ident.to_string();
        let elements: Vec<TokenStream> = elements.iter().filter_map(Element::debug).collect();
        let functions: Vec<TokenStream> = debug_functions
            .iter()
            .map(|function| {
                let function_string: String = function.to_string();
                quote! {
                    field(#function_string, &self.#function())
                }
            })
            .collect();
        let functions: Option<TokenStream> = (!functions.is_empty()).then_some(quote! {
            .#(#functions).*
        });
        quote! {
            impl ::core::fmt::Debug for #ident {
                fn fmt(&self, formatter: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    formatter
                        .debug_struct(#ident_string)
                        .#(#elements).*
                        #functions
                        .finish()
                }
            }
        }
    }

    fn implement(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let offsets: Vec<TokenStream> = self.offsets();
        let read_memories: Vec<TokenStream> = self.read_memories();
        let read_ports: Vec<TokenStream> = self.read_ports();
        let sizes: Vec<TokenStream> = self.sizes();
        let write_memories: Vec<TokenStream> = self.write_memories();
        let write_ports: Vec<TokenStream> = self.write_ports();
        quote! {
            impl #ident {
                #(#offsets)*
                #(#read_memories)*
                #(#read_ports)*
                #(#sizes)*
                #(#write_memories)*
                #(#write_ports)*
            }
        }
    }

    fn new_address(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        quote! {
            pub unsafe fn new_address(address: usize) -> Self {
                let address: *mut #ident = address as *mut #ident;
                Self::Memory(unsafe {
                    &mut *address
                })
            }
        }
    }

    fn new_port(&self) -> TokenStream {
        quote! {
            #[cfg(target_arch = "x86_64")]
            pub unsafe fn new_port(port: u16) -> Self {
                Self::Port(port)
            }
        }
    }

    fn offset_asserts(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .map(|element| element.offset_assert(self))
            .collect()
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

    fn read_memories(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(Element::read_memory)
            .collect()
    }

    fn read_ports(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(Element::read_port)
            .collect()
    }

    fn reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.read(self))
            .collect()
    }

    fn sizes(&self) -> Vec<TokenStream> {
        self.elements.iter().map(Element::size).collect()
    }

    fn true_declaration(&self) -> TokenStream {
        let Self {
            vis,
            ident,
            elements,
            debug_functions: _,
        } = self;
        let elements: Vec<TokenStream> = elements.iter().map(Element::true_declaration).collect();
        quote! {
            #[repr(C)]
            #vis struct #ident {
                #(#elements),*
            }
        }
    }

    fn write_memories(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(Element::write_memory)
            .collect()
    }

    fn write_ports(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(Element::write_port)
            .collect()
    }

    fn writes(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.write(self))
            .collect()
    }
}

impl From<ItemStruct> for Structure {
    fn from(item_struct: ItemStruct) -> Self {
        if let ItemStruct {
            attrs,
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
            let debug_functions: Vec<Ident> = attrs.iter().fold(
                Vec::default(),
                |mut debug_functions,
                 Attribute {
                     pound_token: _,
                     style: _,
                     bracket_token: _,
                     meta,
                 }| {
                    if let Meta::List(
                        meta_list @ MetaList {
                            path,
                            delimiter: _,
                            tokens: _,
                        },
                    ) = meta
                        && path.is_ident("debug_functions")
                    {
                        let new_debug_functions: Punctuated<Ident, Comma> = meta_list
                            .parse_args_with(Punctuated::parse_terminated)
                            .unwrap();
                        debug_functions.append(&mut new_debug_functions.iter().cloned().collect());
                    }
                    debug_functions
                },
            );
            Self {
                vis,
                ident,
                elements,
                debug_functions,
            }
        } else {
            panic!();
        }
    }
}

impl From<Structure> for TokenStream {
    fn from(structure: Structure) -> Self {
        let accessor_debug: TokenStream = structure.accessor_debug();
        let accessor_declaration: TokenStream = structure.accessor_declaration();
        let accessor_implement: TokenStream = structure.accessor_implement();
        let debug: TokenStream = structure.debug();
        let implement: TokenStream = structure.implement();
        let offset_asserts: Vec<TokenStream> = structure.offset_asserts();
        let true_declaration: TokenStream = structure.true_declaration();
        quote! {
            #true_declaration
            #(#offset_asserts)*
            #implement
            #debug
            #accessor_declaration
            #accessor_implement
            #accessor_debug
        }
    }
}

struct Element {
    ident: Option<Ident>,
    index: usize,
    is_register: bool,
    span: Span,
    ty: Type,
}

impl Element {
    fn accessor_debug(&self) -> Option<TokenStream> {
        self.ident
            .as_ref()
            .zip(self.read_ident())
            .map(|(ident, read)| {
                let ident_string: String = ident.to_string();
                quote! {
                    field(#ident_string, &unsafe {
                        self.#read()
                    })
                }
            })
    }

    fn const_ident(&self, suffix: &str) -> Ident {
        Ident::new(
            &format!("{}_{}", self.ident().to_string().to_uppercase(), suffix),
            self.span,
        )
    }

    fn debug(&self) -> Option<TokenStream> {
        self.ident
            .as_ref()
            .zip(self.read_memory_ident())
            .map(|(ident, read_memory)| {
                let ident_string: String = ident.to_string();
                if self.is_register {
                    quote! {
                        field(#ident_string, &unsafe {
                            self.#read_memory()
                        })
                    }
                } else {
                    quote! {
                        field(#ident_string, &self.#ident)
                    }
                }
            })
    }

    fn function_ident(&self, prefix: &str, suffix: Option<&str>) -> Option<Ident> {
        self.ident.as_ref().map(|ident| {
            let prefix: String = prefix.to_string();
            let stem: String = ident.to_string();
            let suffix: Option<String> = suffix.map(String::from);
            let words: Vec<String> = [Some(prefix), Some(stem), suffix]
                .into_iter()
                .flatten()
                .collect();
            Ident::new(&words.join("_"), ident.span())
        })
    }

    fn ident(&self) -> Ident {
        let Self {
            ident,
            index,
            is_register: _,
            span,
            ty: _,
        } = self;
        ident
            .clone()
            .unwrap_or(Ident::new(&format!("reserved{}", index), *span))
    }

    fn new(index: usize, field: Field) -> Self {
        if let Field {
            attrs,
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
            let is_register: bool = attrs.iter().fold(
                true,
                |is_register,
                 Attribute {
                     pound_token: _,
                     style: _,
                     bracket_token: _,
                     meta,
                 }| match meta {
                    Meta::Path(path) => match path.get_ident().unwrap().to_string().as_str() {
                        "unregister" => false,
                        _ => is_register,
                    },
                    _ => is_register,
                },
            );
            Self {
                ident,
                index,
                is_register,
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
            const #offset: usize = #value;
        }
    }

    fn offset_assert(&self, structure: &Structure) -> TokenStream {
        let structure: &Ident = &structure.ident;
        let element: Ident = self.ident();
        let offset: Ident = self.offset_ident();
        quote! {
            const _: () = assert!(::core::mem::offset_of!(#structure, #element) == #structure::#offset);
        }
    }

    fn offset_ident(&self) -> Ident {
        self.const_ident("OFFSET")
    }

    fn pretty_type(&self) -> Option<Type> {
        self.ident.as_ref().map(|_| self.ty.clone())
    }

    fn read(&self, structure: &Structure) -> Option<TokenStream> {
        let read: Ident = self.read_ident()?;
        let pretty_type: Type = self.pretty_type()?;
        let read_memory: Ident = self.read_memory_ident()?;
        let read_port: Ident = self.read_port_ident()?;
        let structure: &Ident = &structure.ident;
        Some(quote! {
            pub unsafe fn #read(&self) -> #pretty_type {
                match self {
                    Self::Memory(memory) => unsafe {
                        memory.#read_memory()
                    },
                    #[cfg(target_arch = "x86_64")]
                    Self::Port(port) => unsafe {
                        #structure::#read_port(*port)
                    },
                }
            }
        })
    }

    fn read_ident(&self) -> Option<Ident> {
        self.function_ident("read", None)
    }

    fn read_memory(&self) -> Option<TokenStream> {
        let ident: &Ident = self.ident.as_ref()?;
        let read_memory: Ident = self.read_memory_ident()?;
        let pretty_type: Type = self.pretty_type()?;
        Some(if self.is_register {
            quote! {
                pub unsafe fn #read_memory(&self) -> #pretty_type {
                    unsafe {
                        self.#ident.read_memory()
                    }
                }
            }
        } else {
            quote! {
                pub fn #read_memory(&self) -> #pretty_type {
                    self.#ident
                }
            }
        })
    }

    fn read_memory_ident(&self) -> Option<Ident> {
        self.function_ident("read", Some("memory"))
    }

    fn read_port(&self) -> Option<TokenStream> {
        self.read_port_ident()
            .zip(self.pretty_type())
            .map(|(read_port, pretty_type)| {
                let offset: Ident = self.offset_ident();
                let ty: Type = self.true_type();
                if self.is_register {
                    quote! {
                        #[cfg(target_arch = "x86_64")]
                        pub unsafe fn #read_port(port: u16) -> #pretty_type {
                            let port: u16 = port + Self::#offset as u16;
                            unsafe {
                                #ty::read_port(port)
                            }
                        }
                    }
                } else {
                    let read: TokenStream = match ty {
                        Type::Path(TypePath {
                            attrs: _,
                            qself: _,
                            ref path,
                        }) => match path.get_ident().unwrap().to_string().as_str() {
                            "u8" => quote! {
                                unsafe {
                                    ::core::arch::asm!("in al, dx", out("al") value, in("dx") port);
                                }
                            },
                            "u16" => quote! {
                                unsafe {
                                    ::core::arch::asm!("in ax, dx", out("ax") value, in("dx") port);
                                }
                            },
                            "u32" => quote! {
                                unsafe {
                                    ::core::arch::asm!("in eax, dx", out("eax") value, in("dx") port);
                                }
                            },
                            "u64" | "u128" => quote! {
                                for current_port in (port..)
                                    .step_by(::core::mem::size_of::<u32>())
                                    .take_while(|current_port| (*current_port as usize) < (port as usize) + ::core::mem::size_of::<#ty>()) {
                                    let mut buffer: u32;
                                    unsafe {
                                        ::core::arch::asm!("in eax, dx", out("eax") buffer, in("dx") current_port);
                                    }
                                    value += (buffer as #ty) << (((current_port - port) as u32) * u8::BITS);
                                }
                            },
                            _ => panic!(),
                        },
                        _ => panic!(),
                    };
                    quote! {
                        #[cfg(target_arch = "x86_64")]
                        pub unsafe fn #read_port(port: u16) -> #pretty_type {
                            let port: u16 = port + Self::#offset as u16;
                            let mut value: #ty = 0;
                            #read
                            value
                        }
                    }
                }
            })
    }

    fn read_port_ident(&self) -> Option<Ident> {
        self.function_ident("read", Some("port"))
    }

    fn size(&self) -> TokenStream {
        let size: Ident = self.size_ident();
        let true_type: Type = self.true_type();
        quote! {
            const #size: usize = ::core::mem::size_of::<#true_type>();
        }
    }

    fn size_ident(&self) -> Ident {
        self.const_ident("SIZE")
    }

    fn true_declaration(&self) -> TokenStream {
        let ident: Ident = self.ident();
        let true_type: Type = self.true_type();
        quote! {
            #ident: #true_type
        }
    }

    fn true_type(&self) -> Type {
        let Self {
            ident,
            index: _,
            is_register,
            span: _,
            ty,
        } = self;
        match (ident, is_register, ty) {
            (
                Some(_),
                true,
                Type::Path(TypePath {
                    attrs,
                    qself,
                    path:
                        Path {
                            leading_colon,
                            segments,
                        },
                }),
            ) => {
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
            }
            _ => ty.clone(),
        }
    }

    fn write(&self, structure: &Structure) -> Option<TokenStream> {
        self.is_register.then_some(())?;
        let write: Ident = self.write_ident()?;
        let pretty_type: Type = self.pretty_type()?;
        let write_memory: Ident = self.write_memory_ident()?;
        let write_port: Ident = self.write_port_ident()?;
        let structure: &Ident = &structure.ident;
        Some(quote! {
            pub unsafe fn #write(&mut self, value: #pretty_type) {
                match self {
                    Self::Memory(memory) => unsafe {
                        memory.#write_memory(value);
                    },
                    #[cfg(target_arch = "x86_64")]
                    Self::Port(port) => unsafe {
                        #structure::#write_port(*port, value);
                    },
                }
            }
        })
    }

    fn write_ident(&self) -> Option<Ident> {
        self.function_ident("write", None)
    }

    fn write_memory(&self) -> Option<TokenStream> {
        self.is_register.then_some(())?;
        let ident: &Ident = self.ident.as_ref()?;
        let write_memory: Ident = self.write_memory_ident()?;
        let pretty_type: Type = self.pretty_type()?;
        Some(quote! {
            pub unsafe fn #write_memory(&mut self, value: #pretty_type) {
                unsafe {
                    self.#ident.write_memory(value);
                }
            }
        })
    }

    fn write_memory_ident(&self) -> Option<Ident> {
        self.function_ident("write", Some("memory"))
    }

    fn write_port(&self) -> Option<TokenStream> {
        self.is_register.then_some(())?;
        self.write_port_ident()
            .zip(self.pretty_type())
            .map(|(write_port, pretty_type)| {
                let offset: Ident = self.offset_ident();
                let ty: Type = self.true_type();
                quote! {
                    #[cfg(target_arch = "x86_64")]
                    pub unsafe fn #write_port(port: u16, value: #pretty_type) {
                        let port: u16 = port + Self::#offset as u16;
                        unsafe {
                            #ty::write_port(port, value);
                        }
                    }
                }
            })
    }

    fn write_port_ident(&self) -> Option<Ident> {
        self.function_ident("write", Some("port"))
    }
}
