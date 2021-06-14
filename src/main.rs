#![no_std] // do not load the standard library
// developer mode
#![allow(unused_imports)]
#![allow(unused_variables)]
// imports
use core::panic::PanicInfo; // panic handler import

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

fn main() {
    let a = 1;
}
