#![no_std] // Unlink the standard library
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(NeekOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use NeekOS::println;

// Overwrite the entry point chain
#[no_mangle] // Tell Rustc not to mangle the name of our start fn
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function 
    // named '_start' by default
    
    println!("Hello World{}", "!");

    // Initialize Interrupt Descriptor Table
    NeekOS::init();

    // Invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3();


    // Trigger a page fault
    // unsafe {
    //    *(0xdeadbeef as *mut u8) = 42;
    //}
    
    // Provoke a kernel stack overflow
    //fn stack_overflow() {
    //    stack_overflow() // for each recursion, the return address is pushed
    //}
    //stack_overflow();

    #[cfg(test)]
    test_main();

    println!("It did not crash!");

    NeekOS::hlt_loop(); // wait for interrupts, sleep in the meantime
}

// This diverging function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    NeekOS::hlt_loop();
}

/// This function is called on panic (in test mode)
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    NeekOS::test_panic_handler(info)
}

