#![feature(alloc_error_handler)]
#![feature(panic_info_message)]
#![feature(const_fn)]
#![feature(decl_macro)]
#![feature(asm)]
#![feature(global_asm)]
#![feature(auto_traits)]
#![feature(negative_impls)]
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]


#[cfg(not(test))]
mod init;

extern crate alloc;

pub mod allocator;
pub mod console;
pub mod framebuffer;
pub mod mutex;
pub mod param;
pub mod shell;

use console::kprintln;
use alloc::string;

use allocator::Allocator;

#[cfg_attr(not(test), global_allocator)]
pub static ALLOCATOR: Allocator = Allocator::uninitialized();

fn kmain() -> ! {
    unsafe {
        ALLOCATOR.initialize();
    }

    framebuffer::FRAMEBUFFER.lock().draw_homer();
    kprintln!("Welcome to Brentward OS");
    let mut allocated_string = string::String::from("This is a string on the heap");
    kprintln!("{}", allocated_string);
    allocated_string.push_str(": this has been pushed into it");
    kprintln!("{}", allocated_string);
    // for line in 0..1000 {
    //     kprintln!("line: {}", line);
    // }
    shell::shell("> ");
}
