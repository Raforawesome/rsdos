#![allow(dead_code, unused_variables)]
use core::alloc::Layout;
use core::alloc::GlobalAlloc;


struct Allocator {}
impl Allocator {
	pub const fn new() -> Self { Self {} }
}
unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        todo!()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        todo!()
    }
}
const ALLOCATOR: Allocator = Allocator::new();


pub fn size_of<T>(_: T) -> Layout {
	Layout::new::<T>()
}

pub fn malloc<T>(l: T) -> *mut u8 {
	let layout: Layout = size_of(l);
	unsafe {
		ALLOCATOR.alloc(layout) as *mut u8
	}
}