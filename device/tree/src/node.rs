use {
    crate::{
        property::{Property, status::Status},
        structure::Structure,
    },
    alloc::{
        collections::vec_deque::VecDeque,
        string::{String, ToString},
        vec,
        vec::Vec,
    },
    core::iter::once,
    memory::Regions,
};

#[derive(Debug)]
pub struct Node {
    name: Name,
    properties: Vec<Property>,
    children: Vec<Self>,
}

impl Node {
    pub fn find_from_name(&self, name: &str) -> Vec<&Self> {
        let mut nodes: Vec<&Self> = self
            .children
            .iter()
            .flat_map(|child| child.find_from_name(name).into_iter())
            .collect();
        if self.is_ok() && self.name.name.as_str() == name {
            nodes.push(self);
        }
        nodes
    }

    pub fn memories(&self) -> Vec<&Self> {
        if let Some("memory") = self.device_type() {
            vec![self]
        } else {
            self.children
                .iter()
                .filter(|child| child.is_ok())
                .flat_map(|child| child.memories().into_iter())
                .collect()
        }
    }

    pub fn regions(&self) -> Regions<u128> {
        self.properties
            .iter()
            .find_map(|property| {
                if let Property::Reg(reg) = property {
                    Some(reg.into())
                } else {
                    None
                }
            })
            .unwrap_or(Regions::<u128>::default())
            + self
                .children
                .iter()
                .map(|child| child.regions())
                .sum::<Regions<u128>>()
    }

    pub fn reserved_memories(&self) -> Vec<&Self> {
        if let "reserved-memory" = self.name.name.as_str() {
            vec![self]
        } else {
            self.children
                .iter()
                .flat_map(|child| child.reserved_memories().into_iter())
                .collect()
        }
    }

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

    fn device_type(&self) -> Option<&str> {
        self.properties.iter().find_map(|property| {
            if let Property::DeviceType(device_type) = property {
                Some(device_type.as_str())
            } else {
                None
            }
        })
    }

    fn find_from_path(&self, path: &VecDeque<&Name>) -> Option<&Self> {
        let mut path: VecDeque<&Name> = path.clone();
        if let Some(name) = path.pop_front() {
            if *name == self.name {
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

    fn first_analyze<T: Iterator<Item = Structure>>(name: Name, structures: &mut T) -> Self {
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

    fn interrupt_cells(&self) -> Option<usize> {
        self.properties.iter().find_map(|property| {
            if let Property::InterruptCells(interrupt_cells) = property {
                Some(*interrupt_cells as usize)
            } else {
                None
            }
        })
    }

    fn is_ok(&self) -> bool {
        match self.status() {
            Some(Status::Okay) | None => true,
            _ => false,
        }
    }

    fn names(&self, specifier: &str) -> Option<Vec<String>> {
        self.properties.iter().find_map(|property| {
            if let Property::Names {
                specifier: property_specifier,
                names,
            } = property
            {
                (property_specifier.as_str() == specifier).then_some(names.clone())
            } else {
                None
            }
        })
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

    fn specifier_cells(&self, specifier: &str) -> Option<usize> {
        self.properties.iter().find_map(|property| {
            if let Property::Cells {
                specifier: property_specifier,
                cells,
            } = property
                && property_specifier == specifier
            {
                Some(*cells as usize)
            } else {
                None
            }
        })
    }

    fn status(&self) -> Option<&Status> {
        self.properties.iter().find_map(|property| match property {
            Property::Status(status) => Some(status),
            _ => None,
        })
    }
}

impl FromIterator<Structure> for Node {
    fn from_iter<T: IntoIterator<Item = Structure>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        if let Some(Structure::BeginNode { name }) = iter.next() {
            let root: Self = Self::first_analyze(name, &mut iter);
            SecondAnalyzer::root(&root).second_analyze(&root)
        } else {
            panic!();
        }
    }
}

impl SecondAnalyzed for Node {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self {
        let Self {
            name,
            properties,
            children: _,
        } = self;
        let properties: Vec<Property> = properties
            .iter()
            .map(|property| second_analyzer.second_analyze(property))
            .collect();
        let children: Vec<Self> = second_analyzer
            .children()
            .into_iter()
            .map(|second_analyzer| second_analyzer.second_analyze(second_analyzer.node))
            .collect();
        Self {
            name: name.clone(),
            properties,
            children,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Name {
    name: String,
    unit_address: Option<u128>,
}

impl From<&str> for Name {
    fn from(name: &str) -> Self {
        let name_and_unit_address: Result<[&str; 2], _> =
            name.split('@').collect::<Vec<&str>>().try_into();
        match name_and_unit_address {
            Ok([name, unit_address]) => Self {
                name: name.to_string(),
                unit_address: Some(u128::from_str_radix(unit_address, 16).unwrap()),
            },
            Err(_) => Self {
                name: name.to_string(),
                unit_address: None,
            },
        }
    }
}

pub struct SecondAnalyzer<'a> {
    node: &'a Node,
    path: VecDeque<&'a Name>,
    root: &'a Node,
}

impl<'a> SecondAnalyzer<'a> {
    pub fn address_cells(&self) -> usize {
        self.node.address_cells()
    }

    pub fn interrupt_cells(&self) -> Option<usize> {
        self.node.interrupt_cells()
    }

    pub fn interrupt_parent_interrupt_cells(&self) -> Option<usize> {
        self.interrupt_parent()
            .and_then(|node| node.interrupt_cells())
    }

    pub fn parent_address_cells(&self) -> Option<usize> {
        self.parent_node().map(|node| node.address_cells())
    }

    pub fn parent_size_cells(&self) -> Option<usize> {
        self.parent_node().map(|node| node.size_cells())
    }

    pub fn phandle_address_cells(&self, phandle: u32) -> Option<usize> {
        self.node_from_phandle(phandle)
            .map(|node| node.address_cells())
    }

    pub fn phandle_interrupt_cells(&self, phandle: u32) -> Option<usize> {
        self.node_from_phandle(phandle)
            .and_then(|node| node.interrupt_cells())
    }

    pub fn phandle_names(&self, phandle: u32, specifier: &str) -> Option<Vec<String>> {
        self.node_from_phandle(phandle)
            .and_then(|node| node.names(specifier))
    }

    pub fn phandle_specifier_cells(&self, phandle: u32, specifier: &str) -> Option<usize> {
        self.node_from_phandle(phandle)
            .and_then(|node| node.specifier_cells(specifier))
    }

    pub fn second_analyze<T: SecondAnalyzed>(&self, analyzed: &T) -> T {
        analyzed.second_analyze(self)
    }

    pub fn second_analyze_with_specifier<T: SecondAnalyzedWithSpecifier>(
        &self,
        analyzed: &T,
        specifier: &str,
    ) -> T {
        analyzed.second_analyze_with_specifier(self, specifier)
    }

    pub fn size_cells(&self) -> usize {
        self.node.size_cells()
    }

    pub fn specifier_cells(&self, specifier: &str) -> Option<usize> {
        self.node.specifier_cells(specifier)
    }

    fn children(&self) -> Vec<Self> {
        let Self { node, path, root } = self;
        node.children
            .iter()
            .map(|node| {
                let mut path: VecDeque<&Name> = path.clone();
                path.push_back(&node.name);
                Self { node, path, root }
            })
            .collect()
    }

    fn interrupt_parent(&'a self) -> Option<&'a Node> {
        self.interrupt_parent_phandle()
            .and_then(|phandle| self.node_from_phandle(phandle))
    }

    fn interrupt_parent_phandle(&'a self) -> Option<u32> {
        self.node
            .properties
            .iter()
            .find_map(|property| {
                if let Property::InterruptParent(phandle) = property {
                    Some(*phandle)
                } else {
                    None
                }
            })
            .or_else(|| {
                self.parent()
                    .and_then(|parent| parent.interrupt_parent_phandle())
            })
    }

    fn node_from_phandle(&'a self, phandle: u32) -> Option<&'a Node> {
        self.root.find_from_phandle(phandle)
    }

    fn parent(&'a self) -> Option<Self> {
        let Self {
            node: _,
            path,
            root,
        } = self;
        let mut path: VecDeque<&Name> = path.clone();
        path.pop_back()
            .and_then(|_| root.find_from_path(&path))
            .map(|node| Self { node, path, root })
    }

    fn parent_node(&'a self) -> Option<&'a Node> {
        let Self {
            node: _,
            path,
            root,
        } = self;
        let mut path: VecDeque<&Name> = path.clone();
        path.pop_back().and_then(|_| root.find_from_path(&path))
    }

    fn root(root: &'a Node) -> Self {
        Self {
            node: root,
            path: once(&root.name).collect(),
            root,
        }
    }
}

pub trait SecondAnalyzed {
    fn second_analyze(&self, second_analyzer: &SecondAnalyzer<'_>) -> Self;
}

pub trait SecondAnalyzedWithSpecifier {
    fn second_analyze_with_specifier(
        &self,
        second_analyzer: &SecondAnalyzer<'_>,
        specifier: &str,
    ) -> Self;
}
