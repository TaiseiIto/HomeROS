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
        unimplemented!();
    }

    unsafe fn dealloc(&self, address: *mut u8, layout: Layout) {
        unimplemented!();
    }
}

struct Node {
    allocated: bool,
    previous: Option<*mut Node>,
    next: Option<*mut Node>,
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

    fn alloc_by_myself(&self, layout: Layout) -> Option<*mut u8> {
        (!self.allocated)
            .then(|| {
                self.available_range().and_then(
                    |Range {
                         start: available_head,
                         end: available_tail,
                     }| {
                        let head: usize =
                            (available_head + layout.align() - 1) & !(layout.align() - 1);
                        let tail: usize = head + layout.size();
                        (tail <= available_tail).then(|| head as *mut u8)
                    },
                )
            })
            .flatten()
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
