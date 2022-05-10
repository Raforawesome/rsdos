#![no_std]
use core::panic::PanicInfo;

fn main() {
	// println!("Hello, world!");
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}