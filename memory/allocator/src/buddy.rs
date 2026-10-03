use {
    alloc::alloc::Layout,
    core::{
        cmp::{max, min},
        fmt::{self, Debug, Formatter},
        mem::size_of,
        ops::Range,
        ptr::slice_from_raw_parts_mut,
    },
    unit::prefix::KIBI,
};

pub struct Roots {
    address: Range<usize>,
    current_root_start: usize,
}

impl Roots {
    fn next_root_start(&self) -> Option<usize> {
        let current_root_start: usize = self.current_root_start;
        let address_end: usize = self.address.end;
        let xor: usize = current_root_start ^ address_end;
        xor.highest_one().map(|xor_highest_one| {
            match current_root_start
                .lowest_one()
                .filter(|current_root_start_lowest_one| {
                    xor_highest_one > *current_root_start_lowest_one
                }) {
                Some(current_root_start_lowest_one) => {
                    current_root_start + (1 << current_root_start_lowest_one)
                }
                None => address_end,
            }
        })
    }
}

impl From<Range<usize>> for Roots {
    fn from(address: Range<usize>) -> Self {
        let current_root_start: usize = address.start;
        Self {
            address,
            current_root_start,
        }
    }
}

impl Iterator for Roots {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_root_start().map(|next_root_start| {
            let current_root_start: usize = self.current_root_start;
            self.current_root_start = next_root_start;
            current_root_start..next_root_start
        })
    }
}

#[derive(Clone)]
pub struct Nodes(*mut [Node]);

impl Nodes {
    const MAX_SIZE: usize = (4 * KIBI) as usize;
    const MAX_LENGTH: usize = Self::MAX_SIZE / size_of::<Node>();
    const MIN_LENGTH: usize = 1;
    const MIN_SIZE: usize = Self::MIN_LENGTH * size_of::<Node>();

    pub fn alloc(&mut self, layout: Layout) -> Option<*mut u8> {
        unimplemented!();
    }

    pub fn dealloc(&mut self, address: usize) {
        unimplemented!();
    }

    pub fn initialize(address: Range<usize>) {
        if let Ok(nodes @ Self(..)) = address.clone().try_into() {
            let node_list_address: usize = nodes.as_mut_ptr() as usize;
            nodes
                .root()
                .get_mut()
                .initialize(address.start..node_list_address);
        };
    }

    fn as_mut_ptr(&self) -> *mut Node {
        self.0.as_mut_ptr()
    }

    fn root(&self) -> NodeInNodes {
        NodeInNodes {
            nodes: self.clone(),
            index: 0,
        }
    }
}

impl Debug for Nodes {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.root().get_mut().fmt(formatter)
    }
}

impl TryFrom<Range<usize>> for Nodes {
    type Error = ();

    fn try_from(address: Range<usize>) -> Result<Self, Self::Error> {
        let Range { start, end } = address;
        let actual_size: usize = end - start;
        let nominal_size: usize = actual_size.next_power_of_two();
        let list_size: usize = min(nominal_size / 2, Self::MAX_SIZE);
        let list_length: usize = list_size / size_of::<Node>();
        let list_start: usize = (end - list_size) & !(list_size - 1);
        (0 < list_length && start < list_start)
            .then_some(Self(unsafe {
                slice_from_raw_parts_mut(list_start as *mut Node, list_length)
            }))
            .ok_or(())
    }
}

struct NodeInNodes {
    nodes: Nodes,
    index: usize,
}

impl NodeInNodes {
    fn get_mut(&mut self) -> &mut Node {
        let Self {
            nodes: Nodes(nodes),
            index,
        } = self;
        unsafe { &mut *nodes.get_unchecked_mut(*index) }
    }

    fn higher_half_mut(&mut self) -> Option<NodeInNodes> {
        let Self { nodes, index } = self;
        let index: usize = 2 * *index + 2;
        (index < nodes.0.len()).then_some(nodes.root()).or({
            self.get_mut()
                .higher_half_range()
                .try_into()
                .ok()
                .map(|nodes: Nodes| nodes.root())
        })
    }

    fn lower_half_mut(&mut self) -> Option<NodeInNodes> {
        let Self { nodes, index } = self;
        let index: usize = 2 * *index + 1;
        (index < nodes.0.len()).then_some(nodes.root()).or({
            self.get_mut()
                .lower_half_range()
                .try_into()
                .ok()
                .map(|nodes: Nodes| nodes.root())
        })
    }

    fn satisfies(&mut self, layout: Layout) -> bool {
        self.get_mut().satisfies(layout)
    }
}

#[derive(Debug)]
struct Node {
    state: State,
    address: Range<usize>,
    max_size: usize,
}

impl Node {
    fn division_point(&self) -> usize {
        let address: &Range<usize> = &self.address;
        address.start + address.len().next_power_of_two() / 2
    }

    fn higher_half_range(&self) -> Range<usize> {
        self.division_point()..self.address.end
    }

    fn initialize(&mut self, address: Range<usize>) {
        self.state = State::Free;
        self.max_size = address.len();
        self.address = address;
    }

    fn lower_half_range(&self) -> Range<usize> {
        self.address.start..self.division_point()
    }

    fn satisfies(&self, layout: Layout) -> bool {
        match self.state {
            State::Allocated => false,
            State::Divided => unimplemented!(),
            State::Free => {
                layout.size() < self.address.len()
                    && self.address.start.is_multiple_of(layout.align())
            }
        }
    }
}

#[derive(Debug)]
enum State {
    Allocated,
    Divided,
    Free,
}
