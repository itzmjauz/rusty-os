#![no_std] // do not load the standard library
#![no_main] // main is not the entry point

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo; // panic handler import

// Function called at panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
//    println!("Hello World! : #{}", 1);
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        for byte in framebuffer.buffer_mut() {
            *byte = 0x90;
        }
    }
    
    loop {}
}
