use {
    super::{property::Property, structure::Structure},
    alloc::{string::String, vec::Vec},
};

#[derive(Debug)]
pub struct Node {
    name: String,
    properties: Vec<Property>,
    children: Vec<Self>,
}

impl Node {
    fn address_cells(&self) -> usize {
        self.properties
            .iter()
            .find_map(|property| {
                if let Property::AddressCells(address_cells) = property {
                    Some(*address_cells as usize)
                } else {
                    None
                }
            })
            .unwrap_or(2)
    }

    fn read<T: Iterator<Item = Structure>>(name: String, structures: &mut T) -> Self {
        let mut properties: Vec<Property> = Vec::new();
        let mut children: Vec<Self> = Vec::new();
        while let Some(structure) = structures.next() {
            match structure {
                Structure::BeginNode { name } => {
                    children.push(Self::read(name, structures));
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

impl FromIterator<Structure> for Node {
    fn from_iter<T: IntoIterator<Item = Structure>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        if let Some(Structure::BeginNode { name }) = iter.next() {
            Self::read(name, &mut iter)
        } else {
            panic!();
        }
    }
}
