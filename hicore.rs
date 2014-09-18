#![no_std]
#![feature(lang_items)]

extern crate libc;
extern crate core;

use libc::puts;
use core::intrinsics::transmute;

#[start]
fn start(_argc: int, _argv: *const *const u8) -> int {
	let s = "Hi Rust! (uses core crate)\0"; // &str
    unsafe {
    	let (s,_): (*const i8, uint) = transmute(s); // see core::raw::Slice
    	puts(s);
    }
	return 0;
}

#[lang = "stack_exhausted"] extern fn stack_exhausted() {}
#[lang = "eh_personality"] extern fn eh_personality() {}

#[lang = "begin_unwind"]
extern fn begin_unwind(_args: &core::fmt::Arguments,
                        _file: &str,
                        _line: uint) -> ! {
    loop {}
}
