#![no_std]  // do not link standard libraries
#![no_main]
use core::panic::PanicInfo;

#[no_mangle]  // keep function name as-is in genned code
pub extern "C" fn _start() -> ! {  // entry point
	loop {}
}


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {  // custom panic handler
	loop {}
}