use {
    super::{
        property::Property,
        structure::{Structure, StructureIterator},
    },
    alloc::{string::String, vec::Vec},
};

#[derive(Debug)]
struct Node {
    name: String,
    properties: Vec<Property>,
    children: Vec<Self>,
}

impl Node {
    fn new(name: String, structures: &mut StructureIterator<'_>) -> Self {
        let mut properties: Vec<Property> = Vec::new();
        let mut children: Vec<Self> = Vec::new();
        while let Some(structure) = structures.next() {
            match structure {
                Structure::BeginNode { name } => {
                    children.push(Self::new(name, structures));
                }
                Structure::End => panic!(),
                Structure::EndNode => {
                    break;
                }
                Structure::Nop => {}
                Structure::Property(property) => {
                    properties.push(property);
                }
            }
        }
        Self {
            name,
            properties,
            children,
        }
    }
}
