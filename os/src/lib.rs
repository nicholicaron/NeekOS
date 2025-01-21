#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

use core::panic::PanicInfo;

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>()); // For functions, their type is
                                                               // their name, so we print this to
                                                               // SERIAL1
        self(); // invoke the test function (we require that self implements the Fn() trait
        serial_println!("[ok]"); // Indicate that the function did not panic
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

/// This diverging function handles panicked tests and outputs relevant info to serial
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/// Overwrite entry point for `cargo test`
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! { // linker looks for a function named '_start' by default
    init(); // Initialize Interrupt Descriptor Table
    test_main();
    loop {}
}

/// This diverging function is called on panic (in test mode).
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}



#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4); // 0xf4 is the iobase of the isa-debug-exit device
        port.write(exit_code as u32); // write the exit code to the iobase port. Cast to u32
                                      // because we specified the iosize of the isa-debug-exit
                                      // device to be 4 bytes
    }
}

/// Initialize Interrupt Descriptor Table
pub fn init() {
    gdt::init();
    interrupts::init_idt();
}
