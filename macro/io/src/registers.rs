use {
    proc_macro2::{Span, TokenStream},
    quote::quote,
    std::iter,
    syn::{
        Field, Fields, FieldsNamed, Ident, ItemStruct, Path, PathSegment, Type, TypePath,
        Visibility, punctuated::Punctuated, token::PathSep,
    },
};

pub struct Structure {
    vis: Visibility,
    ident: Ident,
    elements: Vec<Element>,
}

impl Structure {
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
        let reads: Vec<TokenStream> = self.reads();
        let writes: Vec<TokenStream> = self.writes();
        quote! {
            impl #accessor {
                #(#reads)*
                #(#writes)*
            }
        }
    }

    fn debug(&self) -> TokenStream {
        let ident: &Ident = &self.ident;
        let ident_string: String = ident.to_string();
        let elements: Vec<TokenStream> = self
            .elements
            .iter()
            .filter_map(|element| element.debug())
            .collect();
        quote! {
            impl core::fmt::Debug for #ident {
                fn fmt(&self, formatter: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    formatter
                        .debug_struct(#ident_string)
                        .#(#elements).*
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
            .filter_map(|element| element.read_memory())
            .collect()
    }

    fn read_ports(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.read_port())
            .collect()
    }

    fn reads(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.read(self))
            .collect()
    }

    fn sizes(&self) -> Vec<TokenStream> {
        self.elements.iter().map(|element| element.size()).collect()
    }

    fn true_declaration(&self) -> TokenStream {
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
            #[repr(C)]
            #vis struct #ident {
                #(#elements),*
            }
        }
    }

    fn write_memories(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.write_memory())
            .collect()
    }

    fn write_ports(&self) -> Vec<TokenStream> {
        self.elements
            .iter()
            .filter_map(|element| element.write_port())
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

    fn debug(&self) -> Option<TokenStream> {
        self.ident
            .as_ref()
            .zip(self.read_memory_ident())
            .map(|(ident, read_memory)| {
                let ident_string: String = ident.to_string();
                quote! {
                    field(#ident_string, &unsafe {
                        self.#read_memory()
                    })
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
            const #offset: usize = #value;
        }
    }

    fn offset_assert(&self, structure: &Structure) -> TokenStream {
        let structure: &Ident = &structure.ident;
        let element: Ident = self.ident();
        let offset: Ident = self.offset_ident();
        quote! {
            const _: () = assert!(core::mem::offset_of!(#structure, #element) == #structure::#offset);
        }
    }

    fn offset_ident(&self) -> Ident {
        self.const_ident("OFFSET")
    }

    fn pretty_type(&self) -> Option<Path> {
        if let Type::Path(TypePath {
            attrs: _,
            qself: _,
            path: Path {
                leading_colon,
                segments,
            },
        }) = &self.ty
        {
            let leading_colon: Option<PathSep> = *leading_colon;
            let mut segments: Punctuated<PathSegment, PathSep> = segments.clone();
            if let Some(last_segment) = segments.last_mut() {
                let ident: &mut Ident = &mut last_segment.ident;
                *ident = Ident::new(&format!("{}Pretty", ident), ident.span());
            } else {
                panic!();
            }
            Some(Path {
                leading_colon,
                segments,
            })
        } else {
            None
        }
    }

    fn read(&self, structure: &Structure) -> Option<TokenStream> {
        let read: Option<Ident> = self.read_ident();
        let pretty_type: Option<Path> = self.pretty_type();
        let read_memory: Option<Ident> = self.read_memory_ident();
        let read_port: Option<Ident> = self.read_port_ident();
        if let (Some(read), Some(pretty_type), Some(read_memory), Some(read_port)) =
            (read, pretty_type, read_memory, read_port)
        {
            Some((read, pretty_type, read_memory, read_port))
        } else {
            None
        }
        .map(|(read, pretty_type, read_memory, read_port)| {
            let structure: &Ident = &structure.ident;
            quote! {
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
            }
        })
    }

    fn read_ident(&self) -> Option<Ident> {
        self.function_ident("read", None)
    }

    fn read_memory(&self) -> Option<TokenStream> {
        if let (Some(ident), Some(read_memory), Some(pretty_type)) = (
            self.ident.as_ref(),
            self.read_memory_ident(),
            self.pretty_type(),
        ) {
            Some((ident, read_memory, pretty_type))
        } else {
            None
        }
        .map(|(ident, read_memory, pretty_type)| {
            quote! {
                pub unsafe fn #read_memory(&self) -> #pretty_type {
                    unsafe {
                        self.#ident.read_memory()
                    }
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
                let ty: &Type = &self.ty;
                quote! {
                    #[cfg(target_arch = "x86_64")]
                    pub unsafe fn #read_port(port: u16) -> #pretty_type {
                        let port: u16 = port + Self::#offset as u16;
                        unsafe {
                            #ty::read_port(port)
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
        let ty: &Type = &self.ty;
        quote! {
            const #size: usize = core::mem::size_of::<#ty>();
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

    fn write(&self, structure: &Structure) -> Option<TokenStream> {
        let write: Option<Ident> = self.write_ident();
        let pretty_type: Option<Path> = self.pretty_type();
        let write_memory: Option<Ident> = self.write_memory_ident();
        let write_port: Option<Ident> = self.write_port_ident();
        if let (Some(write), Some(pretty_type), Some(write_memory), Some(write_port)) =
            (write, pretty_type, write_memory, write_port)
        {
            Some((write, pretty_type, write_memory, write_port))
        } else {
            None
        }
        .map(|(write, pretty_type, write_memory, write_port)| {
            let structure: &Ident = &structure.ident;
            quote! {
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
            }
        })
    }

    fn write_ident(&self) -> Option<Ident> {
        self.function_ident("write", None)
    }

    fn write_memory(&self) -> Option<TokenStream> {
        if let (Some(ident), Some(write_memory), Some(pretty_type)) = (
            self.ident.as_ref(),
            self.write_memory_ident(),
            self.pretty_type(),
        ) {
            Some((ident, write_memory, pretty_type))
        } else {
            None
        }
        .map(|(ident, write_memory, pretty_type)| {
            quote! {
                pub unsafe fn #write_memory(&mut self, value: #pretty_type) {
                    unsafe {
                        self.#ident.write_memory(value);
                    }
                }
            }
        })
    }

    fn write_memory_ident(&self) -> Option<Ident> {
        self.function_ident("write", Some("memory"))
    }

    fn write_port(&self) -> Option<TokenStream> {
        self.write_port_ident()
            .zip(self.pretty_type())
            .map(|(write_port, pretty_type)| {
                let offset: Ident = self.offset_ident();
                let ty: &Type = &self.ty;
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
