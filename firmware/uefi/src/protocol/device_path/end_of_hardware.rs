use super::{Node, Type};

#[derive(Debug)]
#[repr(C)]
pub struct Protocol {
    ty: Type,
}

impl Node for Protocol {}
