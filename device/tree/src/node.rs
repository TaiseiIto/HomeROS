use {
    super::{property::Property, structure::Structure},
    alloc::{collections::vec_deque::VecDeque, string::String, vec::Vec},
    core::iter::once,
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

    fn find_from_path(&self, path: &VecDeque<&str>) -> Option<&Self> {
        let mut path: VecDeque<&str> = path.clone();
        if let Some(name) = path.pop_front() {
            if name == self.name {
                if path.is_empty() {
                    Some(self)
                } else {
                    self.children
                        .iter()
                        .find_map(|child| child.find_from_path(&path))
                }
            } else {
                None
            }
        } else {
            panic!();
        }
    }

    fn find_from_phandle(&self, phandle: u32) -> Option<&Self> {
        self.phandle()
            .and_then(|my_phandle| {
                if my_phandle == phandle {
                    Some(self)
                } else {
                    None
                }
            })
            .or_else(|| {
                self.children
                    .iter()
                    .find_map(|child| child.find_from_phandle(phandle))
            })
    }

    fn first_analyze<T: Iterator<Item = Structure>>(name: String, structures: &mut T) -> Self {
        let mut properties: Vec<Property> = Vec::new();
        let mut children: Vec<Self> = Vec::new();
        while let Some(structure) = structures.next() {
            match structure {
                Structure::BeginNode { name } => {
                    children.push(Self::first_analyze(name, structures));
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

    fn phandle(&self) -> Option<u32> {
        self.properties.iter().find_map(|property| {
            if let Property::PHandle(phandle) = property {
                Some(*phandle)
            } else {
                None
            }
        })
    }

    fn size_cells(&self) -> usize {
        self.properties
            .iter()
            .find_map(|property| {
                if let Property::SizeCells(size_cells) = property {
                    Some(*size_cells as usize)
                } else {
                    None
                }
            })
            .unwrap_or(1)
    }
}

impl FromIterator<Structure> for Node {
    fn from_iter<T: IntoIterator<Item = Structure>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        if let Some(Structure::BeginNode { name }) = iter.next() {
            Self::first_analyze(name, &mut iter)
        } else {
            panic!();
        }
    }
}

pub struct SecondAnalyzer<'a> {
    node: &'a Node,
    path: VecDeque<&'a str>,
    root: &'a Node,
}

impl<'a> SecondAnalyzer<'a> {
    pub fn parent_address_cells(&self) -> usize {
        self.parent().address_cells()
    }

    pub fn parent_size_cells(&self) -> usize {
        self.parent().size_cells()
    }

    pub fn phandle_address_cells(&self, phandle: u32) -> usize {
        self.node_from_phandle(phandle).address_cells()
    }

    pub fn phandle_size_cells(&self, phandle: u32) -> usize {
        self.node_from_phandle(phandle).size_cells()
    }

    fn children(&self) -> Vec<Self> {
        let Self { node, path, root } = self;
        node.children
            .iter()
            .map(|node| {
                let mut path: VecDeque<&str> = path.clone();
                path.push_back(&node.name);
                Self { node, path, root }
            })
            .collect()
    }

    fn node_from_phandle(&'a self, phandle: u32) -> &'a Node {
        self.root.find_from_phandle(phandle).unwrap()
    }

    fn parent(&'a self) -> &'a Node {
        let Self { node, path, root } = self;
        let mut path: VecDeque<&str> = path.clone();
        path.pop_back().unwrap();
        root.find_from_path(&path).unwrap()
    }

    fn root(root: &'a Node) -> Self {
        Self {
            node: root,
            path: once(root.name.as_str()).collect(),
            root,
        }
    }
}

pub trait SecondAnalyzed {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self;
}
