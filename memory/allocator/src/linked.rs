use {
    alloc::alloc::Layout,
    core::{
        alloc::GlobalAlloc,
        mem::{align_of, size_of},
        ops::Range,
    },
};

pub struct List {
    head: *mut Node,
}

impl List {
    pub fn new(head: usize) -> Self {
        Self {
            head: Node::new(head),
        }
    }
}

unsafe impl GlobalAlloc for List {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        unsafe { &mut *self.head }.alloc(layout)
    }

    unsafe fn dealloc(&self, address: *mut u8, layout: Layout) {
        unsafe { &mut *self.head }.dealloc(address, layout)
    }
}

struct Node {
    allocated: bool,
    previous: Option<*mut Self>,
    next: Option<*mut Self>,
}

impl Node {
    fn address(&self) -> usize {
        self as *const Self as usize
    }

    fn available_head(&self) -> usize {
        self.address() + size_of::<Self>()
    }

    fn available_range(&self) -> Option<Range<usize>> {
        self.available_tail()
            .map(|available_tail| self.available_head()..available_tail)
    }

    fn available_tail(&self) -> Option<usize> {
        self.next().map(|next| next.address())
    }

    fn alloc(&mut self, layout: Layout) -> *mut u8 {
        self.alloc_myself(layout)
            .or_else(|| self.next_mut().map(|next| next.alloc(layout)))
            .unwrap_or_else(|| self.alloc_by_extending(layout))
    }

    fn alloc_by_extending(&mut self, layout: Layout) -> *mut u8 {
        assert!(self.next().is_none());
        self.allocated = true;
        let allocated_head: usize =
            (self.available_head() + layout.align() - 1) & !(layout.align() - 1);
        let allocated_tail: usize = allocated_head + layout.size();
        let next_node_head: usize =
            (allocated_tail + align_of::<Self>() - 1) & !(align_of::<Self>() - 1);
        self.connect(unsafe { &mut *Self::new(next_node_head) });
        allocated_head as *mut u8
    }

    fn alloc_myself(&mut self, layout: Layout) -> Option<*mut u8> {
        (!self.allocated)
            .then(|| {
                self.available_range().and_then(
                    |Range {
                         start: available_head,
                         end: available_tail,
                     }| {
                        let allocated_head: usize =
                            (available_head + layout.align() - 1) & !(layout.align() - 1);
                        let allocated_tail: usize = allocated_head + layout.size();
                        (allocated_tail <= available_tail).then(|| {
                            self.divide(allocated_tail);
                            self.allocated = true;
                            allocated_head as *mut u8
                        })
                    },
                )
            })
            .flatten()
    }

    fn connect(&mut self, next: &mut Self) {
        self.next = Some(next as *mut Self);
        next.previous = Some(self as *mut Self);
    }

    fn dealloc(&mut self, address: *mut u8, layout: Layout) {
        if self
            .available_range()
            .is_some_and(|available_range| available_range.contains(&(address as usize)))
        {
            self.dealloc_myself(address, layout);
        } else {
            self.next_mut().unwrap().dealloc(address, layout);
        }
    }

    fn dealloc_myself(&mut self, address: *mut u8, layout: Layout) {
        let align: usize = layout.align();
        let size: usize = layout.size();
        let deallocated_head: usize = address as usize;
        let deallocated_tail: usize = deallocated_head + size;
        let Range {
            start: available_head,
            end: available_tail,
        }: Range<usize> = self.available_range().unwrap();
        assert_eq!(deallocated_head & (align - 1), 0);
        assert!(available_head <= deallocated_head);
        assert!(deallocated_tail <= available_tail);
        self.allocated = false;
        self.merge();
    }

    fn disconnect_next(&mut self) -> Option<*mut Self> {
        self.next.take().inspect(|&next| {
            unsafe { &mut *next }.previous = None;
        })
    }

    fn disconnect_previous(&mut self) -> Option<*mut Self> {
        self.previous.take().inspect(|&previous| {
            unsafe { &mut *previous }.next = None;
        })
    }

    fn divide(&mut self, divide_point: usize) {
        let new_node_head: usize =
            (divide_point + align_of::<Self>() - 1) & !(align_of::<Self>() - 1);
        let new_node_tail: usize = new_node_head + size_of::<Self>();
        if self.available_range().is_some_and(
            |Range {
                 start: available_head,
                 end: available_tail,
             }| available_head < new_node_head && new_node_tail < available_tail,
        ) {
            let new_node: &mut Self = unsafe { &mut *Self::new(new_node_head) };
            if let Some(next_node) = self.next_mut() {
                new_node.connect(next_node);
            }
            self.connect(new_node);
        }
    }

    fn merge(&mut self) {
        if !self.allocated {
            if self.previous().is_some_and(|previous| !previous.allocated) {
                self.previous_mut().unwrap().merge();
            } else {
                self.merge_next();
            }
        }
    }

    fn merge_next(&mut self) {
        if !self.allocated && self.next().is_some_and(|next| !next.allocated) {
            if let Some(next) = self.disconnect_next()
                && let Some(next_next) = unsafe { &mut *next }.next_mut()
            {
                self.connect(next_next);
            }
            self.merge_next();
        }
    }

    fn new(node: usize) -> *mut Self {
        let node: usize = (node + align_of::<Self>() - 1) & !(align_of::<Self>() - 1);
        let node: *mut Self = node as *mut Self;
        unsafe {
            let node: &mut Self = &mut *node;
            node.allocated = false;
            node.previous = None;
            node.next = None;
        }
        node
    }

    fn next(&self) -> Option<&Self> {
        self.next.map(|next| unsafe { &*next })
    }

    fn next_mut(&mut self) -> Option<&mut Self> {
        self.next.map(|next| unsafe { &mut *next })
    }

    fn previous(&self) -> Option<&Self> {
        self.previous.map(|previous| unsafe { &*previous })
    }

    fn previous_mut(&mut self) -> Option<&mut Self> {
        self.previous.map(|previous| unsafe { &mut *previous })
    }
}
