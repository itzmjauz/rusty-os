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

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // entry point of system
    vga_buffer::print_something();

    loop {}
}
