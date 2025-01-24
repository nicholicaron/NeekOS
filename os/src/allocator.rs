use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use bump::BumpAllocator;
use x86_64::{
    structures::paging::{
        mapper::MapToError, FrameAllocator, Mapper, Page, PageTableFlags, Size4KiB,
    },
    VirtAddr,
};
//use linked_list_allocator::LockedHeap;

pub const HEAP_START: usize = 0x_4444_4444_0000; // Create easily recognizable pointer to virtual
                                                 // memory range for our heap region
pub const HEAP_SIZE: usize = 100 * 1024; // 100KiB

pub mod bump;
pub mod linked_list;

#[global_allocator]
//static ALLOCATOR: LockedHeap = LockedHeap::empty(); // Use allocator from linked_list_allocator
static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

/// A wrapper around spin::Mutex to permit trait implementations
pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl <A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked{
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

/// Align the given address `addr` upwards to alignment `align`
///
/// Requires that `align` is a power of two
// Note: here is a faster implementation that utilizes bit manipulation (and requires align is a
// power of two)
// (addr + align - 1) * !(align -1)
fn align_up(addr: usize, align: usize) -> usize {
    let remainder = addr % align;
    if remainder == 0 {
        addr // addr already aligned
    } else {
        addr - remainder + align
    }
}

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
