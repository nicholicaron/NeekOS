/// This file defines a bump allocator (AKA stack allocator)
///
/// Benefits: Simple and can thus be quite fast
/// Drawback: Can only free all memory at once

use alloc::alloc::{GlobalAlloc, Layout};
use super::{align_up, Locked};
use core::ptr;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}

impl BumpAllocator {
    /// Creates a new empty bump allocator
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }

    /// Initializes the bump allocator with the given heap bounds.
    ///
    /// This method is unsafe because the caller must ensure that the given
    /// memory range is unused. Also, this method must be called only once
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        self.heap_start = heap_start;
        self.heap_end = heap_start + heap_size;
        self.next = heap_start;
    }
}

unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    // Layout describes the desired  size and alignment that the allocated memory should have. 
    // Returns a raw pointer to the first byte of the allocated memory block
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut bump = self.lock(); // get a mutable reference

        let alloc_start = align_up(bump.next, layout.align());
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };

        if alloc_end > bump.heap_end {
            ptr::null_mut() // out of memory. Note, this is not idiomatic for rust but matches the
                            // API of other common allocator for easy interchange
        } else {
            bump.next = alloc_end;
            bump.allocations += 1;
            alloc_start as *mut u8
        }
    }

    // Responsible for freeing a memory block, takes the pointer returned by alloc and the 
    // layout that was used for the allocation as arguments
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        let mut bump = self.lock(); // get a mutable reference

        bump.allocations  -= 1;
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}
