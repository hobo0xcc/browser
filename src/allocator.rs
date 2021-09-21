use crate::*;
use alloc::alloc::Layout;
use linked_list_allocator::LockedHeap;

const HEAP_SIZE: usize = 0x100000;
static mut HEAP: &[u8] = &[0; HEAP_SIZE];

#[global_allocator]
static GLOBAL: LockedHeap = LockedHeap::empty();

pub unsafe fn init() {
    let heap_start = HEAP.as_ptr() as usize;
    GLOBAL.lock().init(heap_start, HEAP_SIZE);
}

#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {
    loop {}
}
