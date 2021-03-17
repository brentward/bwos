#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(never_type)]
#![no_std]

pub mod atags;
pub mod common;
pub mod gpio;
pub mod hdmi_framebuffer;
pub mod interrupt;
pub mod local_interrupt;
pub mod rng;
pub mod timer;
pub mod uart;
pub mod videocore_mailbox;
pub mod font;
mod homer;
