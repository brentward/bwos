#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(never_type)]
#![no_std]

pub mod atags;
pub mod common;
pub mod framebuffer;
pub mod gpio;
pub mod interrupt;
pub mod local_interrupt;
pub mod rng;
pub mod timer;
pub mod uart;
pub mod videocore_mailbox;
mod homer;
