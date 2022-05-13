#![allow(dead_code, unused_variables)]
use core::alloc::Layout;
use core::alloc::GlobalAlloc;
use core::ptr::null_mut;


pub struct Allocator;

impl Allocator {
	pub const fn new() -> Self { Self {} }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
		core::ptr::write_bytes(ptr, 0, 0);
        todo!()
    }
}
#[global_allocator]
static ALLOCATOR: Allocator = Allocator;


pub fn size_of<T>(_: T) -> Layout {
	Layout::new::<T>()
}

pub fn malloc<T>(l: T) -> *mut u8 {
	let layout: Layout = size_of(l);
	unsafe {
		ALLOCATOR.alloc(layout) as *mut u8
	}
}