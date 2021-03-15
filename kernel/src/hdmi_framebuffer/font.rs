use core::fmt::Write;

#[derive(Copy, Clone, Debug)]
pub struct CP850Font {
    char_count: usize,
    pub char_size: usize,
    pub char_height: usize,
    pub char_width: usize,
}

impl CP850Font {
    pub const CR: u8 = b'\r';
    pub const LF: u8 = b'\n';
    pub const BELL: u8 = 7;
    pub const BACK: u8 = 8;
    pub const DEL: u8 = 127;

    pub fn new() -> CP850Font {
        use super::freebsd_cp850::FONT_DATA;
        use super::BYTES_PER_CHAR;

        let char_count = FONT_DATA.len() / BYTES_PER_CHAR;
        let char_size = BYTES_PER_CHAR;
        let char_height = 16;
        let char_width = 8;
        CP850Font {
            char_count,
            char_size,
            char_height,
            char_width,
        }
    }

    pub fn get_byte(&self, ch_byte: u8, line: usize) -> u8 {
        use super::freebsd_cp850::FONT_DATA;
        use super::BYTES_PER_CHAR;

        let char_index = match ch_byte as usize {
            index if index < self.char_count => index,
            _ => 0,
        };
        FONT_DATA[(char_index * self.char_size) + line]
    }

    pub fn len(&self) -> usize {
        self.char_count
    }
}


#[derive(Copy, Clone, Debug)]
pub struct Font {
    header_size: usize,
    char_count: usize,
    pub char_size: usize,
    pub char_height: usize,
    pub char_width: usize,
    char_ptr: usize,
}

impl Font {
    pub const CR: u8 = b'\r';
    pub const LF: u8 = b'\n';
    pub const BELL: u8 = 7;
    pub const BACK: u8 = 8;
    pub const DEL: u8 = 127;

    pub fn new() -> Font {
        crate::console::CONSOLE.lock().write_str("Loading raw font...");
        let raw_font = include_bytes!("font.psf");

        let font_header_address = raw_font as *const u8 as usize;
        let font_header_ptr = font_header_address as *const PCScreenFontHeader;
        let (
            header_size,
            char_count,
            char_size,
            char_height,
            char_width,
        ) = unsafe {
            if (*font_header_ptr).magic != PCScreenFontHeader::PSF2_MAGIC {
                panic!("Invalid font data")
            };
            (
                (*font_header_ptr).header_size as usize,
                (*font_header_ptr).length as usize,
                (*font_header_ptr).char_size as usize,
                (*font_header_ptr).height as usize,
                (*font_header_ptr).width as usize,
            )
        };
        let char_ptr = font_header_address + header_size;
        Font {
            header_size,
            char_count,
            char_size,
            char_height,
            char_width,
            char_ptr,
        }
    }

    pub fn get_byte(&self, ch_byte: u8, line: usize) -> u8 {
        let char_index = match ch_byte as usize {
            index if index < self.char_count => index,
            _ => 63,
        };
        let byte_address = self.char_ptr + (char_index * self.char_size)
            + (line * self.bytes_per_line());
        let byte = byte_address as *const u8;

        unsafe {
            *byte
        }
    }

    pub fn len(&self) -> usize {
        self.char_count
    }

    pub fn bytes_per_line(&self) -> usize {
        (self.char_width + 7) / 8
    }
}


#[repr(C)]
struct PCScreenFontHeader {
    magic: u32,
    version: u32,
    header_size: u32,
    flags: u32,
    length: u32,
    char_size: u32,
    height: u32,
    width: u32,
}

impl PCScreenFontHeader {
    pub const PSF2_MAGIC: u32 = 0x864AB572;

}

