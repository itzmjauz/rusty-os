#![no_std] // do not load the standard library
#![no_main] // main is not the entry point

#![allow(unused_imports)]
#![allow(unused_variables)]

mod vga_buffer;

use core::panic::PanicInfo; // panic handler import

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello  World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // entry point of system
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
