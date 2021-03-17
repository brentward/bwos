use pi::videocore_mailbox;
use pi::hdmi_framebuffer::{HDMIFrameBuffer, HDMIConfig, PixelColor};
use shim::io;

use crate::mutex::Mutex;

pub const HORIZONTAL_RESOLUTION: u32 = 960;
pub const VERTICAL_RESOLUTION: u32 = 540;
pub const COLOR_DEPTH: u32 = 32;
pub const VIRTUAL_VERTICAL_MULTIPLIER: u32 = 3;
pub const BYTES_PER_CHAR: usize = 16;

#[repr(C)]
pub struct FrameBuffer {
    inner: Option<HDMIFrameBuffer>,
}

impl FrameBuffer {
    const fn new() -> FrameBuffer {
        FrameBuffer { inner: None }
    }
    fn initialize(&mut self, config: HDMIConfig) {
        match self.inner {
            None => self.inner = Some(HDMIFrameBuffer::new(config)),
            _ => (),
        }
    }

    fn inner(&mut self) -> &mut HDMIFrameBuffer {
        match self.inner {
            Some(ref mut framebuffer) => framebuffer,
            _ => {
                self.initialize(HDMIConfig::new(HORIZONTAL_RESOLUTION, VERTICAL_RESOLUTION, COLOR_DEPTH, VIRTUAL_VERTICAL_MULTIPLIER));
                self.inner()
            }
        }
    }

    pub fn draw_homer(&mut self) {
        self.inner().draw_homer()
    }

    pub fn set_foreground_color(&mut self, color: &PixelColor) {
        self.inner().foreground_color.red = color.red;
        self.inner().foreground_color.green = color.green;
        self.inner().foreground_color.blue = color.blue;
        self.inner().foreground_color.alpha = color.alpha;
    }

    pub fn set_background_color(&mut self, color: &PixelColor) {
        self.inner().background_color.red = color.red;
        self.inner().background_color.green = color.green;
        self.inner().background_color.blue = color.blue;
        self.inner().background_color.alpha = color.alpha;
    }
    pub fn clear(&mut self) {
        self.inner().clear();
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.inner().write_byte(byte)
    }

    pub fn draw_cursor(&mut self) {
        self.inner().draw_cursor()
    }
}

impl io::Write for FrameBuffer {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.inner().write(buf)
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

impl core::fmt::Write for FrameBuffer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        self.inner().write_str(s)
    }
}

pub static FRAMEBUFFER: Mutex<FrameBuffer> = Mutex::new(FrameBuffer::new());

