use crate::common::IO_BASE;

use volatile::{WriteVolatile, prelude::*};
use volatile::{ReadVolatile, Volatile, Reserved};
use aarch64::*;

/// The base address for the VideoCore Mailbox registers.
const VIDEOCORE_MBOX_REG_BASE: usize = IO_BASE + 0xb880;

#[repr(C)]
#[allow(non_snake_case)]
struct Registers {
    READ: ReadVolatile<u32>,
    __r0: Reserved<u32>,
    __r1: Reserved<u32>,
    __r2: Reserved<u32>,
    READ_PEEK: ReadVolatile<u32>,
    READ_SENDER: ReadVolatile<u32>,
    READ_STATUS: ReadVolatile<u32>,
    READ_CONFIG: ReadVolatile<u32>,
    WRITE: WriteVolatile<u32>,
    __r3: Reserved<u32>,
    __r4: Reserved<u32>,
    __r5: Reserved<u32>,
    WRITE_PEEK: ReadVolatile<u32>,
    WRITE_SENDER: ReadVolatile<u32>,
    WRITE_STATUS: ReadVolatile<u32>,
    WRITE_CONFIG: ReadVolatile<u32>
}

/// Enum representing Mailbox Channels.
#[repr(u8)]
#[derive(PartialEq)]
pub enum MailboxChannel {
    PowerManagement = 0,
    Framebuffer = 1,
    VirtualUART = 2,
    VCHIQ = 3,
    LEDs = 4,
    Buttons = 5,
    TouchScreen = 6,
    Count = 7,
    PropertyARMToVC = 8,
    PropertyVCToARM = 9,
    Reserved10 = 10,
    Reserved11 = 11,
    Reserved12 = 12,
    Reserved13 = 13,
    Reserved14 = 14,
    Reserved15 = 15,
}

impl From<u32> for MailboxChannel {
    fn from(data: u32) -> MailboxChannel {
        use MailboxChannel::*;
        match data & 0xf {
            0 => PowerManagement,
            1 => Framebuffer,
            2 => VirtualUART,
            3 => VCHIQ,
            4 => LEDs,
            5 => Buttons,
            6 => TouchScreen,
            7 => Count,
            8 => PropertyARMToVC,
            9 => PropertyVCToARM,
            10 => Reserved10,
            11 => Reserved11,
            12 => Reserved12,
            13 => Reserved13,
            14 => Reserved14,
            15 => Reserved15,
            _ => unreachable!("Mailbox channel should not be over 15 given the 0xF mask"),
        }
    }
}
/// Enum representing Mailbox request/response codes
#[repr(u32)]
#[derive(PartialEq)]
pub enum MailboxBufStatus {
    ProcessRequest = 0x0000_0000,
    RequestSuccessful = 0x8000_0000,
    ErrorParsingRequest = 0x8000_0001,
}

impl From<u32> for MailboxBufStatus {
    fn from(status: u32) -> MailboxBufStatus {
        use MailboxBufStatus::*;
        match status {
            0x0000_0000 => ProcessRequest,
            0x8000_0000 => RequestSuccessful,
            0x8000_0001 => ErrorParsingRequest,
            _ => panic!("Invalid MailboxBufStatus"),
        }
    }
}

/// Enum representing Mailbox Tags
#[repr(u32)]
pub enum MailboxTag {
    EndTag = 0x0000_0000,
    GetFirmwareRevision = 0x0000_0001,
    GetBoardModel = 0x0001_0001,
    GetBoardRevision = 0x0001_0002,
    GetBoardMacAddress = 0x0001_0003,
    GetBoardSerial = 0x0001_0004,
    GetARMMemory = 0x0001_0005,
    GetVCMemory = 0x0001_0006,
    GetClocks = 0x0001_0007,
    GetCommandLine = 0x0005_0001,
    GetDMAChannels = 0x0006_0001,
    GetPowerState = 0x0002_0001,
    GetTiming = 0x0002_0002,
    SetPowerState = 0x0002_8001,
    GetClockState = 0x0003_0001,
    SetClockState = 0x0003_8001,
    GetClockRate = 0x0003_0002,
    SetClockRate = 0x0003_8002,
    GetMaxClockRate = 0x0003_0004,
    GetMinClockRate = 0x0003_0007,
    GetTurbo = 0x0003_0009,
    SetTurbo = 0x0003_8009,
    GetVoltage = 0x0003_0003,
    SetVoltage = 0x0003_8003,
    GetMaxVoltage = 0x0003_0005,
    GetMinVoltage = 0x0003_0008,
    GetTemperature = 0x0003_0006,
    GetMaxTemperature = 0x0003_000a,
    AllocateMemory = 0x0003_000c,
    LockMemory = 0x0003_000d,
    UnlockMemory = 0x0003_000e,
    ReleaseMemory = 0x0003_000f,
    ExecuteCode = 0x0003_0010,
    GetDispmanxResourceMemHandle = 0x0003_0014,
    AllocateBuffer = 0x0004_0001,
    ReleaseBuffer = 0x0004_8001,
    BlankScreen = 0x0004_0002,
    GetPhysicalWidthHeight = 0x0004_0003,
    TestPhysicalWidthHeight = 0x0004_4003,
    SetPhysicalWidthHeight = 0x0004_8003,
    GetVirtualWidthHeight = 0x0004_0004,
    TestVirtualWidthHeight = 0x0004_4004,
    SetVirtualWidthHeight = 0x0004_8004,
    GetDepth = 0x0004_0005,
    TestDepth = 0x0004_4005,
    SetDepth = 0x0004_8005,
    GetPixelOrder = 0x0004_0006,
    TestPixelOrder = 0x0004_4006,
    SetPixelOrder = 0x0004_8006,
    GetAlphaMode = 0x0004_0007,
    TestAlphaMode = 0x0004_4007,
    SetAlphaMode = 0x0004_8007,
    GetPitch = 0x0004_0008,
    GetVirtualOffset = 0x0004_0009,
    TestVirtualOffset = 0x0004_4009,
    SetVirtualOffset = 0x0004_8009,
    GetOverscan = 0x0004_000a,
    TestOverscan = 0x0004_400a,
    SetOverscan = 0x0004_800a,
    GetPalette = 0x0004_000b,
    TestPalette = 0x0004_400b,
    SetPalette = 0x0004_800b,
    SetCursorInfo = 0x0000_8010,
    SetCursorState = 0x0000_8011,
}

/// Enum representing Mailbox Status
#[repr(u32)]
pub enum MailboxStatus {
    Full = 0x8000_0000,
    Empty = 0x4000_0000,
}

/// Enum representing Pixel Order
#[repr(u32)]
#[derive(PartialEq, Debug)]
pub enum PixelOrder {
    BGR = 0x0,
    RBG = 0x1,
}

impl From<u32> for PixelOrder {
    fn from(order: u32) -> PixelOrder {
        use PixelOrder::*;
        match order {
            0x0 => BGR,
            0x1 => RBG,
            _ => panic!("Invalid PixelOrder"),
        }
    }
}

pub struct VideoCoreMailbox {
    registers: &'static mut Registers,
}

impl VideoCoreMailbox {
    pub fn new() -> VideoCoreMailbox {
        VideoCoreMailbox {
            registers: unsafe { &mut *(VIDEOCORE_MBOX_REG_BASE as *mut Registers) }
        }
    }

    pub fn call(&mut self, channel: MailboxChannel, buf: &MailboxBuf) -> Result<(), ()> {
        while self.registers.WRITE_STATUS.has_mask(MailboxStatus::Full as u32) {
            nop();
        }

        let message = ((buf as *const MailboxBuf) as u32 & !0xf) | channel as u32;

        self.registers.WRITE.write(message);
        
        loop {
            while self.registers.READ_STATUS.has_mask(MailboxStatus::Empty as u32) {
                nop();
            }
            let data = self.registers.READ.read();
            if message == data {
                let response_status = buf.get_status();
                return match response_status {
                    MailboxBufStatus::RequestSuccessful => Ok(()),
                    _ => Err(()),
                }
            }
        }
    }

    pub fn write_mailbox(&mut self, channel: MailboxChannel, data: u32) {
        while self.registers.WRITE_STATUS.has_mask(MailboxStatus::Full as u32) {
            nop();
        }

        self.registers.WRITE.write(data << 4 | channel as u32);
    }

    pub fn read_mailbox(&self, channel: MailboxChannel) -> u32 {
        loop {
            while self.registers.READ_STATUS.has_mask(MailboxStatus::Empty as u32) {
                nop();
            }
            let mut data = self.registers.READ.read();
            let read_channel = MailboxChannel::from(data);
            data >>= 4;
            if read_channel == channel {
                return data
            }
        }
    }
}

#[repr(C, align(16))]
pub struct MailboxBuf {
    pub size: u32,
    status: u32,
    pub tag_sequence: [u32; 36],
}

impl MailboxBuf {
    pub fn new() -> Self {
        MailboxBuf {
            size: 0,
            status: 0,
            tag_sequence: [0; 36],
        }
    }

    pub fn get_status(&self) -> MailboxBufStatus {
        MailboxBufStatus::from(self.status)
    }

    pub fn prepare_buf(&mut self, tags_len: usize) {
        self.size = (tags_len as u32 + 3) * 4;
        self.status = MailboxBufStatus::ProcessRequest as u32;
        self.tag_sequence[tags_len] = MailboxTag::EndTag as u32;

    }
}