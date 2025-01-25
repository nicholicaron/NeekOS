use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
use bump::BumpAllocator;
use linked_list::LinkedListAllocator;
use fixed_size_block::FixedSizeBlockAllocator;
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
pub mod fixed_size_block;

#[global_allocator]
// Use allocator from linked_list_allocator crate
//static ALLOCATOR: LockedHeap = LockedHeap::empty(); 
//
// Use bump allocator
//static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());
//
//Use LinkedListAllocator
//static ALLOCATOR: Locked<LinkedListAllocator> = Locked::new(LinkedListAllocator::new()); 
//
// Use fixed block allocator
static ALLOCATOR: Locked<FixedSizeBlockAllocator> = Locked::new(FixedSizeBlockAllocator::new());


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
///
/// Bitmask trick: 
/// Since align is a power of two, its binary representation has only a single bit set 
/// (e.g. 0b000100000). This means that align - 1 has all the lower bits set (e.g. 0b00011111).
/// By creating the bitwise NOT through the ! operator, we get a number that has all the bits 
/// set except for the bits lower than align (e.g. 0b…111111111100000).
/// By performing a bitwise AND on an address and !(align - 1), we align the address downwards.
/// This works by clearing all the bits that are lower than align.
/// Since we want to align upwards instead of downwards, we increase the addr by align - 1 before 
/// performing the bitwise AND. This way, already aligned addresses remain the same while 
/// non-aligned addresses are rounded to the next alignment boundary.
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) * !(align - 1)
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
