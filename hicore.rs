#![no_std]
#![feature(lang_items)]

extern crate libc;
extern crate core;

use libc::puts;
use core::str::StrPrelude; // &str::as_prt()

#[start]
fn start(_argc: int, _argv: *const *const u8) -> int {
    unsafe {
        puts("Hi Rust! (uses core crate)\0".as_ptr() as *const i8);
    }
    return 0;
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "panic_fmt"]
extern fn panic_fmt(_args: &core::fmt::Arguments,
                    _file: &str,
                    _line: uint) -> ! {
    loop {}
}
