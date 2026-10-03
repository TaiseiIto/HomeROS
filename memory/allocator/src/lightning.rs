use {
    core::{
        cmp::{max, min},
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
            current_root_start
                .lowest_one()
                .filter(|current_root_start_lowest_one| {
                    xor_highest_one > *current_root_start_lowest_one
                })
                .map(|current_root_start_lowest_one| {
                    current_root_start + (1 << current_root_start_lowest_one)
                })
                .unwrap_or(address_end)
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

    pub fn alloc(&mut self, size: usize) -> Option<*mut u8> {
        self.root().alloc(size)
    }

    pub fn dealloc(&mut self, address: usize) {
        self.root().dealloc(address)
    }

    pub fn initialize(address: Range<usize>) {
        if let Ok(nodes @ Self(..)) = address.clone().try_into() {
            nodes.root().initialize(address);
        }
    }

    fn len(&self) -> usize {
        self.0.len()
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
            .then(|| {
                Self(slice_from_raw_parts_mut(
                    list_start as *mut Node,
                    list_length,
                ))
            })
            .ok_or(())
    }
}

struct NodeInNodes {
    nodes: Nodes,
    index: usize,
}

impl NodeInNodes {
    fn address(&mut self) -> Range<usize> {
        self.get_mut().address.clone()
    }

    fn alloc(&mut self, size: usize) -> Option<*mut u8> {
        match self.state().clone() {
            State::Allocated => None,
            State::Divided => self
                .higher_half_mut()
                .and_then(|mut higher_half| higher_half.alloc(size))
                .or_else(|| {
                    self.lower_half_mut()
                        .and_then(|mut lower_half| lower_half.alloc(size))
                })
                .inspect(|_| {
                    *self.max_size() = max(
                        self.higher_half_mut()
                            .map(|mut higher_half| *higher_half.max_size())
                            .unwrap_or(0),
                        self.lower_half_mut()
                            .map(|mut lower_half| *lower_half.max_size())
                            .unwrap_or(0),
                    )
                }),
            State::Free => (size <= *self.max_size()).then(|| {
                self.divides();
                self.alloc(size).unwrap_or_else(|| {
                    self.merges();
                    let provided: *mut u8 = self.provides();
                    *self.state() = State::Allocated;
                    *self.max_size() = 0;
                    provided
                })
            }),
        }
    }

    fn can_divide(&mut self) -> bool {
        self.is_free()
    }

    fn can_merge(&mut self) -> bool {
        matches!(self.state(), State::Divided)
            && self
                .higher_half_mut()
                .map(|mut higher_half| higher_half.state().clone())
                .is_none_or(|state| matches!(state, State::Free))
            && self
                .lower_half_mut()
                .map(|mut lower_half| lower_half.state().clone())
                .is_none_or(|state| matches!(state, State::Free))
    }

    fn can_provide(&mut self) -> bool {
        self.is_free()
    }

    fn dealloc(&mut self, address: usize) {
        if self.address().contains(&address) {
            match self.state().clone() {
                State::Allocated => {
                    *self.state() = State::Free;
                    *self.max_size() = self.address().len();
                }
                State::Divided => {
                    if let Some(mut higher_half) = self.higher_half_mut() {
                        higher_half.dealloc(address);
                    } else if let Some(mut lower_half) = self.lower_half_mut() {
                        lower_half.dealloc(address);
                    } else {
                        panic!();
                    }
                    if self.can_merge() {
                        self.merges();
                    } else {
                        *self.max_size() = max(
                            self.higher_half_mut()
                                .map(|mut higher_half| *higher_half.max_size())
                                .unwrap_or(0),
                            self.lower_half_mut()
                                .map(|mut lower_half| *lower_half.max_size())
                                .unwrap_or(0),
                        )
                    }
                }
                State::Free => panic!(),
            }
        }
    }

    fn divides(&mut self) {
        assert!(self.can_divide());
        *self.state() = State::Divided;
        let higher_half_range: Range<usize> = self.get_mut().higher_half_range();
        let lower_half_range: Range<usize> = self.get_mut().lower_half_range();
        *self.max_size() = max(
            self.higher_half_mut()
                .map(|mut higher_half| {
                    higher_half.initialize(higher_half_range);
                    *higher_half.max_size()
                })
                .unwrap_or(0),
            self.lower_half_mut()
                .map(|mut lower_half| {
                    lower_half.initialize(lower_half_range);
                    *lower_half.max_size()
                })
                .unwrap_or(0),
        );
    }

    fn get_mut(&mut self) -> &mut Node {
        let Self {
            nodes: Nodes(nodes),
            index,
        } = self;
        unsafe { &mut *nodes.get_unchecked_mut(*index) }
    }

    fn higher_half_mut(&mut self) -> Option<Self> {
        let Self { nodes, index } = self;
        let index: usize = 2 * *index + 2;
        (index < nodes.len())
            .then(|| Self {
                nodes: nodes.clone(),
                index,
            })
            .or_else(|| {
                self.get_mut()
                    .higher_half_range()
                    .try_into()
                    .ok()
                    .map(|nodes: Nodes| nodes.root())
            })
    }

    fn initialize(&mut self, address: Range<usize>) {
        let nodes_address: usize = self.nodes.as_mut_ptr() as usize;
        let address: Range<usize> = if address.contains(&nodes_address) {
            address.start..nodes_address
        } else {
            address
        };
        self.get_mut().initialize(address);
    }

    fn is_free(&mut self) -> bool {
        matches!(self.state(), State::Free)
    }

    fn lower_half_mut(&mut self) -> Option<Self> {
        let Self { nodes, index } = self;
        let index: usize = 2 * *index + 1;
        (index < nodes.len())
            .then(|| Self {
                nodes: nodes.clone(),
                index,
            })
            .or_else(|| {
                self.get_mut()
                    .lower_half_range()
                    .try_into()
                    .ok()
                    .map(|nodes: Nodes| nodes.root())
            })
    }

    fn max_size(&mut self) -> &mut usize {
        &mut self.get_mut().max_size
    }

    fn merges(&mut self) {
        assert!(self.can_merge());
        *self.state() = State::Free;
        *self.max_size() = self.address().len();
    }

    fn provides(&mut self) -> *mut u8 {
        assert!(self.can_provide());
        self.get_mut().provides()
    }

    fn state(&mut self) -> &mut State {
        &mut self.get_mut().state
    }
}

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

    fn provides(&mut self) -> *mut u8 {
        let address: usize = self.address.start;
        address as *mut u8
    }
}

#[derive(Clone)]
enum State {
    Allocated,
    Divided,
    Free,
}
