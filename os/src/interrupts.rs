use x86_64::structures::idt::{InterruptDescriptorTable,InterruptStackFrame};
use crate::{println,print};
use lazy_static::lazy_static;
use crate::{gdt,hlt_loop};
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::PageFaultErrorCode;

// This file defines how the OS should handle various interrupts
// Note: Hardware Programmable Interrupt Controller (PIC) based in Intel 8259
//                      ____________                          ____________
// Real Time Clock --> |            |   Timer -------------> |            |
// ACPI -------------> |            |   Keyboard-----------> |            |      _____
// Available --------> | Secondary  |----------------------> | Primary    |     |     |
// Available --------> | Interrupt  |   Serial Port 2 -----> | Interrupt  |---> | CPU |
// Mouse ------------> | Controller |   Serial Port 1 -----> | Controller |     |_____|
// Co-Processor -----> |  (PIC_2)   |   Parallel Port 2/3 -> |  (PIC_1)   |
// Primary ATA ------> |            |   Floppy disk -------> |            |
// Secondary ATA ----> |____________|   Parallel Port 1----> |____________|



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
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
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


pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(
    unsafe {ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)});

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard, // Note: handles PS/2 keyboards
}

impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }

    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    // Send End Of Interrupt (EOI) signal
    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;
    use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
    use spin::Mutex;

    lazy_static!{
        // By default, PS/2 keyboards emulate scancode set 1 (based on the IBM XT keyboard)
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(ScancodeSet1::new(),
                layouts::Us104Key, HandleControl::Ignore) // Do not convert Ctrl+[a-z] to unicode
                                                          // values, handle like normal keys
            );
    }

    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60); // Data port of the PS/2 controller

    let scancode: u8 = unsafe {port.read()};

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) { // add_byte translates the scancode
                                                               // into an Option<KeyEvent>
        if let Some(key) = keyboard.process_keyevent(key_event) { // Translate Key event to a
                                                                  // character if possible
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read()); // Cr2 register automatically set by CPU on
                                                     // page fault, contains accessed virtual
                                                     // address that caused the page fault
    println!("Error Code: {:?}", error_code); // Provides more info about the type of memory access
                                              // that caused the page fault (e.g. read or write)
    println!("{:#?}", stack_frame);
    hlt_loop();
}
