#![no_std] // do not load the standard library
#![no_main] // main is not the entry point
#![allow(unused_imports)]
#![allow(unused_variables)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod serial;
mod vga_buffer;

use core::panic::PanicInfo; // panic handler import

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
    serial_println!("[failed]\n");
    serial_println!("Error: {}", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
// Testing framework
#[cfg(test)]
fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    // exit qemu
    exit_qemu(QemuExitCode::Success);
}
// Qemu exit  code
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}
// Qemu exit function
pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}
//Implement testable trait
pub trait Testable {
    fn run(&self) -> ();
}
//Implementation
impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World! : #{}", 1);

    #[cfg(test)]
    test_main();
    loop {}
}

//testcases
#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
