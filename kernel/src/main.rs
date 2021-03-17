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

pub mod console;
pub mod framebuffer;
pub mod mutex;
pub mod shell;

use console::{kprintln, kprint};
use pi::{timer, videocore_mailbox};
use core::time::Duration;


fn kmain() -> ! {
    timer::spin_sleep(Duration::from_millis(100));
    framebuffer::FRAMEBUFFER.lock().draw_homer();
    kprintln!("\nWelcome to Brentward OS");
    // FRAMEBUFFER.lock().clear();
    // FRAMEBUFFER.lock().print("Hello world\n");
    // FRAMEBUFFER.lock().print("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyx1234567890ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyx1234567890\n");
    // let random_generator = pi::rng::Rng::new();
    // for hundreds in 48u8..58 {
    //     for tens in 48u8..58 {
    //         for ones in 48u8..58 {
    //             let byte_array = [hundreds, tens, ones];
    //             let number = from_utf8(&byte_array).expect("Invalid UTF-8");
    //             FRAMEBUFFER.lock().print("line: ");
    //             FRAMEBUFFER.lock().print(number);
    //             FRAMEBUFFER.lock().print(" -- ");
    //
    //             let mut str_array = [0u8, 0, 0, 10];
    //             str_array[0] = random_generator.rand(33, 127) as u8;
    //             str_array[1] = random_generator.rand(33, 127) as u8;
    //             str_array[2] = random_generator.rand(33, 127) as u8;
    //             let message = from_utf8(&str_array).expect("Invalid UTF-8 string");
    //             FRAMEBUFFER.lock().print(message);
    //         }
    //     }
    // }
    shell::shell("> ");
}
