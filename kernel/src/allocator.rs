use linked_list_allocator::LockedHeap;

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

pub fn init() {
    const HEAP_SIZE: usize = 0x100000;

    let alloc = crate::mem::PHYS.alloc(HEAP_SIZE, vmem::AllocStrategy::NextFit).unwrap();
    unsafe {
        ALLOCATOR.lock().init(alloc, HEAP_SIZE);
    }
}