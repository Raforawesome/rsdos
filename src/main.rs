#![no_std]
#![no_main]
use core::panic::PanicInfo;

fn main() {
	// println!("Hello, world!");
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	loop {}
}