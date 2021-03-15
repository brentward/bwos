// use crate::videocore_mailbox;
// use crate::homer::{HOMER_DATA, HOMER_HEIGHT, HOMER_WIDTH};
// use crate::mutex::Mutex;
// use crate::console::kprintln;
//
// use core::include_bytes;
// use core::mem::transmute;
// use pi::videocore_mailbox::VideoCoreMailbox;
//
// pub struct GlobalFrameBuffer {
//     inner: Option<Framebuffer>
// }
//
// impl GlobalFrameBuffer {
//     const fn new() -> GlobalFrameBuffer {
//         GlobalFrameBuffer { inner: None }
//     }
//
//     fn initialize(&mut self, horizontal_resolution: u32, vertical_resolution: u32) {
//         match self.inner {
//             None => self.inner = Some(Framebuffer::new(horizontal_resolution, vertical_resolution)),
//             _ => (),
//         }
//     }
//
//     fn inner(&mut self) -> &mut Framebuffer {
//         match self.inner {
//             Some(ref mut framebuffer) => framebuffer,
//             _ => {
//                 self.initialize(960, 540);
//                 self.inner()
//             }
//         }
//     }
//
//     pub fn draw_homer(&mut self) {
//         self.inner().draw_homer()
//     }
//
//     // pub fn print_at(&mut self, x: usize, y: usize, s: &str) {
//     //     self.inner().print_at(x, y, s)
//     // }
//
//     pub fn print(&mut self, s: &str) {
//         self.inner().print(s)
//     }
//
//     pub fn scroll_up(&mut self) {
//         self.inner().scroll_up()
//     }
//
//     pub fn set_foreground_color(&mut self, color: &PixelColor) {
//         self.inner().foreground_color.red = color.red;
//         self.inner().foreground_color.green = color.green;
//         self.inner().foreground_color.blue = color.blue;
//         self.inner().foreground_color.alpha = color.alpha;
//     }
//
//     pub fn set_background_color(&mut self, color: &PixelColor) {
//         self.inner().background_color.red = color.red;
//         self.inner().background_color.green = color.green;
//         self.inner().background_color.blue = color.blue;
//         self.inner().background_color.alpha = color.alpha;
//     }
// }
//
// pub static GLOBAL_FRAMEBUFFER: Mutex<GlobalFrameBuffer> = Mutex::new(GlobalFrameBuffer::new());
//
// pub struct Framebuffer {
//     framebuffer_address: usize,
//     framebuffer_len: usize,
//     font_chars_address: usize,
//     font_chars_count: usize,
//     font_char_size: usize,
//     font_char_height: usize,
//     font_char_width: usize,
//     videocore_mailbox: VideoCoreMailbox,
//     pub physical_width: usize,
//     pub physical_height: usize,
//     pub virtual_width: usize,
//     pub virtual_height: usize,
//     pub virtual_offset_x: usize,
//     pub virtual_offset_y: usize,
//     pub bit_depth: usize,
//     pub pixel_order: videocore_mailbox::PixelOrder,
//     pub pitch: usize,
//     pub cursor_x: usize,
//     pub cursor_y: usize,
//     pub foreground_color: PixelColor,
//     pub background_color: PixelColor,
// }
//
// impl Framebuffer {
//     pub fn new(horizontal_resolution: u32, vertical_resolution: u32) -> Framebuffer {
//         // kprintln!("creating mbox buf");
//         let mut mailbox_buf = videocore_mailbox::MailboxBuf::new();
//         // mailbox_buf.size = 35 * 4;
//         // mailbox_buf.set_status_process_request();
//         mailbox_buf.tag_sequence[0] = videocore_mailbox::MailboxTag::SetPhysicalWidthHeight as u32;
//         mailbox_buf.tag_sequence[1] = 8;
//         mailbox_buf.tag_sequence[2] = 8;
//         mailbox_buf.tag_sequence[3] = horizontal_resolution;
//         mailbox_buf.tag_sequence[4] = vertical_resolution;
//
//         mailbox_buf.tag_sequence[5] = videocore_mailbox::MailboxTag::SetVirtualWidthHeight as u32;
//         mailbox_buf.tag_sequence[6] = 8;
//         mailbox_buf.tag_sequence[7] = 8;
//         mailbox_buf.tag_sequence[8] = horizontal_resolution;
//         mailbox_buf.tag_sequence[9] = vertical_resolution * 2;
//         // mailbox_buf.tag_sequence[9] = 65536;
//
//         mailbox_buf.tag_sequence[10] = videocore_mailbox::MailboxTag::SetVirtualOffset as u32;
//         mailbox_buf.tag_sequence[11] = 8;
//         mailbox_buf.tag_sequence[12] = 8;
//         mailbox_buf.tag_sequence[13] = 0;
//         mailbox_buf.tag_sequence[14] = 0;
//
//         mailbox_buf.tag_sequence[15] = videocore_mailbox::MailboxTag::SetDepth as u32;
//         mailbox_buf.tag_sequence[16] = 4;
//         mailbox_buf.tag_sequence[17] = 4;
//         mailbox_buf.tag_sequence[18] = 32;
//
//         mailbox_buf.tag_sequence[19] = videocore_mailbox::MailboxTag::SetPixelOrder as u32;
//         mailbox_buf.tag_sequence[20] = 4;
//         mailbox_buf.tag_sequence[21] = 4;
//         mailbox_buf.tag_sequence[22] = 1;
//
//         mailbox_buf.tag_sequence[23] = videocore_mailbox::MailboxTag::AllocateBuffer as u32;
//         mailbox_buf.tag_sequence[24] = 8;
//         mailbox_buf.tag_sequence[25] = 8;
//         mailbox_buf.tag_sequence[26] = 4096;
//         mailbox_buf.tag_sequence[27] = 0;
//
//         mailbox_buf.tag_sequence[28] = videocore_mailbox::MailboxTag::GetPitch as u32;
//         mailbox_buf.tag_sequence[29] = 4;
//         mailbox_buf.tag_sequence[30] = 4;
//         mailbox_buf.tag_sequence[31] = 0;
//
//         // mailbox_buf.tag_sequence[32] = videocore_mailbox::MailboxTag::EndTag as u32;
//         mailbox_buf.prepare_buf(32);
//         // kprintln!("creating mailbox");
//         let mut videocore_mailbox = videocore_mailbox::VideoCoreMailbox::new();
//         let channel = videocore_mailbox::MailboxChannel::PropertyARMToVC;
//         // kprintln!("calling mailbox");
//         match videocore_mailbox.call(channel, &mailbox_buf) {
//             Ok(_) => (),
//             Err(_) => panic!("mailbox.call() error"),
//         }
//
//         // kprintln!("load font");
//         let framebuffer_address = (mailbox_buf.tag_sequence[26] as usize & 0x3fff_ffff);
//         let framebuffer_len = mailbox_buf.tag_sequence[27] as usize;
//         let physical_width = mailbox_buf.tag_sequence[3] as usize;
//         let physical_height = mailbox_buf.tag_sequence[4] as usize;
//         let virtual_width = mailbox_buf.tag_sequence[8] as usize;
//         let virtual_height = mailbox_buf.tag_sequence[9] as usize;
//         let virtual_offset_x = mailbox_buf.tag_sequence[13] as usize;
//         let virtual_offset_y = mailbox_buf.tag_sequence[14] as usize;
//         let bit_depth = mailbox_buf.tag_sequence[18] as usize;
//         let pixel_order = videocore_mailbox::PixelOrder::from(mailbox_buf.tag_sequence[22]);
//         let pitch = mailbox_buf.tag_sequence[31] as usize;
//         kprintln!("physical_width: {}", physical_width);
//         kprintln!("physical_height: {}", physical_height);
//         kprintln!("virtual_width: {}", virtual_width);
//         kprintln!("virtual_height: {}", virtual_height);
//         kprintln!("virtual_offset_x: {}", virtual_offset_x);
//         kprintln!("virtual_offset_y: {}", virtual_offset_y);
//         // kprintln!("returning framebuffer");
//
//         let raw_font = include_bytes!("font.psf");
//         let font_header_address = raw_font as *const u8 as usize;
//         let font_header_ptr = font_header_address as *const PCScreenFontHeader;
//         // kprintln!("deref font");
//         if unsafe { (*font_header_ptr).magic } != PCScreenFontHeader::PSF2_MAGIC {
//             panic!("Invalid font data")
//         }
//         let header_size = unsafe { (*font_header_ptr).header_size } as usize;
//         let font_chars_count = unsafe { (*font_header_ptr).length } as usize;
//         let font_char_size = unsafe { (*font_header_ptr).char_size } as usize;
//         let font_char_height = unsafe { (*font_header_ptr).height } as usize;
//         let font_char_width = unsafe { (*font_header_ptr).width } as usize;
//         let font_chars_address = font_header_address + header_size;
//         let lines = physical_height / font_char_height;
//         let columns = physical_width / font_char_width;
//         // kprintln!("load font");
//         Framebuffer {
//             framebuffer_address,
//             framebuffer_len,
//             font_chars_address,
//             font_chars_count,
//             font_char_size,
//             font_char_height,
//             font_char_width,
//             videocore_mailbox,
//             physical_width,
//             physical_height,
//             virtual_width,
//             virtual_height,
//             virtual_offset_x,
//             virtual_offset_y,
//             bit_depth,
//             pixel_order,
//             pitch,
//             cursor_x: 0,
//             cursor_y: 0,
//             foreground_color: PixelColor::white(),
//             background_color: PixelColor::black(),
//         }
//     }
//
//     pub fn draw_homer(&mut self) {
//         let mut current_pixel_address = self.framebuffer_address;
//         let mut offset = 0;
//         for _ in 0..HOMER_HEIGHT {
//             for _ in 0..HOMER_WIDTH {
//                 let mut pixel = current_pixel_address as *mut RawPixel;
//                 let pixel_color = PixelColor {
//                     red: (((HOMER_DATA[offset] - 33) << 2) | ((HOMER_DATA[offset + 1] - 33) >> 4)),
//                     green: ((((HOMER_DATA[offset + 1] - 33) & 0xf) << 4) | ((HOMER_DATA[offset + 2] - 33) >> 2)),
//                     blue: ((((HOMER_DATA[offset + 2] - 33) & 0x3) << 6) | (HOMER_DATA[offset + 3] - 33)),
//                     alpha: 0,
//                 };
//                 let colored_pixel: RawPixel = match self.pixel_order {
//                     videocore_mailbox::PixelOrder::RBG => {
//                         unsafe { transmute(RGBPixel::from(&pixel_color)) }
//                     }
//                     videocore_mailbox::PixelOrder::BGR => {
//                         unsafe { transmute(BGRPixel::from(&pixel_color)) }
//                     }
//                 };
//                 unsafe {
//                     *pixel = colored_pixel.clone();
//                 }
//
//                 // match self.pixel_order {
//                 //     videocore_mailbox::PixelOrder::RBG => {
//                 //         let mut pixel = current_pixel_address as *mut RGBPixel;
//                 //         unsafe {
//                 //             (*pixel).red = (((HOMER_DATA[offset] - 33) << 2) | ((HOMER_DATA[offset + 1] -33) >> 4));
//                 //             (*pixel).green = ((((HOMER_DATA[offset + 1] - 33) & 0xf) << 4) | ((HOMER_DATA[offset + 2] - 33) >> 2));
//                 //             (*pixel).blue = ((((HOMER_DATA[offset + 2] - 33) & 0x3) << 6) | (HOMER_DATA[offset + 3] - 33));
//                 //         }
//                 //     }
//                 //     videocore_mailbox::PixelOrder::BGR => {
//                 //         let mut pixel = current_pixel_address as *mut BGRPixel;
//                 //         unsafe {
//                 //             (*pixel).red = (((HOMER_DATA[offset] - 33) << 2) | ((HOMER_DATA[offset + 1] -33) >> 4));
//                 //             (*pixel).green = ((((HOMER_DATA[offset + 1] - 33) & 0xf) << 4) | ((HOMER_DATA[offset + 2] - 33) >> 2));
//                 //             (*pixel).blue = ((((HOMER_DATA[offset + 2] - 33) & 0x3) << 6) | (HOMER_DATA[offset + 3] - 33));
//                 //         }
//                 //     }
//                 // }
//                 offset += 4;
//                 current_pixel_address += (self.bit_depth / 8);
//             }
//             current_pixel_address += self.pitch - (HOMER_WIDTH * (self.bit_depth / 8));
//         }
//         self.cursor_y = HOMER_HEIGHT;
//     }
//
//     fn scroll_up(&mut self) {
//         let virtual_offset_y = ((self.virtual_offset_y + self.font_char_height) % self.physical_height) as u32;
//         let mut mailbox_buf = videocore_mailbox::MailboxBuf::new();
//         // mailbox_buf.size = 7 * 4;
//         // mailbox_buf.set_status_process_request();
//         mailbox_buf.tag_sequence[0] = videocore_mailbox::MailboxTag::SetVirtualOffset as u32;
//         mailbox_buf.tag_sequence[1] = 8;
//         mailbox_buf.tag_sequence[2] = 8;
//         mailbox_buf.tag_sequence[3] = self.virtual_offset_x as u32;
//         mailbox_buf.tag_sequence[4] = virtual_offset_y;
//         mailbox_buf.prepare_buf(5);
//         // mailbox_buf.tag_sequence[5] = videocore_mailbox::MailboxTag::EndTag as u32;
//         let channel = videocore_mailbox::MailboxChannel::PropertyARMToVC;
//         // kprintln!("calling mailbox");
//         match self.videocore_mailbox.call(channel, &mailbox_buf) {
//             Ok(_) => (),
//             Err(_) => panic!("mailbox.call() error"),
//         }
//         self.virtual_offset_x = mailbox_buf.tag_sequence[3] as usize;
//         self.virtual_offset_y = mailbox_buf.tag_sequence[4] as usize;
//         if self.cursor_y >= (self.virtual_offset_y + self.physical_height) {
//             self.cursor_y -= self.physical_height;
//         }
//         for y in self.cursor_y..(self.virtual_offset_y + self.physical_height) {
//             for x in 0..self.physical_width {
//                 let pixel = (self.framebuffer_address + (y * self.pitch) + (x * (self.bit_depth / 8))) as *mut RawPixel;
//                 unsafe { (*pixel).0 = 0 };
//             }
//         }
//
//
//         // for y in (0..self.virtual_resolution_vertical) {
//         //     for x in (0..self.virtual_resolution_horizontal) {
//         //         for pixel_channel in 0..(self.bit_depth as usize / 8) {
//         //             let pixel_byte_address = self.framebuffer_address + (y * self.pitch) + (x * (self.bit_depth as usize / 8)) + pixel_channel;
//         //             let next_pixel_address = pixel_byte_address + (self.pitch * self.font_char_height);
//         //             let pixel_byte = pixel_byte_address as *mut u8;
//         //             let next_pixel_byte = next_pixel_address as *const u8;
//         //             if y < self.virtual_resolution_vertical - self.font_char_height {
//         //                 unsafe {
//         //                     *pixel_byte = *next_pixel_byte;
//         //                 }
//         //             } else {
//         //                 unsafe {
//         //                     *pixel_byte = 0;
//         //                 }
//         //             }
//         //
//         //         }
//         //     }
//         // }
//     }
//
//     // pub fn set_background_color(&mut self, background: PixelColor) {
//     //     self.background_color = background;
//     // }
//     //
//     // pub fn set_foreground_color(&mut self, foreground: PixelColor) {
//     //     self.foreground_color = foreground;
//     // }
//     //
//     fn advance_cursor(&mut self, offset: usize) {
//         unimplemented!("Framebuffer::advance_cursor()")
//     }
//
//     pub fn print(&mut self, s: &str) {
//         // let raw_font = include_bytes!("font.psf");
//         // let font_header_address = raw_font as *const u8 as usize;
//         // let font_header_ptr = font_header_address as *const PCScreenFontHeader;
//         // if unsafe { (*font_header_ptr).magic } != PCScreenFontHeader::PSF2_MAGIC {
//         //     panic!("Invalid font data")
//         // }
//         // let header_size = unsafe { (*font_header_ptr).header_size } as usize;
//         // let length = unsafe { (*font_header_ptr).length } as usize;
//         // let char_size = unsafe { (*font_header_ptr).char_size } as usize;
//         // let height = unsafe { (*font_header_ptr).height } as usize;
//         // let width = unsafe { (*font_header_ptr).width } as usize;
//         // kprintln!("font_header_address: {}", font_header_address);
//         //
//         // kprintln!("Header_size: {}", header_size);
//         // kprintln!("length: {}", length);
//         // kprintln!("char_size: {}", char_size);
//         // kprintln!("height: {}", height);
//         // kprintln!("width: {}", width);
//         for ch in s.chars() {
//             let cursor_y_alternate = if self.cursor_y < self.physical_height {
//                 self.cursor_y + self.physical_height
//             } else {
//                 self.cursor_y % self.physical_height
//             };
//             let mut offset_actual = (self.cursor_y * self.pitch) + (self.cursor_x * (self.bit_depth / 8));
//             let mut offset_alternate = (cursor_y_alternate * self.pitch) + (self.cursor_x * (self.bit_depth / 8));
//             // let mut line = offset;
//             // let mut mask = 0;
//             let bytes_per_line = (self.font_char_width + 7) / 8;
//             let colored_pixel: RawPixel = match self.pixel_order {
//                 videocore_mailbox::PixelOrder::RBG => {
//                     unsafe { transmute(RGBPixel::from(&self.foreground_color)) }
//                 }
//                 videocore_mailbox::PixelOrder::BGR => {
//                     unsafe { transmute(BGRPixel::from(&self.foreground_color)) }
//                 }
//             };
//             let blank_pixel = RawPixel(0);
//             match ch {
//                 '\r' => self.cursor_x = 0,
//                 '\n' => {
//                     self.cursor_x = 0;
//                     self.cursor_y += self.font_char_height;
//                     while self.cursor_y >= (self.virtual_offset_y + self.physical_height) {
//                         self.scroll_up();
//                     }
//
//                 }
//                 ch => {
//                     let glyph_index = ch as usize;
//                     if glyph_index < self.font_chars_count {
//                         let mut glyph_address = self.font_chars_address + (glyph_index * self.font_char_size);
//
//                         for _ in 0..self.font_char_height {
//                             let mut line = offset_actual;
//                             let mut line_alternate = offset_alternate;
//                             let mut mask = 1 << (self.font_char_width - 1);
//                             for _ in 0..self.font_char_width {
//                                 let glyph = glyph_address as *const u8;
//                                 let mut current_pixel = (self.framebuffer_address + line) as *mut RawPixel;
//                                 let mut alternate_pixel = (self.framebuffer_address + line_alternate) as *mut RawPixel;
//
//                                 unsafe {
//                                     match unsafe { *glyph & mask } == 0 {
//                                         true => {
//                                             *current_pixel = blank_pixel.clone();
//                                             *alternate_pixel = blank_pixel.clone();
//                                         },
//                                         false => {
//                                             *current_pixel = colored_pixel.clone();
//                                             *alternate_pixel = colored_pixel.clone();
//                                         },
//                                     }
//                                 }
//                                 mask >>= 1;
//                                 line += 4;
//                                 line_alternate += 4;
//                             }
//                             glyph_address += bytes_per_line;
//                             offset_actual += self.pitch;
//                             offset_alternate += self.pitch
//                         }
//                         self.cursor_x += self.font_char_width;
//                         if self.cursor_x >= self.physical_width {
//                             self.cursor_x = 0;
//                             self.cursor_y += self.font_char_height;
//                             while self.cursor_y >= (self.virtual_offset_y + self.physical_height) {
//                                 self.scroll_up();
//                             }
//                         }
//
//
//                     } else {
//                         kprintln!("Character out of bounds of font");
//                     }
//                 }
//             }
//
//         }
//     }
// }
//
// #[repr(C)]
// struct PCScreenFontHeader {
//     magic: u32,
//     version: u32,
//     header_size: u32,
//     flags: u32,
//     length: u32,
//     char_size: u32,
//     height: u32,
//     width: u32,
// }
//
// impl PCScreenFontHeader {
//     const PSF2_MAGIC: u32 = 0x864AB572;
// }
//
// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct RGBPixel {
//     pub red: u8,
//     pub green: u8,
//     pub blue: u8,
//     pub alpha: u8,
// }
//
// impl From<&PixelColor> for RGBPixel {
//     fn from(color: &PixelColor) -> Self {
//         Self {
//             red: color.red,
//             green: color.green,
//             blue: color.blue,
//             alpha: color.alpha
//         }
//     }
// }
//
//
// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct BGRPixel {
//     pub blue: u8,
//     pub green: u8,
//     pub red: u8,
//     pub alpha: u8,
// }
//
// impl From<&PixelColor> for BGRPixel {
//     fn from(color: &PixelColor) -> Self {
//         Self {
//             red: color.red,
//             green: color.green,
//             blue: color.blue,
//             alpha: color.alpha
//         }
//     }
// }
//
// #[repr(C)]
// #[derive(Copy, Clone)]
// pub struct RawPixel(u32);
//
// pub struct PixelColor {
//     pub blue: u8,
//     pub green: u8,
//     pub red: u8,
//     pub alpha: u8,
// }
//
// impl PixelColor {
//     pub fn white() -> PixelColor {
//         PixelColor {
//             red: 203,
//             green: 203,
//             blue: 203,
//             alpha: 0,
//         }
//     }
//
//     pub fn grey() -> PixelColor {
//         PixelColor {
//             red: 127,
//             green: 127,
//             blue: 127,
//             alpha: 0,
//         }
//     }
//     pub fn red() -> PixelColor {
//         PixelColor {
//             red: 191,
//             green: 47,
//             blue: 31,
//             alpha: 0,
//         }
//     }
//     pub fn green() -> PixelColor {
//         PixelColor {
//             red: 31,
//             green: 191,
//             blue: 31,
//             alpha: 0,
//         }
//     }
//     pub fn blue() -> PixelColor {
//         PixelColor {
//             red: 71,
//             green: 49,
//             blue: 223,
//             alpha: 0,
//         }
//     }
//     pub fn yellow() -> PixelColor {
//         PixelColor {
//             red: 167,
//             green: 167,
//             blue: 39,
//             alpha: 0,
//         }
//     }
//     pub fn cyan() -> PixelColor {
//         PixelColor {
//             red: 47,
//             green: 191,
//             blue: 199,
//             alpha: 0,
//         }
//     }
//     pub fn magenta() -> PixelColor {
//         PixelColor {
//             red: 211,
//             green: 55,
//             blue: 211,
//             alpha: 0,
//         }
//     }
//
//     pub fn b_white() -> PixelColor {
//         PixelColor {
//             red: 255,
//             green: 255,
//             blue: 255,
//             alpha: 0,
//         }
//     }
//
//     pub fn black() -> PixelColor {
//         PixelColor {
//             red: 0,
//             green: 0,
//             blue: 0,
//             alpha: 0,
//         }
//     }
//     pub fn b_red() -> PixelColor {
//         PixelColor {
//             red: 255,
//             green: 0,
//             blue: 0,
//             alpha: 0,
//         }
//     }
//     pub fn b_green() -> PixelColor {
//         PixelColor {
//             red: 0,
//             green: 255,
//             blue: 0,
//             alpha: 0,
//         }
//     }
//     pub fn b_blue() -> PixelColor {
//         PixelColor {
//             red: 0,
//             green: 0,
//             blue: 255,
//             alpha: 0,
//         }
//     }
//     pub fn b_yellow() -> PixelColor {
//         PixelColor {
//             red: 255,
//             green: 255,
//             blue: 0,
//             alpha: 0,
//         }
//     }
//     pub fn b_cyan() -> PixelColor {
//         PixelColor {
//             red: 0,
//             green: 255,
//             blue: 255,
//             alpha: 0,
//         }
//     }
//     pub fn b_magenta() -> PixelColor {
//         PixelColor {
//             red: 255,
//             green: 0,
//             blue: 255,
//             alpha: 0,
//         }
//     }
//
//     pub fn from_hex(hex: u32) -> PixelColor {
//         let red = ((hex >> 16) & 0xff) as u8;
//         let green = ((hex >> 8) & 0xff) as u8;
//         let blue = (hex & 0xff) as u8;
//         PixelColor {
//             blue,
//             green,
//             red,
//             alpha: 0,
//         }
//     }
// }
//
// impl From<u32> for PixelColor {
//     fn from(value: u32) -> Self {
//         unsafe { transmute(value) }
//     }
// }
//
// // struct TextBuffer {
// //     line_00: [char; 210],
// //     line_01: [char; 210],
// //     line_02: [char; 210],
// //     line_03: [char; 210],
// //     line_04: [char; 210],
// //     line_05: [char; 210],
// //     line_06: [char; 210],
// //     line_07: [char; 210],
// //     line_08: [char; 210],
// //     line_09: [char; 210],
// //     line_10: [char; 210],
// //     line_11: [char; 210],
// //     line_12: [char; 210],
// //     line_13: [char; 210],
// //     line_14: [char; 210],
// //     line_15: [char; 210],
// //     line_16: [char; 210],
// //     line_17: [char; 210],
// //     line_18: [char; 210],
// //     line_19: [char; 210],
// //     line_20: [char; 210],
// //     line_21: [char; 210],
// //     line_22: [char; 210],
// //     line_23: [char; 210],
// //     line_24: [char; 210],
// //     line_25: [char; 210],
// //     line_26: [char; 210],
// //     line_27: [char; 210],
// //     line_28: [char; 210],
// //     line_29: [char; 210],
// //     line_30: [char; 210],
// //     line_31: [char; 210],
// //     line_32: [char; 210],
// //     line_33: [char; 210],
// //     line_34: [char; 210],
// //     line_35: [char; 210],
// //     line_36: [char; 210],
// //     line_37: [char; 210],
// //     line_38: [char; 210],
// //     line_39: [char; 210],
// //     line_40: [char; 210],
// //     line_41: [char; 210],
// //     line_42: [char; 210],
// //     line_43: [char; 210],
// //     line_44: [char; 210],
// //     line_45: [char; 210],
// //     line_46: [char; 210],
// //     line_47: [char; 210],
// //     line_48: [char; 210],
// //     line_49: [char; 210],
// //     line_50: [char; 210],
// //     line_51: [char; 210],
// //     line_52: [char; 210],
// //     line_53: [char; 210],
// //     line_54: [char; 210],
// //     line_55: [char; 210],
// //     line_56: [char; 210],
// //     line_57: [char; 210],
// //     line_58: [char; 210],
// //     line_59: [char; 210],
// //     line_60: [char; 210],
// //     line_61: [char; 210],
// //     line_62: [char; 210],
// //     line_63: [char; 210],
// //     line_64: [char; 210],
// // }


// for line_index in 0..FONT_HEIGHT {
//     for glyph_index in 0..FONT_WIDTH {
//         let mut current_pixel = (self.framebuffer_address +
//             offset_actual + (line_index * self.pitch) +
//             (glyph_index * (self.bit_depth / 8))) as *mut RawPixel;
//         let mut alternate_pixel = (self.framebuffer_address +
//             offset_alternate + (line_index * self.pitch) +
//             (glyph_index * (self.bit_depth / 8))) as *mut RawPixel;
//         let pixel = self.get_pixel_data(glyph_byte, glyph_index);
//         unsafe {
//             *current_pixel = pixel;
//             *alternate_pixel = pixel;
//         }
//     }
// }
// self.cursor_x += FONT_WIDTH;
// if self.cursor_x >= self.physical_width {
//     self.cursor_x = 0;
//     self.cursor_y += FONT_HEIGHT;
//     while self.cursor_y >= (self.virtual_offset_y + self.physical_height) {
//         self.scroll_up();
//     }
// }
//
//
