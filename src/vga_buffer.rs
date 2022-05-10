#![allow(dead_code)]
pub struct VGABuffer {}

impl VGABuffer {
	const ADDRESS: *mut u8 = 0xb8000 as *mut u8;

	pub fn write(buffer: &[u8], clr: u8) {
		for (i, byte) in buffer.iter().enumerate() {
			unsafe {
				*Self::ADDRESS.add(i * 2) = *byte;
				*Self::ADDRESS.add(i * 2 + 1) = clr;
			}
		}
	}
}