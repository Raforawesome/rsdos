#![allow(dead_code)]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
	Black = 0x0,
	Blue = 0x1,
	Green = 0x2,
	Cyan = 0x3,
	Red = 0x4,
	Magenta = 0x5,
	Brown = 0x6,
	LightGray = 0x7,
	DarkGray = 0x8,
	LightBlue = 0x9,
	LightGreen = 0xa,
	LightCyan = 0xb,
	LightRed = 0xc,
	Pink = 0xd,
	Yellow = 0xe,
	White = 0xf,
}


struct Writer {
	column: u8,
	row: u8
}

impl Writer {
	const HEIGHT: u8 = 25;
	const WIDTH: u8 = 80;

	fn new() -> Self {
		Self { column: 0, row: 0 }
	}
}

impl Writer {
	fn write(&mut self, byte: u8, clr: Color) -> Result<(), ()> {
		if self.column >= Self::WIDTH && self.row >= Self::HEIGHT {
			Err(())
		} else {
			if self.column >= Self::WIDTH {
				self.row += 1;
				self.column = 0;
				unsafe {
					*VGABuffer::ADDRESS.add((self.column as usize) * 2) = byte;
					*VGABuffer::ADDRESS.add((self.column as usize) * 2 + 1) = clr as u8;
				}
			} else {
				self.column += 1;
				unsafe {
					*VGABuffer::ADDRESS.add((self.column as usize) * 2) = byte;
					*VGABuffer::ADDRESS.add((self.column as usize) * 2 + 1) = clr as u8;
				}
			}
			Ok(())
		}
	}
}


pub struct VGABuffer {
	writer: Writer
}

impl VGABuffer {
	const ADDRESS: *mut u8 = 0xb8000 as *mut u8;

	pub fn new() -> Self {
		Self {
			writer: Writer::new()
		}
	}

	// pub fn write(buffer: &[u8], clr: Color) {
	// 	for (i, byte) in buffer.iter().enumerate() {
	// 		unsafe {}
	// 	}
	// }
}

impl VGABuffer {
	pub fn write(&mut self, bytes: &[u8], clr: Color) -> Result<(), ()> {
		for byte in bytes {
			self.writer.write(*byte, clr)?
		}
		Ok(())
	}
}