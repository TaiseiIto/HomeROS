use {
    proc_macro2::{Span, TokenStream},
    quote::quote,
    std::{
        cmp::Ordering::{Equal, Greater, Less},
        iter,
        ops::Range,
    },
    syn::{
        Attribute, Expr, ExprAssign, ExprLit, ExprPath, Field, Fields, FieldsNamed, Ident,
        ItemStruct, Lit, Meta, MetaList, Path, PathSegment, Type, TypePath, Visibility,
        punctuated::Punctuated,
        token::{Comma, PathSep},
    },
};

pub struct Structure {
    vis: Visibility,
    ident: Ident,
    element_repetitions: Vec<ElementRepetition>,
    debug_functions: Vec<Ident>,
}

impl Structure {
    fn accessor_debug(&self) -> TokenStream {
        let Self {
            vis: _,
            ident,
            element_repetitions,
            debug_functions: _,
        } = self;
        let accessor: Ident = self.accessor_ident();
        let ident_string: String = ident.to_string();
        let elements: Vec<TokenStream> = element_repetitions
            .iter()
            .flat_map(ElementRepetition::accessor_debugs)
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
        let Self {
            vis,
            ident,
            element_repetitions: _,
            debug_functions: _,
        } = self;
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
        Ident::new(&format!("{}Accessor", ident), Span::call_site())
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
            element_repetitions,
            debug_functions,
        } = self;
        let ident_string: String = ident.to_string();
        let elements: Vec<TokenStream> = element_repetitions
            .iter()
            .flat_map(ElementRepetition::debugs)
            .collect();
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

    fn declaration(&self) -> TokenStream {
        let Self {
            vis,
            ident,
            element_repetitions: _,
            debug_functions: _,
        } = self;
        let output_elements: Vec<TokenStream> = self
            .output_elements()
            .iter()
            .map(OutputElement::declaration)
            .collect();
        quote! {
            #[repr(C)]
            #vis struct #ident {
                #(#output_elements),*
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
        self.output_elements()
            .iter()
            .map(|output_element| output_element.offset_assert(self))
            .collect()
    }

    fn offsets(&self) -> Vec<TokenStream> {
        self.output_elements()
            .iter()
            .zip(
                self.output_elements()
                    .iter()
                    .rev()
                    .skip(1)
                    .map(Some)
                    .chain(iter::once(None))
                    .rev(),
            )
            .map(|(output_element, previous_output_element)| {
                output_element.offset(previous_output_element)
            })
            .collect()
    }

    fn output_elements(&self) -> Vec<OutputElement> {
        self.into()
    }

    fn read_memories(&self) -> Vec<TokenStream> {
        self.element_repetitions
            .iter()
            .flat_map(ElementRepetition::read_memories)
            .collect()
    }

    fn read_ports(&self) -> Vec<TokenStream> {
        self.element_repetitions
            .iter()
            .flat_map(ElementRepetition::read_ports)
            .collect()
    }

    fn reads(&self) -> Vec<TokenStream> {
        self.element_repetitions
            .iter()
            .flat_map(|element_repetition| element_repetition.reads(self))
            .collect()
    }

    fn sizes(&self) -> Vec<TokenStream> {
        self.output_elements()
            .iter()
            .map(OutputElement::size)
            .collect()
    }

    fn write_memories(&self) -> Vec<TokenStream> {
        self.element_repetitions
            .iter()
            .flat_map(ElementRepetition::write_memories)
            .collect()
    }

    fn write_ports(&self) -> Vec<TokenStream> {
        self.element_repetitions
            .iter()
            .flat_map(ElementRepetition::write_ports)
            .collect()
    }

    fn writes(&self) -> Vec<TokenStream> {
        self.element_repetitions
            .iter()
            .flat_map(|element_repetition| element_repetition.writes(self))
            .collect()
    }
}

impl From<ItemStruct> for Structure {
    fn from(
        ItemStruct {
            attrs,
            vis,
            struct_token: _,
            ident,
            generics: _,
            fields,
            semi_token: _,
        }: ItemStruct,
    ) -> Self {
        if let Fields::Named(FieldsNamed {
            brace_token: _,
            named,
        }) = fields
        {
            let (input_elements, repetitions): (Vec<InputElement>, Vec<Repetition>) =
                named.into_iter().enumerate().map(Into::into).fold(
                    (Vec::default(), Vec::default()),
                    |(mut input_elements, mut repetitions),
                     InputElementAndRepetition {
                         input_element,
                         repetition,
                     }| {
                        input_elements.push(input_element);
                        if let Some(repetition) = repetition {
                            repetitions.push(repetition);
                        }
                        (input_elements, repetitions)
                    },
                );
            let element_repetitions: Vec<ElementRepetition> = input_elements
                .into_iter()
                .enumerate()
                .fold(
                    (Vec::default(), Vec::default()),
                    |(mut element_repetitions, mut input_elements): (
                        Vec<ElementRepetition>,
                        Vec<InputElement>,
                    ),
                     (index, input_element)| {
                        if let Some(Repetition { fields, times }) = repetitions
                            .iter()
                            .find(|Repetition { fields, times: _ }| fields.contains(&index))
                        {
                            input_elements.push(input_element);
                            match input_elements.len().cmp(&fields.len()) {
                                Less => (element_repetitions, input_elements),
                                Equal => {
                                    element_repetitions.push(ElementRepetition {
                                        input_elements,
                                        times: Some(*times),
                                    });
                                    (element_repetitions, Vec::default())
                                }
                                Greater => panic!(),
                            }
                        } else {
                            element_repetitions.push(ElementRepetition {
                                input_elements: vec![input_element],
                                times: None,
                            });
                            (element_repetitions, Vec::default())
                        }
                    },
                )
                .0;
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
                element_repetitions,
                debug_functions,
            }
        } else {
            panic!();
        }
    }
}

impl From<Structure> for TokenStream {
    fn from(structure: Structure) -> Self {
        let declaration: TokenStream = structure.declaration();
        let offset_asserts: Vec<TokenStream> = structure.offset_asserts();
        let implement: TokenStream = structure.implement();
        let debug: TokenStream = structure.debug();
        let accessor_debug: TokenStream = structure.accessor_debug();
        let accessor_declaration: TokenStream = structure.accessor_declaration();
        let accessor_implement: TokenStream = structure.accessor_implement();
        quote! {
            #declaration
            #(#offset_asserts)*
            #implement
            #debug
            #accessor_declaration
            #accessor_implement
            #accessor_debug
        }
    }
}

struct ElementRepetition {
    input_elements: Vec<InputElement>,
    times: Option<usize>,
}

impl ElementRepetition {
    fn accessor_debugs(&self) -> Vec<TokenStream> {
        let Self {
            input_elements,
            times,
        } = self;
        input_elements
            .iter()
            .filter_map(|input_element| input_element.accessor_debug(*times))
            .collect()
    }

    fn debugs(&self) -> Vec<TokenStream> {
        let Self {
            input_elements,
            times,
        } = self;
        input_elements
            .iter()
            .filter_map(|input_element| input_element.debug(*times))
            .collect()
    }

    fn output_elements(&self) -> Vec<OutputElement> {
        self.into()
    }

    fn read_memories(&self) -> Vec<TokenStream> {
        let output_elements: Vec<OutputElement> = self.output_elements();
        let read_registers: Vec<TokenStream> = output_elements
            .iter()
            .filter_map(OutputElement::read_memory)
            .collect();
        let read_arrays: Vec<TokenStream> = self
            .times
            .map(|_| {
                self.input_elements
                    .iter()
                    .filter_map(|input_element| {
                        input_element.read_memory(
                            output_elements
                                .iter()
                                .filter(|output_element| {
                                    input_element.ident == output_element.ident
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        [read_registers, read_arrays].concat()
    }

    fn read_ports(&self) -> Vec<TokenStream> {
        let output_elements: Vec<OutputElement> = self.output_elements();
        let read_registers: Vec<TokenStream> = output_elements
            .iter()
            .filter_map(OutputElement::read_port)
            .collect();
        let read_arrays: Vec<TokenStream> = self
            .times
            .map(|_| {
                self.input_elements
                    .iter()
                    .filter_map(|input_element| {
                        input_element.read_port(
                            output_elements
                                .iter()
                                .filter(|output_element| {
                                    input_element.ident == output_element.ident
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        [read_registers, read_arrays].concat()
    }

    fn reads(&self, structure: &Structure) -> Vec<TokenStream> {
        let output_elements: Vec<OutputElement> = self.output_elements();
        let read_registers: Vec<TokenStream> = output_elements
            .iter()
            .filter_map(|output_element| output_element.read(structure))
            .collect();
        let read_arrays: Vec<TokenStream> = self
            .times
            .map(|_| {
                self.input_elements
                    .iter()
                    .filter_map(|input_element| input_element.read(structure))
                    .collect()
            })
            .unwrap_or_default();
        [read_registers, read_arrays].concat()
    }

    fn write_memories(&self) -> Vec<TokenStream> {
        let output_elements: Vec<OutputElement> = self.output_elements();
        let write_registers: Vec<TokenStream> = output_elements
            .iter()
            .filter_map(OutputElement::write_memory)
            .collect();
        let write_arrays: Vec<TokenStream> = self
            .times
            .map(|_| {
                self.input_elements
                    .iter()
                    .filter_map(|input_element| {
                        input_element.write_memory(
                            output_elements
                                .iter()
                                .filter(|output_element| {
                                    input_element.ident == output_element.ident
                                })
                                .collect(),
                        )
                    })
                    .collect()
            })
            .unwrap_or_default();
        [write_registers, write_arrays].concat()
    }

    /// # TODO
    /// * Implement index access on repeated registers.
    fn write_ports(&self) -> Vec<TokenStream> {
        self.output_elements()
            .iter()
            .filter_map(OutputElement::write_port)
            .collect()
    }

    /// # TODO
    /// * Implement index access on repeated registers.
    fn writes(&self, structure: &Structure) -> Vec<TokenStream> {
        self.output_elements()
            .iter()
            .filter_map(|output_element| output_element.write(structure))
            .collect()
    }
}

struct InputElement {
    ident: Option<Ident>,
    ty: Type,
    index: usize,
}

impl InputElement {
    fn accessor_debug(&self, times: Option<usize>) -> Option<TokenStream> {
        let ident_string: String = self.ident.as_ref()?.to_string();
        let read_ident: Ident = self.read_ident()?;
        let read: TokenStream = if let Some(times) = times {
            let reads: Vec<TokenStream> = (0..times)
                .map(|index| {
                    quote! {
                        self.#read_ident(#index)
                    }
                })
                .collect();
            quote! {
                [#(#reads),*]
            }
        } else {
            quote! {
                self.#read_ident()
            }
        };
        Some(quote! {
            field(#ident_string, &unsafe {
                #read
            })
        })
    }

    fn debug(&self, times: Option<usize>) -> Option<TokenStream> {
        let ident_string: String = self.ident.as_ref()?.to_string();
        let read_ident: Ident = self.read_memory_ident()?;
        let read: TokenStream = if let Some(times) = times {
            let reads: Vec<TokenStream> = (0..times)
                .map(|index| {
                    quote! {
                        self.#read_ident(#index)
                    }
                })
                .collect();
            quote! {
                [#(#reads),*]
            }
        } else {
            quote! {
                self.#read_ident()
            }
        };
        Some(quote! {
            field(#ident_string, &unsafe {
                #read
            })
        })
    }

    fn function_ident(&self, prefix: &str, suffix: Option<&str>) -> Option<Ident> {
        let prefix: String = prefix.to_string();
        let stem: String = self.ident.as_ref()?.to_string();
        let suffix: Option<String> = suffix.map(String::from);
        let words: Vec<String> = [Some(prefix), Some(stem), suffix]
            .into_iter()
            .flatten()
            .collect();
        Some(Ident::new(&words.join("_"), Span::call_site()))
    }

    fn pretty_type(&self) -> Option<Type> {
        self.ident.as_ref()?;
        Some(self.ty.clone())
    }

    fn read(&self, structure: &Structure) -> Option<TokenStream> {
        let structure: &Ident = &structure.ident;
        let pretty_type: Type = self.pretty_type()?;
        let read: Ident = self.read_ident()?;
        let read_memory: Ident = self.read_memory_ident()?;
        let read_port: Ident = self.read_port_ident()?;
        Some(quote! {
            pub unsafe fn #read(&self, index: usize) -> #pretty_type {
                match self {
                    Self::Memory(memory) => unsafe {
                        memory.#read_memory(index)
                    },
                    #[cfg(target_arch = "x86_64")]
                    Self::Port(port) => unsafe {
                        #structure::#read_port(*port, index)
                    },
                }
            }
        })
    }

    fn read_ident(&self) -> Option<Ident> {
        self.function_ident("read", None)
    }

    fn read_memory(&self, output_elements: Vec<&OutputElement>) -> Option<TokenStream> {
        let read_memory: Ident = self.read_memory_ident()?;
        let pretty_type: Type = self.pretty_type()?;
        let read_memories: Vec<TokenStream> = output_elements
            .into_iter()
            .map(|output_element| {
                let read_memory: Ident = output_element.read_memory_ident().unwrap();
                let index: usize = output_element.repeat_index.unwrap();
                quote! {
                    #index => unsafe { self.#read_memory() }
                }
            })
            .collect();
        Some(quote! {
            pub unsafe fn #read_memory(&self, index: usize) -> #pretty_type {
                match index {
                    #(#read_memories),*
                    _ => panic!(),
                }
            }
        })
    }

    fn read_memory_ident(&self) -> Option<Ident> {
        self.function_ident("read", Some("memory"))
    }

    fn read_port(&self, output_elements: Vec<&OutputElement>) -> Option<TokenStream> {
        let read_port: Ident = self.read_port_ident()?;
        let pretty_type: Type = self.pretty_type()?;
        let read_ports: Vec<TokenStream> = output_elements
            .into_iter()
            .map(|output_element| {
                let read_port: Ident = output_element.read_port_ident().unwrap();
                let index: usize = output_element.repeat_index.unwrap();
                quote! {
                    #index => unsafe { Self::#read_port(port) }
                }
            })
            .collect();
        Some(quote! {
            pub unsafe fn #read_port(port: u16, index: usize) -> #pretty_type {
                match index {
                    #(#read_ports),*
                    _ => panic!(),
                }
            }
        })
    }

    fn read_port_ident(&self) -> Option<Ident> {
        self.function_ident("read", Some("port"))
    }

    fn write_memory(&self, output_elements: Vec<&OutputElement>) -> Option<TokenStream> {
        let write_memory: Ident = self.write_memory_ident()?;
        let pretty_type: Type = self.pretty_type()?;
        let write_memories: Vec<TokenStream> = output_elements
            .into_iter()
            .map(|output_element| {
                let write_memory: Ident = output_element.write_memory_ident().unwrap();
                let index: usize = output_element.repeat_index.unwrap();
                quote! {
                    #index => unsafe { self.#write_memory(value) }
                }
            })
            .collect();
        Some(quote! {
            pub unsafe fn #write_memory(&mut self, index: usize, value: #pretty_type) {
                match index {
                    #(#write_memories),*
                    _ => panic!(),
                }
            }
        })
    }

    fn write_memory_ident(&self) -> Option<Ident> {
        self.function_ident("write", Some("memory"))
    }
}

impl From<(Option<Ident>, Type, usize)> for InputElement {
    fn from((ident, ty, index): (Option<Ident>, Type, usize)) -> Self {
        Self {
            ident: ident.filter(|ident| ident != "__"),
            ty,
            index,
        }
    }
}

struct InputElementAndRepetition {
    input_element: InputElement,
    repetition: Option<Repetition>,
}

impl From<(usize, Field)> for InputElementAndRepetition {
    fn from(
        (
            index,
            Field {
                attrs,
                vis: _,
                modifiers: _,
                ident,
                colon_token: _,
                ty,
                default: _,
            },
        ): (usize, Field),
    ) -> Self {
        Self {
            input_element: (ident, ty, index).into(),
            repetition: attrs
                .into_iter()
                .find_map(|attr| (index, attr).try_into().ok()),
        }
    }
}

struct OutputElement {
    ident: Option<Ident>,
    ty: Type,
    input_index: usize,
    repeat_index: Option<usize>,
}

impl OutputElement {
    fn const_ident(&self, suffix: &str) -> Ident {
        Ident::new(
            &format!("{}_{}", self.ident().to_string().to_uppercase(), suffix),
            Span::call_site(),
        )
    }

    fn declaration(&self) -> TokenStream {
        let ident: Ident = self.ident();
        let ty: Type = self.ty();
        quote! {
            #ident: #ty
        }
    }

    fn function_ident(&self, prefix: &str, suffix: Option<&str>) -> Option<Ident> {
        self.ident.as_ref()?;
        let prefix: String = prefix.to_string();
        let stem: String = self.ident().to_string();
        let suffix: Option<String> = suffix.map(String::from);
        let words: Vec<String> = [Some(prefix), Some(stem), suffix]
            .into_iter()
            .flatten()
            .collect();
        Some(Ident::new(&words.join("_"), Span::call_site()))
    }

    fn ident(&self) -> Ident {
        let Self {
            ident,
            ty: _,
            input_index,
            repeat_index,
        } = self;
        let input_index: Option<String> = ident.is_none().then_some(input_index.to_string());
        let ident: String = ident
            .as_ref()
            .map(|ident| ident.to_string())
            .unwrap_or("reserved".to_string());
        let repeat_index: Option<String> =
            repeat_index.map(|repeat_index| repeat_index.to_string());
        let words: Vec<String> = [Some(ident), repeat_index, input_index]
            .into_iter()
            .flatten()
            .collect();
        Ident::new(&words.join("_"), Span::call_site())
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
        let output_element: Ident = self.ident();
        let offset: Ident = self.offset_ident();
        quote! {
            const _: () = assert!(::core::mem::offset_of!(#structure, #output_element) == #structure::#offset);
        }
    }

    fn offset_ident(&self) -> Ident {
        self.const_ident("OFFSET")
    }

    fn pretty_type(&self) -> Option<Type> {
        self.ident.as_ref()?;
        Some(self.ty.clone())
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
        let ident: Ident = self.ident();
        let read_memory: Ident = self.read_memory_ident()?;
        let pretty_type: Type = self.pretty_type()?;
        Some(quote! {
            pub unsafe fn #read_memory(&self) -> #pretty_type {
                unsafe {
                    self.#ident.read_memory()
                }
            }
        })
    }

    fn read_memory_ident(&self) -> Option<Ident> {
        self.function_ident("read", Some("memory"))
    }

    fn read_port(&self) -> Option<TokenStream> {
        let read_port: Ident = self.read_port_ident()?;
        let pretty_type: Type = self.pretty_type()?;
        let offset: Ident = self.offset_ident();
        let ty: Type = self.ty();
        Some(quote! {
            #[cfg(target_arch = "x86_64")]
            pub unsafe fn #read_port(port: u16) -> #pretty_type {
                let port: u16 = port + Self::#offset as u16;
                unsafe {
                    #ty::read_port(port)
                }
            }
        })
    }

    fn read_port_ident(&self) -> Option<Ident> {
        self.function_ident("read", Some("port"))
    }

    fn size(&self) -> TokenStream {
        let size: Ident = self.size_ident();
        let ty: Type = self.ty();
        quote! {
            const #size: usize = ::core::mem::size_of::<#ty>();
        }
    }

    fn size_ident(&self) -> Ident {
        self.const_ident("SIZE")
    }

    fn ty(&self) -> Type {
        let Self {
            ident,
            ty,
            input_index: _,
            repeat_index: _,
        } = self;
        match (ident, ty) {
            (
                Some(_),
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
                    *ident = Ident::new(&format!("{}Raw", ident), Span::call_site());
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
        let ident: Ident = self.ident();
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
        self.write_port_ident()
            .zip(self.pretty_type())
            .map(|(write_port, pretty_type)| {
                let offset: Ident = self.offset_ident();
                let ty: Type = self.ty();
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

impl From<&ElementRepetition> for Vec<OutputElement> {
    fn from(
        ElementRepetition {
            input_elements,
            times,
        }: &ElementRepetition,
    ) -> Self {
        times
            .map(|times| {
                (0..times)
                    .flat_map(|repeat_index| {
                        input_elements
                            .iter()
                            .map(move |InputElement { ident, ty, index }| OutputElement {
                                ident: ident.clone(),
                                ty: ty.clone(),
                                input_index: *index,
                                repeat_index: Some(repeat_index),
                            })
                    })
                    .collect()
            })
            .unwrap_or_else(|| {
                input_elements
                    .iter()
                    .map(|InputElement { ident, ty, index }| OutputElement {
                        ident: ident.clone(),
                        ty: ty.clone(),
                        input_index: *index,
                        repeat_index: None,
                    })
                    .collect()
            })
    }
}

impl From<&Structure> for Vec<OutputElement> {
    fn from(
        Structure {
            vis: _,
            ident: _,
            element_repetitions,
            debug_functions: _,
        }: &Structure,
    ) -> Self {
        element_repetitions
            .iter()
            .flat_map(ElementRepetition::output_elements)
            .collect()
    }
}

struct Repetition {
    fields: Range<usize>,
    times: usize,
}

impl TryFrom<(usize, Attribute)> for Repetition {
    type Error = ();

    fn try_from(
        (
            offset,
            Attribute {
                pound_token: _,
                style: _,
                bracket_token: _,
                meta,
            },
        ): (usize, Attribute),
    ) -> Result<Self, Self::Error> {
        if let Meta::List(
            ref meta_list @ MetaList {
                ref path,
                delimiter: _,
                tokens: _,
            },
        ) = meta
            && path.is_ident("repeat")
        {
            if let (Some(fields), Some(times)) = meta_list
                .parse_args_with(Punctuated::<ExprAssign, Comma>::parse_terminated)
                .unwrap()
                .into_iter()
                .fold(
                    (None, None),
                    |(fields, times),
                     ExprAssign {
                         attrs: _,
                         left,
                         eq_token: _,
                         right,
                     }| {
                        if let (
                            Expr::Path(ExprPath {
                                attrs: _,
                                qself: _,
                                path,
                            }),
                            Expr::Lit(ExprLit {
                                attrs: _,
                                lit: Lit::Int(value),
                            }),
                        ) = (Box::into_inner(left), Box::into_inner(right))
                        {
                            let value: usize = value.base10_parse().unwrap();
                            match path.get_ident().unwrap().to_string().as_str() {
                                "fields" => (Some(value), times),
                                "times" => (fields, Some(value)),
                                _ => panic!(),
                            }
                        } else {
                            panic!();
                        }
                    },
                )
            {
                Ok(Self {
                    fields: offset..offset + fields,
                    times,
                })
            } else {
                panic!();
            }
        } else {
            Err(())
        }
    }
}
