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

    #[cfg(test)]
    test_main();

    loop{}
}

// This diverging function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

/// This function is called on panic (in test mode)
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    NeekOS::test_panic_handler(info)
}

