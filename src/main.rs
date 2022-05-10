#![no_std]  // do not link standard libraries
#![no_main]
use core::panic::PanicInfo;


const HELLO: &[u8] = b"Boot success";
// VGA Buffer Address: 0xb8000
#[no_mangle]  // keep function name as-is in genned code
pub extern "C" fn _start() -> ! {  // entry point
	let vga_buffer: *mut u8 = 0xb8000 as *mut u8;

	for (i, b) in HELLO.iter().enumerate() {
		unsafe {
			*vga_buffer.offset(i as isize * 2) = byte;
			*vga_buffer.offset(i as isize * 2 + 1) = 0xb;
			// Multiplied by 2 because the bytes are written in pairs
			// can be thought of as vga_buffer.add(i).write([byte, 0xb]);
			// i 0:
			// vga_buffer.offset(0)
			// vga_buffer.offset(1)
			// i 1:
			// vga_buffer.offset(2)
			// vga_buffer.offset(3)
			// i 0:
			// vga_buffer.offset(4)
			// vga_buffer.offset(5)
			// and so on
		}
	}
	loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  // custom panic handler
	loop {}
}