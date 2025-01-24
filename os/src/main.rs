#![no_std] // Unlink the standard library
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(NeekOS::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use NeekOS::println;
use bootloader::{BootInfo, entry_point};
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};
use NeekOS::allocator;

entry_point!(kernel_main);

// Overwrite the entry point chain
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use NeekOS::memory;
    use x86_64::{structures::paging::Page, VirtAddr};
    use NeekOS::memory::BootInfoFrameAllocator;
    
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

    // Demonstrates how to translate virtual addresses to physical addresses
    //let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // initialize a mapper
    //let mapper = unsafe {memory::init(phys_mem_offset)};

    //let addresses = [
        // the identiry-mapped vga buffer page
        //0xb8000,
        // some code page
        //0x201008,
        // some stack page
        //0x0100_0020_1a10,
        // virtual address mapped to physical address 0
        //boot_info.physical_memory_offset,
    //];

    //for &address in &addresses {
        //let virt = VirtAddr::new(address);
        //let phys = mapper.translate_addr(virt);
        //println!("{:?} -> {:?}", virt, phys);
    //}

    // Create a mapping to an unused page
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe {memory::init(phys_mem_offset)};
    let mut frame_allocator = unsafe{
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // Create page using above mapping to write to unused (buffer) memory
    //let page = Page::containing_address(VirtAddr::new(0));
    //memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);
    // write the string `New!` to the screen through the new mapping
    //let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    //unsafe {page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    // Initialize heap and allocate memory on it
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");

    // Allocate a number on the heap
    let heap_value = Box::new(42);
    println!("heap_value at {:p}", heap_value);

    // Create a dynamically size vector
    let mut vec = Vec::new();
    for i in 0..500 {
        vec.push(i);
    }
    println!("vec at {:p}", vec.as_slice());

    // Create a reference counted vector. Will be freed when count reaches 0
    let reference_counted = Rc::new(vec![1,2,3]);
    let cloned_reference = reference_counted.clone();
    println!("Current reference count is {}", Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("Reference count is {} not", Rc::strong_count(&cloned_reference));

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

