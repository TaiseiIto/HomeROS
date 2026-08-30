use {alloc::alloc::Layout, core::alloc::GlobalAlloc};

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
    previous: Option<*mut Node>,
    next: Option<*mut Node>,
}

impl Node {
    fn new(node: usize) -> *mut Self {
        let node: *mut Self = node as *mut Self;
        unsafe {
            let node: &mut Self = &mut *node;
            node.previous = None;
            node.next = None;
        }
        node
    }
}
