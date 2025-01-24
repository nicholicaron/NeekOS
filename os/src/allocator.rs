use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};
use linked_list_allocator::LockedHeap;

pub const HEAP_START: usize = 0x_4444_4444_0000; // Create easily recognizable pointer to virtual
                                                 // memory range for our heap region
pub const HEAP_SIZE: usize = 100 * 1024; // 100KiB

#[global_allocator]
static ALLOCATOR: LockedHeap = LockedHeap::empty();

//pub struct Dummy;
//static ALLOCATOR2 Dummy;

//unsafe impl GlobalAlloc for Dummy {
    // Layout describes the desired  size and alignment that the allocated memory should have. 
    // Returns a raw pointer to the first byte of the allocated memory block
//    unsafe fn alloc(&self, _layout: Layout) -> *mut u8{
//        null_mut()
//    }

    // Responsible for freeing a memory block, takes the pointer returned by alloc and the 
    // layout that was used for the allocation as arguments
//    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
//        panic!("Dealloc should never be called")
//    }
//}

/// Initialize Heap
/// Takes mutable references to a (Virtual Memory -> Physical Memory) Mapper and a 
/// FrameAllocator instance (both limited to 4KiB). Returns either Unit type or MapToError
///
/// First, we create the page range by converting HEAP_START to a VirtAddr, then calculate 
/// the heap end address (-1 so it is inclusive). Then convert the addresses into Pages then  
/// create page range.
/// Next, we map all pages of the page range. For each page we allocate a physical frame that
/// the page should be mapped to. Then we set the required PRESENT and WRITABLE flags for the 
/// page, allowing both read and write accesses. 
pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>, 
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
    ) -> Result<(), MapToError<Size4KiB>> {
        let page_range = {
            let heap_start = VirtAddr::new(HEAP_START as u64);
            let heap_end = heap_start + HEAP_SIZE - 1u64;
            let heap_start_page = Page::containing_address(heap_start);
            let heap_end_page = Page::containing_address(heap_end);
            Page::range_inclusive(heap_start_page, heap_end_page)
        };

        for page in page_range {
            let frame = frame_allocator
                .allocate_frame()
                .ok_or(MapToError::FrameAllocationFailed)?;
            let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
            unsafe{
                mapper.map_to(page, frame, flags, frame_allocator)?.flush() // Returns MapperFlush
                                                                            // instance, which we
                                                                            // use to update
                                                                            // the translation
                                                                            // lookaside buffer
            };
        }

        unsafe{
            ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE); 
        }

        Ok(())
}
