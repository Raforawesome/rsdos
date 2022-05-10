#![no_std]  // do not link standard libraries
#![no_main]
mod vga_buffer;
use core::panic::PanicInfo;
use vga_buffer::VGABuffer;


const HELLO: &[u8] = b"Boot success";
// VGA Buffer Address: 0xb8000
#[no_mangle]  // keep function name as-is in genned code
pub extern "C" fn _start() -> ! {  // entry point
	VGABuffer::write(HELLO, 0xb);

	loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  // custom panic handler
	loop {}
}