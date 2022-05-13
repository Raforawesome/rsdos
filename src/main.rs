// dummy comment to catch vscode fake error
#![allow(clippy::empty_loop, clippy::missing_safety_doc)]
#![no_std]  // do not link standard libraries
#![no_main]
// #![warn(
// clippy::all,
// clippy::pedantic,
// clippy::nursery,
// clippy::cargo,
// clippy::perf
// )]

mod vga_buffer;
use core::panic::PanicInfo;
use vga_buffer::{ VGABuffer, Color };


const HELLO: &[u8] = b"Boot success";
// VGA Buffer Address: 0xb8000
#[no_mangle]  // keep function name as-is in genned code
pub unsafe extern "C" fn _start() -> ! {  // entry point
	let mut vga_buffer = VGABuffer::new();
	let _ = vga_buffer.write(HELLO, Color::White);

	loop {}
}


#[panic_handler]
const fn panic(_info: &PanicInfo) -> ! {  // custom panic handler
	loop {}
}