#![no_std] // unlink the standard library
#![no_main] // Main doesn't make sense w/o an underlying runtime that calls it
use core::panic::PanicInfo;

// This diverging function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle] // Tell Rustc not to mangle the name of our start fn
pub extern "C" fn _start() -> ! {
    loop {}
}
