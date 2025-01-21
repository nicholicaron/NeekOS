use x86_64::VirtAddr;
use x86_64::structures::tss::TaskStateSegment;
use x86_64::structures::gdt::{GlobalDescriptorTable, Descriptor, SegmentSelector};
use lazy_static::lazy_static;

// Global Descriptor Table: Used for switching between kernel space and user space, and loading a
// Task State Segment Structure

pub fn init() {
    use x86_64::instructions::tables::load_tss;
    use x86_64::instructions::segmentation::{CS, Segment};
    GDT.0.load();
    unsafe {
        // Reload the code segment register since we changed our GDT
        CS::set_reg(GDT.1.code_selector);
        // Load the TSS (tell the CPU that it should use that TSS)
        load_tss(GDT.1.tss_selector);
    }
}

lazy_static! {
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (gdt, Selectors {code_selector, tss_selector})
    };
}

struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// Use lazy_static because Rust's const evaluator is not powerful enough to do this initialization
// at compile time
lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // Note: this double fault stack has no guard page to protect against stack overflow
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
            // stacks on x86_64 grow downwards, i.e. from high addresses to low addresses
            let stack_start = VirtAddr::from_ptr(&raw const STACK);
            let stack_end = stack_start + STACK_SIZE;
            stack_end
        };
        tss
    };
}
