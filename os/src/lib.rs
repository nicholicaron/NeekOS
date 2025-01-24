#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

extern crate alloc;

use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

pub mod serial;
pub mod vga_buffer;
pub mod interrupts;
pub mod gdt;
pub mod memory;
pub mod allocator;

/// Overwrite entry point for `cargo test`
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init(); // Initialize Interrupt Descriptor Table
    test_main();
    hlt_loop();
}

/// This diverging function is called on panic (in test mode).
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

/// This diverging function handles panicked tests and outputs relevant info to serial
pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    hlt_loop();
}

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

pub fn init() {
    // Initialize Global Descriptor Table
    gdt::init();
    // Initialize Interrupt Descriptor Table
    interrupts::init_idt();
    // Initialize Programmable Interrupt Controller (PIC8259)
    unsafe {interrupts::PICS.lock().initialize()};
    // Tell the CPU to listen to the interrupt controller
    x86_64::instructions::interrupts::enable();
}

// Allow CPU to enter sleep state (halt) until the next interrupt arrives
// A more energy efficient way to wait for user input
pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
