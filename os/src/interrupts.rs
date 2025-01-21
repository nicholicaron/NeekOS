use x86_64::structures::idt::{InterruptDescriptorTable,InterruptStackFrame};
use crate::println;
use lazy_static::lazy_static;
use crate::gdt;

// This file defines how the OS should handle various interrupts

// IDT needs static lifetime because the CPU will access this table on every interrupt until we
// load a different IDT. Any shorter lifetime could lead to use-after-free bugs
// Mutable so we can set handler functions during init
// Lazy Static so init is performed at time of first reference instead of compile time
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        unsafe{
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}

// Use x86-interrupt calling convention -- differs from traditional function calling convention
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}   

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame, _error_code: u64) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[test_case]
fn test_breakpoint_exception() {
    // Invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
