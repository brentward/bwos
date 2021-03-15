// use crate::videocore_mailbox;
// use crate::homer::{homer_data, homer_height, homer_width};
//
// pub struct Framebuffer {
//     framebuffer_address: usize,
//     len: usize,
//     pub physical_resolution_horizontal: usize,
//     pub physical_resolution_vertical: usize,
//     pub virtual_resolution_horizontal: usize,
//     pub virtual_resolution_vertical: usize,
//     pub virtual_offset_horizontal: usize,
//     pub virtual_offset_vertical: usize,
//     pub bit_depth: u32,
//     pub pixel_order: videocore_mailbox::PixelOrder,
//     pub pitch: usize
// }
//
// impl Framebuffer {
//     pub fn new() -> Framebuffer {
//         let mut mailbox_buf = videocore_mailbox::MailboxBuf::new();
//         mailbox_buf.size = 35 * 4;
//         mailbox_buf.set_status_process_request();
//         mailbox_buf.tag_sequence[0] = videocore_mailbox::MailboxTag::SetPhysicalWidthHeight as u32;
//         mailbox_buf.tag_sequence[1] = 8;
//         mailbox_buf.tag_sequence[2] = 8;
//         mailbox_buf.tag_sequence[3] = 1024;
//         mailbox_buf.tag_sequence[4] = 768;
//
//         mailbox_buf.tag_sequence[5] = videocore_mailbox::MailboxTag::SetVirtualWidthHeight as u32;
//         mailbox_buf.tag_sequence[6] = 8;
//         mailbox_buf.tag_sequence[7] = 8;
//         mailbox_buf.tag_sequence[8] = 1024;
//         mailbox_buf.tag_sequence[9] = 768;
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
//         mailbox_buf.tag_sequence[32] = videocore_mailbox::MailboxTag::EndTag as u32;
//
//         let mut mailbox = videocore_mailbox::VideoCoreMailbox::new();
//         let channel = videocore_mailbox::MailboxChannel::PropertyARMToVC;
//         match mailbox.call(channel, &mailbox_buf) {
//             Ok(_) => (),
//             Err(_) => panic!("mailbox.call() error"),
//         }
//
//         let buffer = (mailbox_buf.tag_sequence[26] as usize & 0x3fff_ffff);
//         let len = mailbox_buf.tag_sequence[27] as usize;
//         let physical_resolution_horizontal = mailbox_buf.tag_sequence[3] as usize;
//         let physical_resolution_vertical = mailbox_buf.tag_sequence[4] as usize;
//         let virtual_resolution_horizontal = mailbox_buf.tag_sequence[8] as usize;
//         let virtual_resolution_vertical = mailbox_buf.tag_sequence[9] as usize;
//         let virtual_offset_horizontal = mailbox_buf.tag_sequence[13] as usize;
//         let virtual_offset_vertical = mailbox_buf.tag_sequence[14] as usize;
//         let bit_depth = mailbox_buf.tag_sequence[18];
//         let pixel_order = videocore_mailbox::PixelOrder::from(mailbox_buf.tag_sequence[22]);
//         let pitch = mailbox_buf.tag_sequence[31] as usize;
//         Framebuffer {
//             framebuffer_address: buffer,
//             len,
//             physical_resolution_horizontal,
//             physical_resolution_vertical,
//             virtual_resolution_horizontal,
//             virtual_resolution_vertical,
//             virtual_offset_horizontal,
//             virtual_offset_vertical,
//             bit_depth,
//             pixel_order,
//             pitch,
//         }
//     }
//
//     pub fn draw_homer(&self) {
//         let mut current_pixel_address = self.framebuffer_address +
//             (self.physical_resolution_vertical - homer_height) / 2 * self.pitch +
//             (self.physical_resolution_horizontal - homer_width) * 2;
//         let mut offset = 0usize;
//         for _ in 0..homer_height {
//             for _ in 0..homer_width {
//                 match self.pixel_order {
//                     videocore_mailbox::PixelOrder::RBG => {
//                         let mut pixel = current_pixel_address as *mut u8 as *mut RGBPixel;
//                         unsafe {
//                             (*pixel).red = (((homer_data[offset] - 33) << 2) | ((homer_data[offset + 1] -33) >> 4));
//                             (*pixel).green = ((((homer_data[offset + 1] - 33) & 0xf) << 4) | ((homer_data[offset + 2] - 33) >> 2));
//                             (*pixel).blue = ((((homer_data[offset + 2] - 33) & 0x3) << 6) | (homer_data[offset + 3] - 33));
//                         }
//                     }
//                     videocore_mailbox::PixelOrder::BGR => {
//                         let mut pixel = current_pixel_address as *mut u8 as *mut BGRPixel;
//                         unsafe {
//                             (*pixel).red = (((homer_data[offset] - 33) << 2) | ((homer_data[offset + 1] -33) >> 4));
//                             (*pixel).green = ((((homer_data[offset + 1] - 33) & 0xf) << 4) | ((homer_data[offset + 2] - 33) >> 2));
//                             (*pixel).blue = ((((homer_data[offset + 2] - 33) & 0x3) << 6) | (homer_data[offset + 3] - 33));
//                         }
//                     }
//                     _ => panic!("Invalid pixel order"),
//                 }
//                 offset += 4;
//                 current_pixel_address += 4;
//             }
//             current_pixel_address += self.pitch - (homer_width * 4);
//         }
//     }
// }
//
// #[repr(C)]
// pub struct RGBPixel {
//     pub red: u8,
//     pub green: u8,
//     pub blue: u8,
//     pub alpha: u8,
// }
//
// #[repr(C)]
// pub struct BGRPixel {
//     pub blue: u8,
//     pub green: u8,
//     pub red: u8,
//     pub alpha: u8,
// }
