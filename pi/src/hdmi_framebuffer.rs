use shim::io;

use crate::videocore_mailbox;
use crate::homer::{HOMER_HEIGHT, HOMER_WIDTH, HOMER_DATA};
use crate::font::{CR, LF, BACK, DEL, CP850Font as Font};
use crate::font::charset::Char;

pub struct HDMIFrameBuffer {
    framebuffer_address: usize,
    videocore_mailbox: videocore_mailbox::VideoCoreMailbox,
    physical_width: usize,
    physical_height: usize,
    virtual_width: usize,
    virtual_height: usize,
    virtual_offset_x: usize,
    virtual_offset_y: usize,
    bit_depth: usize,
    pixel_order: videocore_mailbox::PixelOrder,
    pitch: usize,
    cursor_x: usize,
    cursor_y: usize,
    pub foreground_color: PixelColor,
    pub background_color: PixelColor,
    font: Font,
    cursor: char,
    columns: usize,
}

impl HDMIFrameBuffer {
    pub fn new(config: HDMIConfig) -> HDMIFrameBuffer {
        let mut mailbox_buf = videocore_mailbox::MailboxBuf::new();
        mailbox_buf.tag_sequence[0] = videocore_mailbox::MailboxTag::SetPhysicalWidthHeight as u32;
        mailbox_buf.tag_sequence[1] = 8;
        mailbox_buf.tag_sequence[2] = 8;
        mailbox_buf.tag_sequence[3] = config.horizontal_resolution;
        mailbox_buf.tag_sequence[4] = config.vertical_resolution;

        mailbox_buf.tag_sequence[5] = videocore_mailbox::MailboxTag::SetVirtualWidthHeight as u32;
        mailbox_buf.tag_sequence[6] = 8;
        mailbox_buf.tag_sequence[7] = 8;
        mailbox_buf.tag_sequence[8] = config.horizontal_resolution;
        mailbox_buf.tag_sequence[9] = config.vertical_resolution * config.virtual_vertical_multiplier;

        mailbox_buf.tag_sequence[10] = videocore_mailbox::MailboxTag::SetVirtualOffset as u32;
        mailbox_buf.tag_sequence[11] = 8;
        mailbox_buf.tag_sequence[12] = 8;
        mailbox_buf.tag_sequence[13] = 0;
        mailbox_buf.tag_sequence[14] = 0;

        mailbox_buf.tag_sequence[15] = videocore_mailbox::MailboxTag::SetDepth as u32;
        mailbox_buf.tag_sequence[16] = 4;
        mailbox_buf.tag_sequence[17] = 4;
        mailbox_buf.tag_sequence[18] = config.color_depth;

        mailbox_buf.tag_sequence[19] = videocore_mailbox::MailboxTag::SetPixelOrder as u32;
        mailbox_buf.tag_sequence[20] = 4;
        mailbox_buf.tag_sequence[21] = 4;
        mailbox_buf.tag_sequence[22] = 0;

        mailbox_buf.tag_sequence[23] = videocore_mailbox::MailboxTag::AllocateBuffer as u32;
        mailbox_buf.tag_sequence[24] = 8;
        mailbox_buf.tag_sequence[25] = 8;
        mailbox_buf.tag_sequence[26] = 4096;
        mailbox_buf.tag_sequence[27] = 0;

        mailbox_buf.tag_sequence[28] = videocore_mailbox::MailboxTag::GetPitch as u32;
        mailbox_buf.tag_sequence[29] = 4;
        mailbox_buf.tag_sequence[30] = 0;
        mailbox_buf.tag_sequence[31] = 0;

        mailbox_buf.prepare_buf(32);
        let mut videocore_mailbox = videocore_mailbox::VideoCoreMailbox::new();
        let channel = videocore_mailbox::MailboxChannel::PropertyARMToVC;
        match videocore_mailbox.call(channel, &mailbox_buf) {
            Ok(_) => (),
            Err(_) => panic!("mailbox.call() error"),
        }

        let framebuffer_address = mailbox_buf.tag_sequence[26] as usize & 0x3fff_ffff;
        let _framebuffer_len = mailbox_buf.tag_sequence[27] as usize;
        let physical_width = mailbox_buf.tag_sequence[3] as usize;
        let physical_height = mailbox_buf.tag_sequence[4] as usize;
        let virtual_width = mailbox_buf.tag_sequence[8] as usize;
        let virtual_height = mailbox_buf.tag_sequence[9] as usize;
        let virtual_offset_x = mailbox_buf.tag_sequence[13] as usize;
        let virtual_offset_y = mailbox_buf.tag_sequence[14] as usize;
        let bit_depth = mailbox_buf.tag_sequence[18] as usize;
        let pixel_order = videocore_mailbox::PixelOrder::from(mailbox_buf.tag_sequence[22]);
        let pitch = mailbox_buf.tag_sequence[31] as usize;

        let font = Font::new();
        HDMIFrameBuffer {
            framebuffer_address,
            videocore_mailbox,
            physical_width,
            physical_height,
            virtual_width,
            virtual_height,
            virtual_offset_x,
            virtual_offset_y,
            bit_depth,
            pixel_order,
            pitch,
            cursor_x: 0,
            cursor_y: 0,
            foreground_color: PixelColor::white(),
            background_color: PixelColor::black(),
            font,
            cursor: '_',
            columns: physical_width / font.char_width,
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            CR => {
                self.clear_cursor();
                self.cursor_x = 0;
            },
            LF => {
                self.next_line();
            }
            DEL => {
                self.clear_cursor();
                self.cursor_decrement();
                self.clear_cursor();
            }
            BACK => {
                self.clear_cursor();
                self.cursor_decrement();
                self.clear_cursor();
            }
            byte => {
                self.draw_char_byte(byte);
                self.cursor_increment();
            }
        }
    }

    pub fn clear(&self) {
        self.clear_lines(0, self.virtual_height);
    }

    pub fn draw_cursor(&self) {
        let ch_byte = if self.cursor.is_ascii() {
            self.cursor as u8
        } else {
            0
        };
        self.draw_char_byte(ch_byte);
    }

    pub fn get_raw_pixel(&self, color: &PixelColor) -> RawPixel {
        match self.pixel_order {
            videocore_mailbox::PixelOrder::RBG => {
                unsafe { ::core::mem::transmute(RGBPixel::from(color)) }
            }
            videocore_mailbox::PixelOrder::BGR => {
                unsafe { ::core::mem::transmute(BGRPixel::from(color)) }
            }
        }
    }

    pub fn set_pixel(&self, x: usize, y: usize, pixel: RawPixel) -> Result<(), ()> {
        if x >= self.virtual_width || y >= self.virtual_height {
            Err(())
        } else {
            let offset = y * self.pitch + (x * (self.bit_depth / 8));
            let current_pixel = (self.framebuffer_address + offset) as *mut RawPixel;
            unsafe { *current_pixel = pixel }
            Ok(())
        }
    }

    pub fn physical_width(&self) -> usize {
        self.physical_width
    }

    pub fn physical_height(&self) -> usize {
        self.physical_height
    }

    pub fn virtual_width(&self) -> usize {
        self.virtual_width
    }

    pub fn virtual_height(&self) -> usize {
        self.virtual_height
    }

    pub fn virtual_offset_x(&self) -> usize {
        self.virtual_offset_x
    }

    pub fn virtual_offset_y(&self) -> usize {
        self.virtual_offset_y
    }

    pub fn cursor_x(&self) -> usize {
        self.cursor_x
    }

    pub fn cursor_y(&self) -> usize {
        self.cursor_y
    }

    pub fn draw_homer(&mut self) {
        let offset = (self.cursor_y * self.pitch) + (self.cursor_x * (self.bit_depth / 8));
        let offset_alt = (self.cursor_y_alt() * self.pitch) + (self.cursor_x * (self.bit_depth / 8));

        let mut pixel_address = self.framebuffer_address + offset;
        let mut pixel_address_alt = self.framebuffer_address + offset_alt;

        let mut homer_offset = 0;
        for _ in 0..HOMER_HEIGHT {
            for _ in 0..HOMER_WIDTH {
                let pixel = pixel_address as *mut RawPixel;
                let pixel_alt = pixel_address_alt as *mut RawPixel;
                let pixel_color = PixelColor {
                    red: (((HOMER_DATA[homer_offset] - 33) << 2) | ((HOMER_DATA[homer_offset + 1] - 33) >> 4)),
                    green: ((((HOMER_DATA[homer_offset + 1] - 33) & 0xf) << 4) | ((HOMER_DATA[homer_offset + 2] - 33) >> 2)),
                    blue: ((((HOMER_DATA[homer_offset + 2] - 33) & 0x3) << 6) | (HOMER_DATA[homer_offset + 3] - 33)),
                    alpha: 0,
                };
                let homer_pixel: RawPixel = match self.pixel_order {
                    videocore_mailbox::PixelOrder::RBG => {
                        unsafe { ::core::mem::transmute(RGBPixel::from(&pixel_color)) }
                    }
                    videocore_mailbox::PixelOrder::BGR => {
                        unsafe { ::core::mem::transmute(BGRPixel::from(&pixel_color)) }
                    }
                };
                unsafe {
                    *pixel = homer_pixel;
                    *pixel_alt = homer_pixel;
                }
                homer_offset += 4;
                pixel_address += self.bit_depth / 8;
                pixel_address_alt += self.bit_depth / 8;
            }
            pixel_address += self.pitch - (HOMER_WIDTH * (self.bit_depth / 8));
            pixel_address_alt += self.pitch - (HOMER_WIDTH * (self.bit_depth / 8));
        }
        self.cursor_y += HOMER_HEIGHT;
    }

    fn clear_lines(&self, line: usize, count: usize) {
        let color = self.get_raw_pixel(&self.background_color);
        let color_bytes = color.0.to_be_bytes();
        let block_bytes = [
            color_bytes[0], color_bytes[1], color_bytes[2], color_bytes[3],
            color_bytes[0], color_bytes[1], color_bytes[2], color_bytes[3],
            color_bytes[0], color_bytes[1], color_bytes[2], color_bytes[3],
            color_bytes[0], color_bytes[1], color_bytes[2], color_bytes[3],
        ];
        let mut line_start_address = self.framebuffer_address + (line * self.pitch);
        let mut pixel_end_exclusinve_address = line_start_address + (self.virtual_width * (self.bit_depth / 4));
        for _ in 0..count {
            for data_block_address in (line_start_address..pixel_end_exclusinve_address)
                .step_by(::core::mem::size_of::<u128>()) {
                let data_ptr = data_block_address as *mut u128;
                unsafe { *data_ptr = u128::from_be_bytes(block_bytes); }
            }
            line_start_address += self.pitch;
            pixel_end_exclusinve_address += self.pitch;
        }
    }

    fn alt_offset(&self) -> usize {
        self.virtual_height / 2
    }

    fn cursor_y_alt(&self) -> usize {
        (self.cursor_y + self.alt_offset()) % self.virtual_height
    }

    fn next_line(&mut self) {
        self.cursor_y = self.cursor_y + self.font.char_height;
        if self.cursor_y + self.font.char_height > self.virtual_height {
            self.cursor_y = (self.cursor_y + self.alt_offset()) % self.virtual_height;
            self.virtual_offset_y = (self.virtual_offset_y + self.alt_offset()) % self.virtual_height;
        }
        self.clear_lines(self.cursor_y, self.font.char_height);
        self.clear_lines(self.cursor_y_alt(), self.font.char_height);
        // while self.cursor_y + self.font.char_height >= self.virtual_offset_y + self.physical_height {
        //     let mut new_virtual_offset_y= self.virtual_offset_y + 1;
        //     let mut mailbox_buf = videocore_mailbox::MailboxBuf::new();
        //     mailbox_buf.tag_sequence[0] = videocore_mailbox::MailboxTag::SetVirtualOffset as u32;
        //     mailbox_buf.tag_sequence[1] = 8;
        //     mailbox_buf.tag_sequence[2] = 8;
        //     mailbox_buf.tag_sequence[3] = self.virtual_offset_x as u32;
        //     mailbox_buf.tag_sequence[4] = new_virtual_offset_y as u32;
        //     mailbox_buf.prepare_buf(5);
        //     let channel = videocore_mailbox::MailboxChannel::PropertyARMToVC;
        //
        //     match self.videocore_mailbox.call(channel, &mailbox_buf) {
        //         Ok(_) => (),
        //         Err(_) => panic!("mailbox.call() error"),
        //     }
        //     self.virtual_offset_x = mailbox_buf.tag_sequence[3] as usize;
        //     self.virtual_offset_y = mailbox_buf.tag_sequence[4] as usize;
        // }
        if self.cursor_y >= self.virtual_offset_y + self.physical_height {
            let new_virtual_offset_y = self.cursor_y - (self.physical_height - self.font.char_height);
            let mut mailbox_buf = videocore_mailbox::MailboxBuf::new();
            mailbox_buf.tag_sequence[0] = videocore_mailbox::MailboxTag::SetVirtualOffset as u32;
            mailbox_buf.tag_sequence[1] = 8;
            mailbox_buf.tag_sequence[2] = 8;
            mailbox_buf.tag_sequence[3] = self.virtual_offset_x as u32;
            mailbox_buf.tag_sequence[4] = new_virtual_offset_y as u32;
            mailbox_buf.prepare_buf(5);
            let channel = videocore_mailbox::MailboxChannel::PropertyARMToVC;

            match self.videocore_mailbox.call(channel, &mailbox_buf) {
                Ok(_) => (),
                Err(_) => panic!("mailbox.call() error"),
            }
            self.virtual_offset_x = mailbox_buf.tag_sequence[3] as usize;
            self.virtual_offset_y = mailbox_buf.tag_sequence[4] as usize;
        }
    }

    fn cursor_increment(&mut self) {
        self.cursor_x += self.font.char_width;
        if self.cursor_x >= self.columns * self.font.char_width {
            self.cursor_x = 0;
            self.next_line();
        }
    }

    fn cursor_decrement(&mut self) {
        if self.cursor_x < self.font.char_width {
            self.cursor_x = (self.columns - 1) * self.font.char_width;
            self.cursor_y -= self.font.char_height;
        } else {
            self.cursor_x -= self.font.char_width;
        }
    }

    fn clear_cursor(&self) {
        let pixel = self.get_raw_pixel(&self.background_color);
        let offset = (self.cursor_y * self.pitch) + (self.cursor_x * (self.bit_depth / 8));
        let offset_alt = (self.cursor_y_alt() * self.pitch) + (self.cursor_x * (self.bit_depth / 8));
        for line_index in 0..self.font.char_height {
            for pixel_index in 0..self.font.char_width {
                let pixel_offset = (pixel_index * (self.bit_depth / 8)) + (line_index * self.pitch);
                let current_pixel = (self.framebuffer_address + offset + pixel_offset) as *mut RawPixel;
                let alternate_pixel = (self.framebuffer_address + offset_alt + pixel_offset) as *mut RawPixel;
                unsafe {
                    *current_pixel = pixel;
                    *alternate_pixel = pixel;
                }
            }
        }
    }

    fn get_pixel_data(&self, font_byte: u8, index: usize) -> RawPixel {
        if index > 7 { panic!("pixel index is out of expected bounds for FontByte (>7)") }
        match (font_byte & (1 << (7 - index))) == 0 {
            true => self.get_raw_pixel(&self.background_color),
            false => self.get_raw_pixel(&self.foreground_color),
        }
    }

    fn draw_char_byte(&self, char_byte: u8) {
        let offset = (self.cursor_y * self.pitch) + (self.cursor_x * (self.bit_depth / 8));
        let offset_alt = (self.cursor_y_alt() * self.pitch) + (self.cursor_x * (self.bit_depth / 8));
        for line_index in 0..self.font.char_height {
            let char_line_byte = self.font.get_byte(char_byte, line_index);
            for pixel_index in 0..self.font.char_width {
                let pixel_offset = (pixel_index * (self.bit_depth / 8)) + (line_index * self.pitch);
                let current_pixel = (self.framebuffer_address + offset + pixel_offset) as *mut RawPixel;
                let alternate_pixel = (self.framebuffer_address + offset_alt + pixel_offset) as *mut RawPixel;
                let pixel = self.get_pixel_data(char_line_byte, pixel_index);
                unsafe {
                    *current_pixel = pixel;
                    *alternate_pixel = pixel;
                }
            }
        }
    }
}

impl core::fmt::Write for HDMIFrameBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            let byte  = Char::map_char(ch) as u8;
            if byte == b'\n' && self.cursor_x != 0 {
                self.write_byte(b'\r');
            }
            self.write_byte(byte);
        }
        Ok(())
    }
}

impl io::Write for HDMIFrameBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let mut byte_count = 0;
        for byte in buf {
            self.write_byte(*byte);
            byte_count += 1;
        }
        Ok(byte_count)
    }

    fn flush(&mut self) -> Result<(), io::Error> {
        Ok(())
    }
}

/// Describes a video mode.
#[derive(Debug)]
pub struct HDMIConfig {
    pub horizontal_resolution: u32,
    pub vertical_resolution: u32,
    pub color_depth: u32,
    pub virtual_vertical_multiplier: u32,
}

impl HDMIConfig {
    pub fn new(horizontal_resolution: u32, vertical_resolution: u32,
               color_depth: u32, virtual_vertical_multiplier: u32) -> HDMIConfig {
        HDMIConfig {
            horizontal_resolution,
            vertical_resolution,
            color_depth,
            virtual_vertical_multiplier,
        }
    }
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Color {
    White = 7,
    Yellow = 6,
    Magenta = 5,
    Red = 4,
    Cyan = 3,
    Green = 2,
    Blue = 1,
    Black = 0,
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct RGBPixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl From<&PixelColor> for RGBPixel {
    fn from(color: &PixelColor) -> Self {
        Self {
            red: color.red,
            green: color.green,
            blue: color.blue,
            alpha: color.alpha
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct BGRPixel {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8,
}

impl From<&PixelColor> for BGRPixel {
    fn from(color: &PixelColor) -> Self {
        Self {
            red: color.red,
            green: color.green,
            blue: color.blue,
            alpha: color.alpha
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct RawPixel(pub u32);

pub struct PixelColor {
    pub blue: u8,
    pub green: u8,
    pub red: u8,
    pub alpha: u8,
}

impl PixelColor {
    pub fn new(red: u8, green: u8, blue: u8) -> PixelColor {
        PixelColor {
            red,
            green,
            blue,
            alpha: 0
        }
    }

    pub fn white() -> PixelColor {
        PixelColor {
            red: 203,
            green: 203,
            blue: 203,
            alpha: 0,
        }
    }

    pub fn grey() -> PixelColor {
        PixelColor {
            red: 127,
            green: 127,
            blue: 127,
            alpha: 0,
        }
    }
    pub fn red() -> PixelColor {
        PixelColor {
            red: 191,
            green: 47,
            blue: 31,
            alpha: 0,
        }
    }
    pub fn green() -> PixelColor {
        PixelColor {
            red: 31,
            green: 191,
            blue: 31,
            alpha: 0,
        }
    }
    pub fn blue() -> PixelColor {
        PixelColor {
            red: 71,
            green: 49,
            blue: 223,
            alpha: 0,
        }
    }
    pub fn yellow() -> PixelColor {
        PixelColor {
            red: 167,
            green: 167,
            blue: 39,
            alpha: 0,
        }
    }
    pub fn cyan() -> PixelColor {
        PixelColor {
            red: 47,
            green: 191,
            blue: 199,
            alpha: 0,
        }
    }
    pub fn magenta() -> PixelColor {
        PixelColor {
            red: 211,
            green: 55,
            blue: 211,
            alpha: 0,
        }
    }

    pub fn b_white() -> PixelColor {
        PixelColor {
            red: 255,
            green: 255,
            blue: 255,
            alpha: 0,
        }
    }

    pub fn black() -> PixelColor {
        PixelColor {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    }
    pub fn b_red() -> PixelColor {
        PixelColor {
            red: 255,
            green: 0,
            blue: 0,
            alpha: 0,
        }
    }
    pub fn b_green() -> PixelColor {
        PixelColor {
            red: 0,
            green: 255,
            blue: 0,
            alpha: 0,
        }
    }
    pub fn b_blue() -> PixelColor {
        PixelColor {
            red: 0,
            green: 0,
            blue: 255,
            alpha: 0,
        }
    }
    pub fn b_yellow() -> PixelColor {
        PixelColor {
            red: 255,
            green: 255,
            blue: 0,
            alpha: 0,
        }
    }
    pub fn b_cyan() -> PixelColor {
        PixelColor {
            red: 0,
            green: 255,
            blue: 255,
            alpha: 0,
        }
    }
    pub fn b_magenta() -> PixelColor {
        PixelColor {
            red: 255,
            green: 0,
            blue: 255,
            alpha: 0,
        }
    }

    pub fn from_hex(hex: u32) -> PixelColor {
        let red = ((hex >> 16) & 0xff) as u8;
        let green = ((hex >> 8) & 0xff) as u8;
        let blue = (hex & 0xff) as u8;
        PixelColor {
            blue,
            green,
            red,
            alpha: 0,
        }
    }

}

impl From<u32> for PixelColor {
    fn from(value: u32) -> Self {
        unsafe { ::core::mem::transmute(value) }
    }
}
