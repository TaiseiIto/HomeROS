use {
    alloc::alloc::Layout,
    core::{
        cmp::{max, min},
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
            let shift: u32 = current_root_start
                .lowest_one()
                .filter(|current_root_start_lowest_one| {
                    xor_highest_one > *current_root_start_lowest_one
                })
                .unwrap_or(xor_highest_one);
            current_root_start + (1 << shift)
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

pub struct NodeList(&'static mut [Node]);

impl NodeList {
    const MAX_SIZE: usize = (4 * KIBI) as usize;
    const MAX_LENGTH: usize = Self::MAX_SIZE / size_of::<Node>();
    const MIN_LENGTH: usize = 1;
    const MIN_SIZE: usize = Self::MIN_LENGTH * size_of::<Node>();

    pub fn alloc(&mut self, layout: Layout) -> Option<*mut u8> {
        self.0
            .get_mut(0)
            .unwrap()
            .alloc(max(layout.align(), layout.size().next_power_of_two()))
    }

    pub fn dealloc(&mut self, address: usize) {
        self.0.get_mut(0).unwrap().dealloc(address);
    }

    pub fn initialize(region: &Range<usize>) {
        if let Ok(Self(nodes)) = region.try_into() {
            let node_list_address: usize = nodes.as_ptr() as usize;
            nodes
                .get_mut(0)
                .unwrap()
                .initialize(region, node_list_address);
        };
    }
}

impl TryFrom<&Range<usize>> for NodeList {
    type Error = ();

    fn try_from(region: &Range<usize>) -> Result<Self, Self::Error> {
        let Range { start, end } = region;
        assert_eq!((start ^ end).count_ones(), 1);
        let size: usize = end - start;
        (Self::MIN_SIZE < size)
            .then(|| {
                let size: usize = min(size / 2, Self::MAX_SIZE);
                let length: usize = size / size_of::<Node>();
                let start: usize = end - length;
                Self(unsafe { from_raw_parts_mut(start as *mut Node, length) })
            })
            .ok_or(())
    }
}

struct Node {
    index: u8,
    state: State,
    start: usize,
    log_size: u8,
    unavailable_tail_size: usize,
    max_size: usize,
}

impl Node {
    fn alloc(&mut self, size: usize) -> Option<*mut u8> {
        unimplemented!();
    }

    fn dealloc(&mut self, address: usize) {
        unimplemented!();
    }

    fn initialize(&mut self, region: &Range<usize>, node_list_address: usize) {
        self.state = State::Free;
        self.start = region.start;
        self.log_size = region.len().ilog2() as u8;
        self.unavailable_tail_size = region.end - node_list_address;
        self.max_size = node_list_address - region.start;
    }
}

enum State {
    Allocated,
    Divided,
    Free,
    Invalid,
}
