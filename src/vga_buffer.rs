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
	row: u8,
	addr: *mut u8
}

impl Writer {
	const HEIGHT: u8 = 25;
	const WIDTH: u8 = 80;

	const fn new() -> Self {
		Self { column: 0, row: 0, addr: VGABuffer::ADDRESS }
	}
}

impl Writer {
	fn write(&mut self, byte: u8, clr: Color) -> Result<(), ()> {
		if self.column >= Self::WIDTH && self.row >= Self::HEIGHT {
			Err(())
		} else {
			if self.column >= Self::WIDTH || byte == b'\n' {
				self.row += 1;
				self.column = 0;
				if byte == b'\n' { return Ok(()); }
			} else {
				self.column += 1;
			}
			unsafe {
				// *VGABuffer::ADDRESS.add((self.column as usize) * 2) = byte;
				// *VGABuffer::ADDRESS.add((self.column as usize) * 2 + 1) = clr as u8;
				let mut pos: *mut u8 = self.addr.add(self.column as usize * 2);
				pos = pos.add((self.row * 80 * 2) as usize);
				*pos = byte;
				*(pos.add(1usize)) = clr as u8;
				// self.addr = self.addr.add(2);
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

	pub const fn new() -> Self {
		Self {
			writer: Writer::new()
		}
	}
}

impl VGABuffer {
	pub fn write(&mut self, bytes: &[u8], clr: Color) -> Result<(), ()> {
		for byte in bytes {
			self.writer.write(*byte, clr)?;
		}
		Ok(())
	}
}


macro_rules! println {
	($l:literal) => {
		let mut writer: VGABuffer = VGABuffer::new();
		let _ = writer.write($l.as_bytes(), Color::White);
		core::mem::drop(writer);
	}
}