#![no_std] // do not load the standard library
#![no_main] // main is not the entry point
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(custom_test_frameworks)]
#![test_runner(rusty_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo; // panic handler import
use rusty_os::println;

// Function called at panic
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rusty_os::test_panic_handler(info)
}

use bootloader::{entry_point, BootInfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    println!("Hello World! : #{}", 1);
    
    #[cfg(test)]
    test_main();
    loop {}
}
