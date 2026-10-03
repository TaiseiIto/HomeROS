use {
    alloc::alloc::Layout,
    core::{
        cmp::{max, min},
        fmt::{self, Debug, Formatter},
        mem::size_of,
        ops::Range,
        slice::from_raw_parts_mut,
    },
    unit::prefix::KIBI,
};

pub struct Roots {
    region: Range<usize>,
    current_root_start: usize,
}

impl Roots {
    fn next_root_start(&self) -> Option<usize> {
        let current_root_start: usize = self.current_root_start;
        let region_end: usize = self.region.end;
        let xor: usize = current_root_start ^ region_end;
        xor.highest_one().map(|xor_highest_one| {
            match current_root_start
                .lowest_one()
                .filter(|current_root_start_lowest_one| {
                    xor_highest_one > *current_root_start_lowest_one
                }) {
                Some(current_root_start_lowest_one) => {
                    current_root_start + (1 << current_root_start_lowest_one)
                }
                None => region_end,
            }
        })
    }
}

impl From<Range<usize>> for Roots {
    fn from(region: Range<usize>) -> Self {
        let current_root_start: usize = region.start;
        Self {
            region,
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

pub struct Nodes(&'static mut [Node]);

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

    pub fn initialize(region: Range<usize>) {
        if let Ok(Self(nodes)) = region.clone().try_into() {
            let node_list_address: usize = nodes.as_ptr() as usize;
            nodes
                .get_mut(0)
                .unwrap()
                .initialize(region.start..node_list_address);
        };
    }
}

impl Debug for Nodes {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        self.0.get(0).unwrap().fmt(formatter)
    }
}

impl TryFrom<Range<usize>> for Nodes {
    type Error = ();

    fn try_from(region: Range<usize>) -> Result<Self, Self::Error> {
        let Range { start, end } = region;
        let actual_size: usize = end - start;
        let nominal_size: usize = actual_size.next_power_of_two();
        let list_size: usize = min(nominal_size / 2, Self::MAX_SIZE);
        let list_length: usize = list_size / size_of::<Node>();
        let list_start: usize = (end - list_size) & !(list_size - 1);
        (0 < list_length && start < list_start)
            .then_some(Self(unsafe {
                from_raw_parts_mut(list_start as *mut Node, list_length)
            }))
            .ok_or(())
    }
}

#[derive(Debug)]
struct Node {
    state: State,
    address: Range<usize>,
    max_size: usize,
}

impl Node {
    fn initialize(&mut self, region: Range<usize>) {
        self.state = State::Free;
        self.max_size = region.len();
        self.address = region;
    }
}

#[derive(Debug)]
enum State {
    Allocated,
    Divided,
    Free,
}
